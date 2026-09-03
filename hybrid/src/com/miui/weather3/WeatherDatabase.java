package com.miui.weather3;

import android.content.ContentValues;
import android.content.Context;
import android.database.Cursor;
import android.database.sqlite.SQLiteDatabase;
import android.database.sqlite.SQLiteOpenHelper;
import android.util.Log;

/**
 * Small standard-Android copy of the OS3 weather database bootstrap.
 *
 * The Flutter AOT opens this exact file through Drift:
 * /data/user_de/0/<package>/databases/weather.db
 */
final class WeatherDatabase extends SQLiteOpenHelper {
    private static final String TAG = "SnowDexHost";
    private static final String NAME = "weather.db";
    private static final int VERSION = 23;
    private static final String CITY_ID = "weathercn:101280101";

    WeatherDatabase(Context context) {
        super(context, NAME, null, VERSION);
    }

    static void initialize(Context context) {
        // The extracted Flutter host opens the device-protected path
        // /data/user_de/0/<package>/databases/weather.db. Do not let the
        // ordinary Application context silently redirect this to user/0.
        Context deviceProtected = context.createDeviceProtectedStorageContext();
        WeatherDatabase helper = new WeatherDatabase(deviceProtected);
        SQLiteDatabase database = null;
        try {
            database = helper.getWritableDatabase();
            seedDefaultCity(database);
            Log.i(TAG, "weather.db initialized path=" + database.getPath());
        } finally {
            if (database != null) {
                database.close();
            }
            helper.close();
        }
    }

    @Override
    public void onCreate(SQLiteDatabase database) {
        createSchema(database);
    }

    @Override
    public void onUpgrade(SQLiteDatabase database, int oldVersion, int newVersion) {
        // Keep existing weather rows on upgrades. The test profile only needs
        // the donor-compatible tables to exist before Drift opens the file.
        createSchema(database);
    }

    @Override
    public void onDowngrade(SQLiteDatabase database, int oldVersion, int newVersion) {
        createSchema(database);
    }

    private static void createSchema(SQLiteDatabase database) {
        database.execSQL("CREATE TABLE IF NOT EXISTS raw (insert_time VARCHAR(50), city_id VARCHAR(50) PRIMARY KEY, locale VARCHAR(50), data1 TEXT, minute_rain_data TEXT, typhoon_data TEXT)");
        database.execSQL("CREATE TABLE IF NOT EXISTS information (city_id VARCHAR(50) PRIMARY KEY, insert_time VARCHAR(50), data1 TEXT)");
        database.execSQL("CREATE TABLE IF NOT EXISTS weather (publish_time VARCHAR(50), city_id VARCHAR(50), city_name VARCHAR(50), description VARCHAR(50), temperature INTEGER, temperature_range VARCHAR(50), aqilevel VARCHAR(50), locale VARCHAR(50), weather_type INTEGER, begins VARCHAR(50), ends VARCHAR(50), humidity VARCHAR(50), feel_temperature VARCHAR(50), ultraviolet VARCHAR(50), wind_direction VARCHAR(50), wind_power VARCHAR(50), next_hour_rain_probability VARCHAR(50), sunrise VARCHAR(50), sunset VARCHAR(50), wind VARCHAR(50), data1 TEXT, day INTEGER, ebbTide VARCHAR(50), _id VARCHAR(50), pressure VARCHAR(50), pressure_unit VARCHAR(50), pressures VARCHAR(50), risingTide VARCHAR(50), timestamp VARCHARA(50), tmphighs VARCHAR(50), tmplows VARCHAR(50), water VARCHAR(50), weathernamesfrom VARCHAR(50), weathernamesto VARCHAR(50), winds VARCHAR(50), forecast_type VARCHAR(50), PRIMARY KEY (city_id, day))");
        database.execSQL("CREATE TABLE IF NOT EXISTS aqiinfo (_id VARCHAR(50), city_id VARCHAR(50) PRIMARY KEY, city VARCHAR(50), aqi VARCHAR(50), pm25 VARCHAR(50), pm10 VARCHAR(50), so2 VARCHAR(50), no2 VARCHAR(50), pub_time VARCHAR(50), src VARCHAR(50), spot VARCHAR(50), title TEXT, desc TEXT, co VARCHAR(50), o3 VARCHAR(50))");
        database.execSQL("CREATE TABLE IF NOT EXISTS alertinfo (_id VARCHAR(50), city_id VARCHAR(50), city VARCHAR(50), alert VARCHAR(50), pub_time VARCHAR(50), level VARCHAR(50), title VARCHAR(50), icon_url VARCHAR(256), detail TEXT, abnormal TEXT, holiday TEXT, type TEXT, defense TEXT, reported INTEGER, seen INTEGER, PRIMARY KEY (city_id, type))");
        database.execSQL("CREATE TABLE IF NOT EXISTS selectedcity (posID VARCHAR(50), flag INTEGER, position INTEGER, name VARCHAR(50), street_name VARCHAR(256), longtitude VARCHAR(50), latitude VARCHAR(50), belongings VARCHAR(50), extra VARCHAR(50), locale VARCHAR(50), PRIMARY KEY (posID, flag))");
        database.execSQL("CREATE TABLE IF NOT EXISTS weatherappconfig (locateswitch INTEGER, unit INTEGER)");
        database.execSQL("INSERT INTO weatherappconfig (locateswitch, unit) SELECT 1, 1 WHERE NOT EXISTS (SELECT 1 FROM weatherappconfig)");
        database.execSQL("CREATE TABLE IF NOT EXISTS alertfilter (cityid VARCHAR(50))");
        database.execSQL("CREATE TABLE IF NOT EXISTS background (insert_time VARCHAR(50), cityId VARCHAR(50) PRIMARY KEY, data TEXT)");
    }

    private static void seedDefaultCity(SQLiteDatabase database) {
        Cursor cursor = database.rawQuery("SELECT COUNT(*) FROM selectedcity WHERE flag = 1", null);
        try {
            if (cursor.moveToFirst() && cursor.getInt(0) > 0) {
                Log.i(TAG, "selectedcity already contains a flag=1 city");
                return;
            }
        } finally {
            cursor.close();
        }

        ContentValues values = new ContentValues();
        values.put("posID", CITY_ID);
        values.put("flag", 1);
        values.put("position", 0);
        values.put("name", "广州");
        values.put("street_name", "");
        values.put("longtitude", "113.264434");
        values.put("latitude", "23.129163");
        values.put("belongings", "广东省");
        values.put("extra", "");
        values.put("locale", "zh_CN");
        long rowId = database.insertWithOnConflict("selectedcity", null, values, SQLiteDatabase.CONFLICT_REPLACE);
        if (rowId == -1) {
            throw new IllegalStateException("failed to seed selectedcity");
        }
        Log.i(TAG, "seeded selectedcity cityId=" + CITY_ID + " name=广州");
    }
}
