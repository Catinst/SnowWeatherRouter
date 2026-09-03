package com.miui.weather3;

import android.content.Context;
import android.util.Log;

/** Entry point for standard Android host services used by the test profile. */
final class WeatherHostServices {
    private static final String TAG = "SnowDexHost";
    private static boolean initialized;

    private WeatherHostServices() {
    }

    static synchronized void initialize(Context context) {
        if (initialized) {
            return;
        }
        WeatherDatabase.initialize(context);
        initialized = true;
        Log.i(TAG, "standard Android weather host services initialized");
    }
}
