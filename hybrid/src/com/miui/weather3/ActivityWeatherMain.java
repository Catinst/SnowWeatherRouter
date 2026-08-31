package com.miui.weather3;

import android.Manifest;
import android.app.AlertDialog;
import android.app.NativeActivity;
import android.content.ActivityNotFoundException;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.util.Log;

public final class ActivityWeatherMain extends NativeActivity {
    private static final String TAG = "SnowHybridHost";
    private static final String PREFS = "snow_weather_hybrid";
    private static final String KEY_APP_RUN = "app_run";
    private static final int REQUEST_CTA = 1007;
    private static final int REQUEST_LOCATION = 10000;

    static {
        System.loadLibrary("SnowWeatherRouter");
    }

    private static native void nativeSetUserAgreement(boolean agreed);
    private static native void nativeSetLocationPermission(boolean granted);

    private boolean isAgreed() {
        return getSharedPreferences(PREFS, MODE_PRIVATE).getBoolean(KEY_APP_RUN, false);
    }

    private boolean hasForegroundLocation() {
        return checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                || checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED;
    }

    @Override
    protected void onCreate(Bundle state) {
        final boolean agreed = isAgreed();
        nativeSetUserAgreement(agreed);
        nativeSetLocationPermission(hasForegroundLocation());
        super.onCreate(state);
        if (!agreed && state == null) {
            // Match the original order: establish Flutter and its first route,
            // then present the system CTA above the weather activity.
            getWindow().getDecorView().postDelayed(this::launchCta, 500L);
        }
    }

    private void launchCta() {
        Intent intent = new Intent("miui.intent.action.CTA_DECLARE");
        intent.setFlags(0x24010000);
        intent.putExtra("use_network", true);
        intent.putExtra("mandatory_permission", true);
        intent.putExtra("optional_perm_show", false);
        intent.putExtra("user_agreement", "http://www.miui.com/res/doc/eula.html?region=CN&lang=zh_CN");
        intent.putExtra("privacy_policy", "https://privacy.mi.com/weather/zh_CN/");
        intent.putExtra("main_purpose", "展示天气信息");
        intent.putExtra("agree_desc", "请您同意应用联网及以上必要权限");
        intent.putExtra("optional_perm_desc", new CharSequence[]{"用于展示您当前定位的天气信息"});
        try {
            Log.i(TAG, "Launching system CTA requestCode=1007");
            startActivityForResult(intent, REQUEST_CTA);
        } catch (ActivityNotFoundException | SecurityException error) {
            Log.w(TAG, "System CTA unavailable; using explicit local consent dialog", error);
            new AlertDialog.Builder(this)
                    .setTitle("天气服务")
                    .setMessage("是否同意应用联网，并使用定位权限展示当地天气？")
                    .setNegativeButton("不同意", (dialog, which) -> finishAndRemoveTask())
                    .setPositiveButton("同意", (dialog, which) -> handleCtaResult(1))
                    .setCancelable(false)
                    .show();
        }
    }

    private void handleCtaResult(int resultCode) {
        if (resultCode != 1) {
            Log.i(TAG, "CTA declined resultCode=" + resultCode);
            nativeSetUserAgreement(false);
            finishAndRemoveTask();
            return;
        }
        Log.i(TAG, "CTA accepted resultCode=1");
        SharedPreferences.Editor edit = getSharedPreferences(PREFS, MODE_PRIVATE).edit();
        edit.putBoolean(KEY_APP_RUN, true).apply();
        nativeSetUserAgreement(true);
        if (!hasForegroundLocation()) {
            requestPermissions(
                    new String[]{Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION},
                    REQUEST_LOCATION);
        } else {
            nativeSetLocationPermission(true);
            recreate();
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_CTA) {
            handleCtaResult(resultCode);
        }
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        if (requestCode == REQUEST_LOCATION) {
            boolean granted = hasForegroundLocation();
            Log.i(TAG, "Location permission result granted=" + granted);
            nativeSetLocationPermission(granted);
            recreate();
        }
    }
}
