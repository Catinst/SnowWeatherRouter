package com.miui.weather3;

import android.app.Application;
import android.util.Log;

/** Standard DEX application bootstrap replacing the OS3 donor Application shell. */
public final class WeatherApplication extends Application {
    private static final String TAG = "SnowDexHost";

    @Override
    public void onCreate() {
        super.onCreate();
        WeatherHostServices.initialize(this);
        Log.i(TAG, "WeatherApplication onCreate complete");
    }
}
