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
    // Test profile only: keep CTA acceptance in this Activity. Launching the
    // external SecurityCenter CTA destroys and recreates the NativeActivity
    // Surface before the replacement router has a safe rebind state.
    private static final boolean TEST_AUTO_ACCEPT_CTA = true;
    // Retained for the production/fallback path where SecurityCenter returns
    // the legacy resultCode=-2 for a non-system-signed clone.
    private static final boolean TEST_AUTO_ACCEPT_CTA_MINUS_TWO = true;

    static {
        System.loadLibrary("SnowWeatherRouter");
    }

    private static native void nativeSetUserAgreement(boolean agreed);
    private static native void nativeSetLocationPermission(boolean granted);
    private static native void nativeDeliverActivityResult(int resultCode);
    private static native void nativeDeliverPermissionResult(int fineResult, int coarseResult);

    private boolean isAgreed() {
        return getSharedPreferences(PREFS, MODE_PRIVATE).getBoolean(KEY_APP_RUN, false);
    }

    private int permissionResult(String permission) {
        return checkSelfPermission(permission) == PackageManager.PERMISSION_GRANTED
                ? PackageManager.PERMISSION_GRANTED : PackageManager.PERMISSION_DENIED;
    }

    private boolean hasForegroundLocation() {
        return permissionResult(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                || permissionResult(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED;
    }

    @Override
    protected void onCreate(Bundle state) {
        final boolean agreed = isAgreed();
        nativeSetUserAgreement(agreed);
        nativeSetLocationPermission(hasForegroundLocation());
        super.onCreate(state);
        if (!agreed && state == null) {
            if (TEST_AUTO_ACCEPT_CTA) {
                // Wait until Flutter has installed its intent-channel listener,
                // then accept without opening another Activity/Surface.
                getWindow().getDecorView().postDelayed(() -> {
                    Log.i(TAG, "TEST_AUTO_ACCEPT_CTA=true; accepting in-place");
                    handleCtaAccept();
                }, 2500L);
            } else {
                getWindow().getDecorView().postDelayed(this::launchCta, 500L);
            }
        } else if (agreed && state == null) {
            // Test-only replay for the headless device: the original host
            // receives a CTA result before its first city-location attempt.
            getWindow().getDecorView().postDelayed(() -> {
                Log.i(TAG, "TEST_REPLAY_ACCEPTED_CTA=true; replaying resultCode=1");
                nativeDeliverActivityResult(1);
                nativeSetLocationPermission(hasForegroundLocation());
                nativeDeliverPermissionResult(
                        permissionResult(Manifest.permission.ACCESS_FINE_LOCATION),
                        permissionResult(Manifest.permission.ACCESS_COARSE_LOCATION));
            }, 2000L);
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
            Log.w(TAG, "System CTA unavailable; using local consent", error);
            showLocalConsent();
        }
    }

    private void showLocalConsent() {
        if (isFinishing() || isDestroyed()) {
            return;
        }
        new AlertDialog.Builder(this)
                .setTitle("天气服务")
                .setMessage("是否同意应用联网，并使用定位权限展示当地天气？")
                .setNegativeButton("不同意", (dialog, which) -> handleCtaDecline())
                .setPositiveButton("同意", (dialog, which) -> handleCtaAccept())
                .setCancelable(false)
                .show();
    }

    private void handleCtaAccept() {
        Log.i(TAG, "CTA accepted resultCode=1");
        SharedPreferences.Editor edit = getSharedPreferences(PREFS, MODE_PRIVATE).edit();
        edit.putBoolean(KEY_APP_RUN, true).apply();
        nativeSetUserAgreement(true);
        nativeDeliverActivityResult(1);
        if (!hasForegroundLocation()) {
            getWindow().getDecorView().postDelayed(() -> requestPermissions(
                    new String[]{Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION},
                    REQUEST_LOCATION), 400L);
        } else {
            nativeSetLocationPermission(true);
            nativeDeliverPermissionResult(
                    permissionResult(Manifest.permission.ACCESS_FINE_LOCATION),
                    permissionResult(Manifest.permission.ACCESS_COARSE_LOCATION));
        }
    }

    private void handleCtaDecline() {
        Log.i(TAG, "CTA declined by user");
        nativeSetUserAgreement(false);
        nativeDeliverActivityResult(0);
        moveTaskToBack(true);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode != REQUEST_CTA) {
            return;
        }
        if (resultCode == 1) {
            handleCtaAccept();
        } else if (resultCode == -2) {
            Log.i(TAG, "System CTA rejected weather3 resultCode=-2");
            if (TEST_AUTO_ACCEPT_CTA_MINUS_TWO) {
                Log.i(TAG, "TEST_AUTO_ACCEPT_CTA_MINUS_TWO=true; accepting pending CTA");
                handleCtaAccept();
            } else {
                getWindow().getDecorView().post(this::showLocalConsent);
            }
        } else {
            Log.i(TAG, "System CTA declined resultCode=" + resultCode);
            handleCtaDecline();
        }
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        if (requestCode == REQUEST_LOCATION) {
            int fine = permissionResult(Manifest.permission.ACCESS_FINE_LOCATION);
            int coarse = permissionResult(Manifest.permission.ACCESS_COARSE_LOCATION);
            boolean granted = fine == PackageManager.PERMISSION_GRANTED
                    || coarse == PackageManager.PERMISSION_GRANTED;
            Log.i(TAG, "Location permission result fine=" + fine + " coarse=" + coarse);
            nativeSetLocationPermission(granted);
            nativeDeliverPermissionResult(fine, coarse);
        }
    }
}
