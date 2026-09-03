#![no_std]

use core::arch::global_asm;
use core::ffi::{c_char, c_int, c_void};
use core::hint::spin_loop;
use core::mem;
use core::ptr;
use core::slice;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        spin_loop();
    }
}

#[no_mangle]
#[used]
#[link_section = ".rodata.snow"]
pub static SNOW_WEATHER_ROUTER_WATERMARK: [u8; b"SnowWeatherRouter|Snownight|v27-dedupe-surface\0".len()] =
    *b"SnowWeatherRouter|Snownight|v27-dedupe-surface\0";

#[no_mangle]
#[used]
#[link_section = ".rodata.snow"]
pub static SNOW_WEATHER_ROUTER_BUILD: [u8; b"Snow hybrid CTA callback router v18\0".len()] =
    *b"Snow hybrid CTA callback router v18\0";

const ANDROID_LOG_INFO: c_int = 4;
const ANDROID_LOG_WARN: c_int = 5;
const LOG_TAG: &[u8] = b"SnowWeatherRouter\0";
const EMPTY_CSTR: &[u8] = b"\0";

const CHANNEL_LIFECYCLE: &[u8] = b"flutter/lifecycle";
const CHANNEL_NATIVE_READY: &[u8] = b"com.xiaomi.hyperos/native_ready";
const CHANNEL_SYSTEM_PROPERTIES: &[u8] = b"com.android.os.system.properties.method.channel";
const CHANNEL_PACKAGE_MANAGER: &[u8] = b"com.android.package.manager.method.channel";
const CHANNEL_SHARED_ROOT: &[u8] = b"hyperos/shared_preferences";
const CHANNEL_SHARED_CHILD: &[u8] = b"SnowWeatherPrefs";
const CHANNEL_SHARED_APP_RUN: &[u8] = b"SnowWeatherAppRun";
const CHANNEL_DEVICE_LEVEL: &[u8] = b"com.miui.performance.devicelevelutils.method.channel";
const CHANNEL_CONFIGURATION: &[u8] = b"com.xiaomi.hyperos/configuration";
const CHANNEL_ROUNDED_CORNER: &[u8] = b"com.xiaomi.hyperos/rounded_corner";
const CHANNEL_DEVICE_INFO: &[u8] = b"com.xiaomi.hyperos/device_info";
const CHANNEL_HYPER_MATERIAL: &[u8] = b"com.xiaomi.hyperos/hyper_material";
const CHANNEL_SYSTEM_NAVIGATION: &[u8] = b"com.xiaomi.hyperos/system_navigation";
const CHANNEL_SYSTEM_BRIDGE: &[u8] = b"com.miui.hyper.system_bridge";
const CHANNEL_SHARED_PLUGIN: &[u8] = b"plugins.flutter.io/shared_preferences";
const CHANNEL_PATH_PROVIDER: &[u8] = b"plugins.flutter.io/path_provider";
const CHANNEL_URL_LAUNCHER: &[u8] = b"plugins.flutter.io/url_launcher";
const CHANNEL_HAPTIC: &[u8] = b"com.xiaomi.hyperos/haptic";
const CHANNEL_TRACE: &[u8] = b"com.xiaomi.hyperos/trace";
const CHANNEL_SETTINGS: &[u8] = b"com.android.os.provider.settings.method.channel";
const CHANNEL_WEATHER_METHOD: &[u8] = b"weather_method_channel";
const CHANNEL_WEATHER_BASIC: &[u8] = b"weather_channel";
const CHANNEL_SHORTCUT: &[u8] = b"com.android.content.shortcut.method.channel";
const CHANNEL_INTENT: &[u8] = b"com.android.content.intent.method.channel";
const CHANNEL_PERMISSION: &[u8] = b"com.android.permission.method.channel";
const CHANNEL_XMS_LOCATION: &[u8] = b"xms_location";
const CHANNEL_WEATHER_LOCATION: &[u8] = b"weather_location";
const CHANNEL_FLUTTER_NAVIGATION: &[u8] = b"flutter/navigation";

const LIFECYCLE_RESUMED: &[u8] = b"AppLifecycleState.resumed";
const LIFECYCLE_INACTIVE: &[u8] = b"AppLifecycleState.inactive";
const LIFECYCLE_PAUSED: &[u8] = b"AppLifecycleState.paused";
const LIFECYCLE_DETACHED: &[u8] = b"AppLifecycleState.detached";
const ON_READY: &[u8] = b"{\"method\":\"onReady\",\"args\":null}";
const CTA_ACTIVITY_RESULT_ACCEPTED: &[u8] = b"{\"method\":\"onActivityResult\",\"args\":{\"request_code\":1007,\"result_code\":1}}";
const CTA_ROUTE_SEARCH_CITY: &[u8] = b"{\"method\":\"pushRoute\",\"args\":\"/searchCity\"}";
const CTA_ACTIVITY_RESULT_DECLINED: &[u8] = b"{\"method\":\"onActivityResult\",\"args\":{\"request_code\":1007,\"result_code\":0}}";
const PERMISSION_RESULT_BOTH_GRANTED: &[u8] = b"{\"method\":\"on_request_permission_result\",\"args\":{\"permissions\":[\"android.permission.ACCESS_FINE_LOCATION\",\"android.permission.ACCESS_COARSE_LOCATION\"],\"grant_results\":[0,0],\"request_code\":10000}}";
const PERMISSION_RESULT_FINE_ONLY: &[u8] = b"{\"method\":\"on_request_permission_result\",\"args\":{\"permissions\":[\"android.permission.ACCESS_FINE_LOCATION\",\"android.permission.ACCESS_COARSE_LOCATION\"],\"grant_results\":[0,-1],\"request_code\":10000}}";
const PERMISSION_RESULT_COARSE_ONLY: &[u8] = b"{\"method\":\"on_request_permission_result\",\"args\":{\"permissions\":[\"android.permission.ACCESS_FINE_LOCATION\",\"android.permission.ACCESS_COARSE_LOCATION\"],\"grant_results\":[-1,0],\"request_code\":10000}}";
const PERMISSION_RESULT_DENIED: &[u8] = b"{\"method\":\"on_request_permission_result\",\"args\":{\"permissions\":[\"android.permission.ACCESS_FINE_LOCATION\",\"android.permission.ACCESS_COARSE_LOCATION\"],\"grant_results\":[-1,-1],\"request_code\":10000}}";

const JSON_NULL: &[u8] = b"[null]";
const JSON_TRUE: &[u8] = b"[true]";
const JSON_FALSE: &[u8] = b"[false]";
const JSON_ZERO: &[u8] = b"[0]";
const JSON_SIX: &[u8] = b"[6]";
const JSON_2025: &[u8] = b"[2025]";
const JSON_MINUS_ONE: &[u8] = b"[-1]";
const JSON_FALSE_STRING: &[u8] = b"[\"false\"]";
const JSON_EMPTY_MAP: &[u8] = b"[{}]";
const JSON_EMPTY_LIST: &[u8] = b"[[]]";
const JSON_EMPTY_STRING: &[u8] = b"[\"[]\"]";
const JSON_ZERO_CORNERS: &[u8] = b"[[0,0,0,0]]";
const JSON_GLASS_UNSUPPORTED: &[u8] =
    b"[{\"isSupportMaterial\":false,\"isSupportGlass\":false}]";
const JSON_SHARED_OPEN: &[u8] = b"[\"SnowWeatherPrefs\"]";
const JSON_SHARED_APP_RUN_OPEN: &[u8] = b"[\"SnowWeatherAppRun\"]";
const JSON_DEVICE_FLAGSHIP: &[u8] =
    b"[{\"cpu_level\":3,\"gpu_level\":3,\"ram_level\":3}]";
const JSON_PACKAGE_INFO: &[u8] = b"[{\"versionName\":\"[IP]-R-Snow-v18-hybrid\",\"versionCode\":180000255,\"lastUpdateTime\":0,\"applicationInfo\":{\"flags\":0,\"enabled\":true}}]";
const JSON_CN: &[u8] = b"[\"cn\"]";
const JSON_ZH_CN: &[u8] = b"[\"zh-CN\"]";
const JSON_CN_REGION: &[u8] = b"[\"CN\"]";
const JSON_LOCATION_TEST: &[u8] = br##"["{\"mLatitude\":23.108,\"mLongitude\":113.265,\"mStreetName\":\"\",\"mCityName\":\"\",\"mAdminArea\":\"\",\"mSubLocality\":\"\",\"mCountryName\":\"China\",\"mErrorCode\":0,\"mErrorInfo\":\"\",\"mLocationType\":\"5\"}"]"##;
const JSON_CONFIGURATION: &[u8] = b"[{\"screen_layout\":0,\"orientation\":1,\"color_mode\":0,\"screen_type\":0,\"screen_width_dp\":393,\"screen_height_dp\":873,\"smallest_screen_width_dp\":393,\"density_dpi\":440,\"display_id\":0,\"display_name\":\"Built-in Screen\",\"display_logical_density_dpi\":440,\"display_shape_width\":1080,\"display_shape_height\":2400,\"display_cutout\":{\"left\":0,\"top\":0,\"right\":0,\"bottom\":0,\"bounding_rect_left\":{\"left\":0,\"top\":0,\"right\":0,\"bottom\":0},\"bounding_rect_top\":{\"left\":0,\"top\":0,\"right\":0,\"bottom\":0},\"bounding_rect_right\":{\"left\":0,\"top\":0,\"right\":0,\"bottom\":0},\"bounding_rect_bottom\":{\"left\":0,\"top\":0,\"right\":0,\"bottom\":0}},\"window_bounds\":{\"left\":0,\"top\":0,\"right\":1080,\"bottom\":2400},\"is_multi_window\":false,\"dm_width_pixels\":1080,\"dm_height_pixels\":2400,\"dm_density\":2.75,\"dm_density_dpi\":440,\"dm_scaled_density\":2.75,\"dm_x_dpi\":440,\"dm_y_dpi\":440}]";

const STANDARD_NULL: &[u8] = &[0x00, 0x00];
const PIGEON_NULL_REPLY: &[u8] = &[0x0c, 0x01, 0x00];
const PIGEON_EMPTY_MAP_REPLY: &[u8] = &[0x0c, 0x01, 0x0d, 0x00];
const PIGEON_SHARED_PREFS_PREFIX: &[u8] = b"dev.flutter.pigeon.shared_preferences_android.SharedPreferencesApi.";
const STANDARD_TRUE: &[u8] = &[0x00, 0x01];
const STANDARD_FALSE: &[u8] = &[0x00, 0x02];
const STANDARD_EMPTY_MAP: &[u8] = &[0x00, 0x0d, 0x00];
const STANDARD_APP_RUN_TRUE_MAP: &[u8] = &[
    0x00, 0x0d, 0x01,
    0x07, 0x07, b'a', b'p', b'p', b'_', b'r', b'u', b'n',
    0x01,
];
const STANDARD_INT_ZERO: &[u8] = &[0x00, 0x03, 0x00, 0x00, 0x00, 0x00];
const STANDARD_INT_TWO: &[u8] = &[0x00, 0x03, 0x02, 0x00, 0x00, 0x00];
const STANDARD_INT_THREE: &[u8] = &[0x00, 0x03, 0x03, 0x00, 0x00, 0x00];
const BASIC_NULL: &[u8] = &[0x00];
const BASIC_APP_PATH: &[u8] = b"\x07\x21/data/user_de/0/com.miui.weather3";

const ARG_AOT: &[u8] = b"--aot-shared-library-name=libapp.so";
const ARG_ICU: &[u8] = b"--icu-symbol-prefix=_binary_icudtl_dat";
const ARG_IMPELLER: &[u8] = b"--impeller-backend=vulkan";
const ENTRYPOINT: &[u8] = b"main";

#[repr(C)]
pub struct ANativeActivityCallbacks {
    on_start: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_resume: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_save_instance_state:
        Option<unsafe extern "C" fn(*mut ANativeActivity, *mut usize) -> *mut c_void>,
    on_pause: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_stop: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_destroy: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_window_focus_changed: Option<unsafe extern "C" fn(*mut ANativeActivity, c_int)>,
    on_native_window_created:
        Option<unsafe extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    on_native_window_resized:
        Option<unsafe extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    on_native_window_redraw_needed:
        Option<unsafe extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    on_native_window_destroyed:
        Option<unsafe extern "C" fn(*mut ANativeActivity, *mut ANativeWindow)>,
    on_input_queue_created: Option<unsafe extern "C" fn(*mut ANativeActivity, *mut c_void)>,
    on_input_queue_destroyed: Option<unsafe extern "C" fn(*mut ANativeActivity, *mut c_void)>,
    on_content_rect_changed: Option<unsafe extern "C" fn(*mut ANativeActivity, *const c_void)>,
    on_configuration_changed: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
    on_low_memory: Option<unsafe extern "C" fn(*mut ANativeActivity)>,
}

#[repr(C)]
pub struct ANativeActivity {
    callbacks: *mut ANativeActivityCallbacks,
    vm: *mut c_void,
    env: *mut c_void,
    clazz: *mut c_void,
    internal_data_path: *const c_char,
    external_data_path: *const c_char,
    sdk_version: i32,
    _sdk_padding: i32,
    instance: *mut c_void,
    asset_manager: *mut c_void,
    obb_path: *const c_char,
}

#[repr(C)]
pub struct ANativeWindow {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct AInputQueue {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct AInputEvent {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ALooper {
    _opaque: [u8; 0],
}

#[repr(C)]
struct RouterState {
    activity: *mut ANativeActivity,
    runtime: *mut c_void,
    holder: *mut c_void,
    window: *mut ANativeWindow,
    input_queue: *mut AInputQueue,
    input_looper: *mut ALooper,
    resumed: i32,
    initialized_callbacks: i32,
}

impl RouterState {
    const fn empty() -> Self {
        Self {
            activity: ptr::null_mut(),
            runtime: ptr::null_mut(),
            holder: ptr::null_mut(),
            window: ptr::null_mut(),
            input_queue: ptr::null_mut(),
            input_looper: ptr::null_mut(),
            resumed: 0,
            initialized_callbacks: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HyperString {
    ptr: *const u8,
    len: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HyperSlice<T> {
    ptr: *const T,
    len: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HyperViewportMetrics {
    device_pixel_ratio: f64,
    physical_width: f64,
    physical_height: f64,
    physical_touch_slop: f64,
    physical_view_padding_top: f64,
    physical_view_padding_right: f64,
    physical_view_padding_bottom: f64,
    physical_view_padding_left: f64,
    physical_view_inset_top: f64,
    physical_view_inset_right: f64,
    physical_view_inset_bottom: f64,
    physical_view_inset_left: f64,
    physical_system_gesture_inset_top: f64,
    physical_system_gesture_inset_right: f64,
    physical_system_gesture_inset_bottom: f64,
    physical_system_gesture_inset_left: f64,
    display_features: HyperSlice<f64>,
    display_feature_bounds: HyperSlice<i32>,
    display_feature_types: HyperSlice<i32>,
    display_id: usize,
}

impl HyperViewportMetrics {
    const fn empty() -> Self {
        Self {
            device_pixel_ratio: 2.75,
            physical_width: 1080.0,
            physical_height: 2400.0,
            physical_touch_slop: 0.0,
            physical_view_padding_top: 0.0,
            physical_view_padding_right: 104.0,
            physical_view_padding_bottom: 0.0,
            physical_view_padding_left: 53.0,
            physical_view_inset_top: 0.0,
            physical_view_inset_right: 0.0,
            physical_view_inset_bottom: 0.0,
            physical_view_inset_left: 0.0,
            physical_system_gesture_inset_top: 0.0,
            physical_system_gesture_inset_right: 0.0,
            physical_system_gesture_inset_bottom: 0.0,
            physical_system_gesture_inset_left: 0.0,
            display_features: HyperSlice { ptr: ptr::null(), len: 0 },
            display_feature_bounds: HyperSlice { ptr: ptr::null(), len: 0 },
            display_feature_types: HyperSlice { ptr: ptr::null(), len: 0 },
            display_id: 0,
        }
    }
}

static mut STATE: RouterState = RouterState::empty();
static mut HYPER_CALLBACKS: [usize; 16] = [0; 16];
static mut VIEWPORT_METRICS: HyperViewportMetrics = HyperViewportMetrics::empty();
static mut USER_AGREED: i32 = 0;
static mut LOCATION_PERMISSION_GRANTED: i32 = 0;

static mut ENGINE_LOCALES: [HyperString; 5] = [
    HyperString { ptr: b"zh".as_ptr(), len: 2 },
    HyperString { ptr: b"CN".as_ptr(), len: 2 },
    HyperString { ptr: b"".as_ptr(), len: 0 },
    HyperString { ptr: b"".as_ptr(), len: 0 },
    HyperString { ptr: b"".as_ptr(), len: 0 },
];

static mut ENGINE_ARGS: [HyperString; 4] = [
    HyperString {
        ptr: ptr::null(),
        len: 0,
    },
    HyperString {
        ptr: ptr::null(),
        len: 0,
    },
    HyperString {
        ptr: ptr::null(),
        len: 0,
    },
    HyperString {
        ptr: ptr::null(),
        len: 0,
    },
];

unsafe extern "C" {
    static HYPER_FLUTTER_INTERFACE: u8;
    fn ANativeWindow_getWidth(window: *mut ANativeWindow) -> i32;
    fn ANativeWindow_getHeight(window: *mut ANativeWindow) -> i32;
    fn ALooper_forThread() -> *mut ALooper;
    fn ALooper_prepare(opts: c_int) -> *mut ALooper;
    fn ALooper_acquire(looper: *mut ALooper);
    fn ALooper_release(looper: *mut ALooper);
    fn AInputQueue_attachLooper(
        queue: *mut AInputQueue,
        looper: *mut ALooper,
        ident: c_int,
        callback: Option<unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int>,
        data: *mut c_void,
    );
    fn AInputQueue_detachLooper(queue: *mut AInputQueue);
    fn AInputQueue_getEvent(queue: *mut AInputQueue, event: *mut *mut AInputEvent) -> c_int;
    fn AInputQueue_preDispatchEvent(queue: *mut AInputQueue, event: *mut AInputEvent) -> c_int;
    fn AInputQueue_finishEvent(queue: *mut AInputQueue, event: *mut AInputEvent, handled: c_int);
    fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
    fn snow_runtime_create_call(
        function: *const c_void,
        version: usize,
        data_path: *const u8,
        data_path_len: usize,
        args: *const HyperString,
        argc: usize,
    ) -> *mut c_void;
    fn snow_shell_launch_call(
        function: *const c_void,
        holder: *mut c_void,
        entrypoint: *const u8,
        entrypoint_len: usize,
        asset_manager: *mut c_void,
        metadata: *const usize,
    );
}

global_asm!(
    r#"
    .text
    .p2align 2
    .global snow_runtime_create_call
    .type snow_runtime_create_call,%function
snow_runtime_create_call:
    sub sp, sp, #32
    str x30, [sp, #16]
    mov x9, x0
    mov x10, x2
    mov x11, x3
    stp x4, x5, [sp]
    mov x0, x1
    mov x1, xzr
    mov x2, xzr
    mov x3, x10
    mov x4, x11
    mov x5, x10
    mov x6, x11
    blr x9
    ldr x30, [sp, #16]
    add sp, sp, #32
    ret

    .global snow_shell_launch_call
    .type snow_shell_launch_call,%function
snow_shell_launch_call:
    sub sp, sp, #48
    str x30, [sp, #32]
    mov x9, x0
    stp x4, x5, [sp]
    stp xzr, xzr, [sp, #16]
    mov x0, x1
    mov x1, x2
    mov x2, x3
    mov x3, xzr
    mov x4, xzr
    mov x5, xzr
    mov x6, xzr
    blr x9
    ldr x30, [sp, #32]
    add sp, sp, #48
    ret
"#
);

unsafe fn state_ptr() -> *mut RouterState {
    ptr::addr_of_mut!(STATE)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_miui_weather3_ActivityWeatherMain_nativeSetUserAgreement(
    _: *mut c_void,
    _: *mut c_void,
    agreed: u8,
) {
    USER_AGREED = if agreed != 0 { 1 } else { 0 };
    if USER_AGREED != 0 {
        log_static(ANDROID_LOG_INFO, b"Snow user agreement accepted by CTA\0");
    } else {
        log_static(ANDROID_LOG_INFO, b"Snow user agreement pending\0");
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_miui_weather3_ActivityWeatherMain_nativeSetLocationPermission(
    _: *mut c_void,
    _: *mut c_void,
    granted: u8,
) {
    LOCATION_PERMISSION_GRANTED = if granted != 0 { 1 } else { 0 };
    if LOCATION_PERMISSION_GRANTED != 0 {
        log_static(ANDROID_LOG_INFO, b"Snow location permission granted\0");
    } else {
        log_static(ANDROID_LOG_INFO, b"Snow location permission denied\0");
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_miui_weather3_ActivityWeatherMain_nativeDeliverActivityResult(
    _: *mut c_void,
    _: *mut c_void,
    result_code: i32,
) {
    if result_code == 1 {
        USER_AGREED = 1;
        dispatch(CHANNEL_INTENT, CTA_ACTIVITY_RESULT_ACCEPTED);
        dispatch(CHANNEL_FLUTTER_NAVIGATION, CTA_ROUTE_SEARCH_CITY);
        log_static(ANDROID_LOG_INFO, b"Snow delivered CTA resultCode=1 and search route to Flutter\0");
    } else {
        USER_AGREED = 0;
        dispatch(CHANNEL_INTENT, CTA_ACTIVITY_RESULT_DECLINED);
        log_static(ANDROID_LOG_INFO, b"Snow delivered CTA decline to Flutter\0");
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_miui_weather3_ActivityWeatherMain_nativeDeliverPermissionResult(
    _: *mut c_void,
    _: *mut c_void,
    fine_result: i32,
    coarse_result: i32,
) {
    LOCATION_PERMISSION_GRANTED = if fine_result == 0 || coarse_result == 0 { 1 } else { 0 };
    let payload = if fine_result == 0 && coarse_result == 0 {
        PERMISSION_RESULT_BOTH_GRANTED
    } else if fine_result == 0 {
        PERMISSION_RESULT_FINE_ONLY
    } else if coarse_result == 0 {
        PERMISSION_RESULT_COARSE_ONLY
    } else {
        PERMISSION_RESULT_DENIED
    };
    dispatch(CHANNEL_PERMISSION, payload);
    log_static(ANDROID_LOG_INFO, b"Snow delivered location permission result to Flutter\0");
}

unsafe fn interface_entry(offset: usize) -> *const c_void {
    let base = ptr::addr_of!(HYPER_FLUTTER_INTERFACE) as *const u8;
    ptr::read_unaligned(base.add(offset) as *const *const c_void)
}

unsafe fn call_runtime_destroy(runtime: *mut c_void) {
    type Function = unsafe extern "C" fn(*mut c_void);
    let function: Function = mem::transmute(interface_entry(0x10));
    function(runtime);
}

unsafe fn call_create_holder(
    runtime: *mut c_void,
    state: *mut RouterState,
    callbacks: *const usize,
) -> *mut c_void {
    type Function = unsafe extern "C" fn(*mut c_void, *mut RouterState, *const usize) -> *mut c_void;
    let function: Function = mem::transmute(interface_entry(0x40));
    function(runtime, state, callbacks)
}

unsafe fn call_destroy_holder(holder: *mut c_void) {
    type Function = unsafe extern "C" fn(*mut c_void);
    let function: Function = mem::transmute(interface_entry(0x48));
    function(holder);
}

unsafe fn call_dispatch(
    holder: *mut c_void,
    channel: *const u8,
    channel_len: usize,
    payload: *const u8,
    payload_len: usize,
) {
    type Function = unsafe extern "C" fn(
        *mut c_void,
        *const u8,
        usize,
        *const u8,
        usize,
        usize,
    );
    let function: Function = mem::transmute(interface_entry(0x60));
    function(holder, channel, channel_len, payload, payload_len, 0);
}

unsafe fn call_reply(
    holder: *mut c_void,
    payload: *const u8,
    payload_len: usize,
    reply_id: usize,
) {
    type Function = unsafe extern "C" fn(*mut c_void, *const u8, usize, usize);
    let function: Function = mem::transmute(interface_entry(0x78));
    function(holder, payload, payload_len, reply_id);
}

unsafe fn call_set_locales(holder: *mut c_void) {
    type Function = unsafe extern "C" fn(*mut c_void, *const HyperString, usize);
    let function: Function = mem::transmute(interface_entry(0x238));
    function(holder, ptr::addr_of!(ENGINE_LOCALES) as *const HyperString, 5);
}

unsafe fn call_set_viewport_metrics(holder: *mut c_void, metrics: *const HyperViewportMetrics) {
    type Function = unsafe extern "C" fn(*mut c_void, *const HyperViewportMetrics);
    let function: Function = mem::transmute(interface_entry(0xa0));
    function(holder, metrics);
}

unsafe fn call_surface_create(holder: *mut c_void, window: *mut ANativeWindow) {
    type Function = unsafe extern "C" fn(*mut c_void, *mut ANativeWindow);
    let function: Function = mem::transmute(interface_entry(0xa8));
    function(holder, window);
}

unsafe fn call_set_window_size(holder: *mut c_void, width: i32, height: i32) {
    type Function = unsafe extern "C" fn(*mut c_void, i32, i32);
    let function: Function = mem::transmute(interface_entry(0xb0));
    function(holder, width, height);
}

unsafe fn call_update_display_attributes(
    holder: *mut c_void,
    refresh_rate: f64,
    width: f64,
    height: f64,
    device_pixel_ratio: f64,
) {
    type Function = unsafe extern "C" fn(*mut c_void, f64, f64, f64, f64);
    let function: Function = mem::transmute(interface_entry(0xc8));
    function(holder, refresh_rate, width, height, device_pixel_ratio);
}

unsafe fn call_surface_destroy(holder: *mut c_void) {
    type Function = unsafe extern "C" fn(*mut c_void);
    let function: Function = mem::transmute(interface_entry(0xc0));
    function(holder);
}

fn c_strlen(mut value: *const c_char) -> usize {
    if value.is_null() {
        return 0;
    }
    let mut length = 0usize;
    unsafe {
        while ptr::read(value) != 0 {
            length += 1;
            value = value.add(1);
        }
    }
    length
}

fn copy_bytes(output: &mut [u8], position: &mut usize, value: &[u8]) {
    for &byte in value {
        if *position + 1 >= output.len() {
            return;
        }
        output[*position] = byte;
        *position += 1;
    }
}

unsafe fn log_static(priority: c_int, message: &'static [u8]) {
    let _ = __android_log_write(
        priority,
        LOG_TAG.as_ptr() as *const c_char,
        message.as_ptr() as *const c_char,
    );
}

unsafe fn log_platform_message(channel: &[u8], payload: &[u8]) {
    let mut output = [0u8; 640];
    let mut position = 0usize;
    copy_bytes(&mut output, &mut position, b"channel=");
    copy_bytes(&mut output, &mut position, channel);
    copy_bytes(&mut output, &mut position, b" payload=");
    for &byte in payload.iter().take(384) {
        if position + 1 >= output.len() {
            break;
        }
        output[position] = if byte == b'\n' || byte == b'\r' || byte == 0 {
            b'.'
        } else if byte.is_ascii_graphic() || byte == b' ' {
            byte
        } else {
            b'.'
        };
        position += 1;
    }
    output[position] = 0;
    let _ = __android_log_write(
        ANDROID_LOG_INFO,
        LOG_TAG.as_ptr() as *const c_char,
        output.as_ptr() as *const c_char,
    );
}

fn bytes_equal(left: &[u8], right: &[u8]) -> bool {
    left == right
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    let mut index = 0usize;
    while index + needle.len() <= haystack.len() {
        if &haystack[index..index + needle.len()] == needle {
            return true;
        }
        index += 1;
    }
    false
}

fn json_method_is(payload: &[u8], method: &[u8]) -> bool {
    contains(payload, method)
}

fn read_standard_size(payload: &[u8], position: &mut usize) -> Option<usize> {
    if *position >= payload.len() {
        return None;
    }
    let first = payload[*position] as usize;
    *position += 1;
    if first < 254 {
        return Some(first);
    }
    if first == 254 {
        if *position + 2 > payload.len() {
            return None;
        }
        let value = payload[*position] as usize | ((payload[*position + 1] as usize) << 8);
        *position += 2;
        return Some(value);
    }
    if *position + 4 > payload.len() {
        return None;
    }
    let value = payload[*position] as usize
        | ((payload[*position + 1] as usize) << 8)
        | ((payload[*position + 2] as usize) << 16)
        | ((payload[*position + 3] as usize) << 24);
    *position += 4;
    Some(value)
}

fn standard_method<'a>(payload: &'a [u8]) -> Option<&'a [u8]> {
    if payload.first().copied() != Some(7) {
        return None;
    }
    let mut position = 1usize;
    let length = read_standard_size(payload, &mut position)?;
    if position + length > payload.len() {
        return None;
    }
    Some(&payload[position..position + length])
}

fn looks_json(payload: &[u8]) -> bool {
    for &byte in payload.iter().take(8) {
        if byte == b' ' || byte == b'\t' || byte == b'\r' || byte == b'\n' {
            continue;
        }
        return byte == b'{' || byte == b'[';
    }
    false
}

unsafe fn reply(state: *mut RouterState, reply_id: usize, payload: &[u8]) {
    if state.is_null() || reply_id == 0 {
        return;
    }
    let holder = (*state).holder;
    if holder.is_null() {
        return;
    }
    call_reply(holder, payload.as_ptr(), payload.len(), reply_id);
}

unsafe fn dispatch(channel: &[u8], payload: &[u8]) {
    let state = state_ptr();
    if (*state).holder.is_null() {
        return;
    }
    call_dispatch(
        (*state).holder,
        channel.as_ptr(),
        channel.len(),
        payload.as_ptr(),
        payload.len(),
    );
}

unsafe fn send_lifecycle(payload: &[u8]) {
    dispatch(CHANNEL_LIFECYCLE, payload);
}

unsafe fn send_on_ready() {
    dispatch(CHANNEL_NATIVE_READY, ON_READY);
}

unsafe extern "C" fn hyper_noop() {}

unsafe extern "C" fn hyper_first_frame_callback(_: *mut c_void) {
    log_static(ANDROID_LOG_INFO, b"Snow onFirstFrame callback\0");
}

unsafe extern "C" fn platform_message_callback(
    context: *mut c_void,
    channel_ptr: *const u8,
    channel_len: usize,
    reply_id: usize,
    payload_ptr: *const u8,
    payload_len: usize,
) {
    if reply_id == 0 || channel_ptr.is_null() {
        return;
    }
    let state = if context.is_null() {
        state_ptr()
    } else {
        context as *mut RouterState
    };
    let channel = slice::from_raw_parts(channel_ptr, channel_len);
    let payload = if payload_ptr.is_null() {
        &[][..]
    } else {
        slice::from_raw_parts(payload_ptr, payload_len)
    };
    log_platform_message(channel, payload);

    // Pigeon SharedPreferencesApi uses a BasicMessageChannel with a
    // StandardMessageCodec list reply, not a MethodCodec envelope.
    if contains(channel, PIGEON_SHARED_PREFS_PREFIX) {
        if contains(channel, b".getAll") {
            reply(state, reply_id, PIGEON_EMPTY_MAP_REPLY);
        } else {
            reply(state, reply_id, PIGEON_NULL_REPLY);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_NATIVE_READY) {
        reply(state, reply_id, JSON_TRUE);
        send_on_ready();
        return;
    }

    if bytes_equal(channel, CHANNEL_PACKAGE_MANAGER) {
        reply(state, reply_id, JSON_PACKAGE_INFO);
        return;
    }

    if bytes_equal(channel, CHANNEL_SYSTEM_PROPERTIES) {
        if json_method_is(payload, b"getAll") {
            reply(state, reply_id, JSON_EMPTY_MAP);
        } else if json_method_is(payload, b"getString") {
            if contains(payload, b"ro.miui.region") || contains(payload, b"ro.product.country") {
                reply(state, reply_id, JSON_CN_REGION);
            } else if contains(payload, b"persist.sys.locale")
                || contains(payload, b"ro.product.locale")
                || contains(payload, b"ro.product.locale.language")
            {
                reply(state, reply_id, JSON_ZH_CN);
            } else {
                reply(state, reply_id, JSON_FALSE_STRING);
            }
        } else if json_method_is(payload, b"getInt") || json_method_is(payload, b"getLong") {
            reply(state, reply_id, JSON_ZERO);
        } else if json_method_is(payload, b"getBool") || json_method_is(payload, b"getBoolean") {
            reply(state, reply_id, JSON_FALSE);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SHARED_ROOT) {
        if json_method_is(payload, b"open") {
            if contains(payload, b"com.miui.providers.weather.apprun") {
                reply(state, reply_id, JSON_SHARED_APP_RUN_OPEN);
            } else {
                reply(state, reply_id, JSON_SHARED_OPEN);
            }
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SHARED_APP_RUN) {
        match standard_method(payload) {
            Some(method) if method == b"reload" || method == b"getAll" => {
                if USER_AGREED != 0 {
                    reply(state, reply_id, STANDARD_APP_RUN_TRUE_MAP);
                } else {
                    reply(state, reply_id, STANDARD_EMPTY_MAP);
                }
            }
            _ => reply(state, reply_id, STANDARD_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SHARED_CHILD) {
        match standard_method(payload) {
            Some(method) if method == b"reload" || method == b"getAll" => {
                reply(state, reply_id, STANDARD_EMPTY_MAP)
            }
            _ => reply(state, reply_id, STANDARD_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_DEVICE_LEVEL) {
        if json_method_is(payload, b"getDeviceLevel") {
            reply(state, reply_id, JSON_DEVICE_FLAGSHIP);
        } else if json_method_is(payload, b"getMiuiMiddleVersion")
            || json_method_is(payload, b"getMiuiLiteVersion")
        {
            reply(state, reply_id, JSON_ZERO);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_CONFIGURATION) {
        if json_method_is(payload, b"roundedCorner")
            || json_method_is(payload, b"getScreenCornerRadius")
            || json_method_is(payload, b"getRoundedCorners")
        {
            reply(state, reply_id, JSON_ZERO_CORNERS);
        } else if json_method_is(payload, b"configuration")
            || json_method_is(payload, b"configurations")
        {
            reply(state, reply_id, JSON_CONFIGURATION);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_ROUNDED_CORNER) {
        reply(state, reply_id, JSON_EMPTY_LIST);
        return;
    }

    if bytes_equal(channel, CHANNEL_DEVICE_INFO) {
        if json_method_is(payload, b"getDeviceInfo") {
            reply(state, reply_id, JSON_EMPTY_MAP);
        } else if json_method_is(payload, b"Capability") {
            reply(state, reply_id, JSON_FALSE);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_HYPER_MATERIAL) {
        if json_method_is(payload, b"getGlassCapability") {
            reply(state, reply_id, JSON_GLASS_UNSUPPORTED);
        } else if json_method_is(payload, b"getBackgroundBlurEnable")
            || json_method_is(payload, b"getMaterialStyle")
        {
            reply(state, reply_id, JSON_ZERO);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SYSTEM_NAVIGATION) {
        if json_method_is(payload, b"getNavigationMode") {
            reply(state, reply_id, b"[2]");
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }


    if bytes_equal(channel, CHANNEL_SETTINGS) {
        match standard_method(payload) {
            Some(method) if method == b"getLong" => {
                if contains(payload, b"navigation_mode") {
                    reply(state, reply_id, STANDARD_INT_TWO);
                } else if contains(payload, b"location_mode") {
                    // Android reports secure.location_mode=3 when location is
                    // enabled; the original weather app uses this before
                    // entering LocationDataManager::strategy2.
                    reply(state, reply_id, STANDARD_INT_THREE);
                } else {
                    reply(state, reply_id, STANDARD_INT_ZERO);
                }
            }
            _ => reply(state, reply_id, STANDARD_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_XMS_LOCATION) || bytes_equal(channel, CHANNEL_WEATHER_LOCATION) {
        if json_method_is(payload, b"locationFromAmap") || json_method_is(payload, b"locationFromNlp") || json_method_is(payload, b"locationFromWifi") || json_method_is(payload, b"locationFromPhone") {
            log_static(ANDROID_LOG_INFO, b"Snow location channel served v24 result\0");
            reply(state, reply_id, JSON_LOCATION_TEST);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_WEATHER_METHOD) {
        if json_method_is(payload, b"get_device_com_level") {
            reply(state, reply_id, JSON_SIX);
        } else if json_method_is(payload, b"get_device_com_version") {
            reply(state, reply_id, JSON_2025);
        } else if json_method_is(payload, b"get_job_is_working") {
            reply(state, reply_id, JSON_FALSE);
        } else if json_method_is(payload, b"requestScheduleTask") {
            reply(state, reply_id, JSON_TRUE);
        } else if json_method_is(payload, b"get_network_country_iso") {
            reply(state, reply_id, JSON_CN);
        } else if json_method_is(payload, b"get_network_info_is_connected") {
            reply(state, reply_id, JSON_TRUE);
        } else if json_method_is(payload, b"check_permission") {
            if LOCATION_PERMISSION_GRANTED != 0 {
                reply(state, reply_id, JSON_ZERO);
            } else {
                reply(state, reply_id, JSON_MINUS_ONE);
            }
        } else if json_method_is(payload, b"send_broadcast")
            || json_method_is(payload, b"update_cta_result")
            || json_method_is(payload, b"track_normal_event")
            || json_method_is(payload, b"track_page_event")
        {
            reply(state, reply_id, JSON_TRUE);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_WEATHER_BASIC) {
        match standard_method(payload) {
            Some(method) if method == b"get_app_path" => {
                reply(state, reply_id, BASIC_APP_PATH)
            }
            _ => reply(state, reply_id, BASIC_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_INTENT) {
        if json_method_is(payload, b"startActivityForResult")
            || json_method_is(payload, b"startActivity")
        {
            // The Java hybrid host owns Android activity launches. It starts the
            // CTA dialog with requestCode=1007 and recreates this activity only
            // after the real resultCode=1 acceptance has been persisted.
            reply(state, reply_id, JSON_TRUE);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SHORTCUT) {
        if json_method_is(payload, b"getDynamicShortcuts") {
            reply(state, reply_id, JSON_EMPTY_STRING);
        } else if json_method_is(payload, b"reportShortcutUsed")
            || json_method_is(payload, b"addDynamicShortcuts")
            || json_method_is(payload, b"updateShortcuts")
        {
            reply(state, reply_id, JSON_TRUE);
        } else {
            reply(state, reply_id, JSON_NULL);
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SYSTEM_BRIDGE) {
        match standard_method(payload) {
            Some(method) if method == b"is_user_unlocked" => reply(state, reply_id, STANDARD_TRUE),
            _ => reply(state, reply_id, STANDARD_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_SHARED_PLUGIN) {
        match standard_method(payload) {
            Some(method) if method == b"getAll" => reply(state, reply_id, STANDARD_EMPTY_MAP),
            _ => reply(state, reply_id, STANDARD_NULL),
        }
        return;
    }

    if bytes_equal(channel, CHANNEL_PATH_PROVIDER) {
        reply(state, reply_id, STANDARD_NULL);
        return;
    }

    if bytes_equal(channel, CHANNEL_URL_LAUNCHER) {
        reply(state, reply_id, STANDARD_FALSE);
        return;
    }

    if bytes_equal(channel, CHANNEL_HAPTIC) || bytes_equal(channel, CHANNEL_TRACE) {
        reply(state, reply_id, STANDARD_NULL);
        return;
    }

    if looks_json(payload) {
        reply(state, reply_id, JSON_NULL);
    } else {
        reply(state, reply_id, STANDARD_NULL);
    }
}

unsafe fn initialize_hyper_callbacks() {
    let callbacks = ptr::addr_of_mut!(HYPER_CALLBACKS) as *mut usize;
    ptr::write(callbacks.add(0), 0x68);
    ptr::write(callbacks.add(1), hyper_noop as *const () as usize);
    ptr::write(callbacks.add(2), hyper_first_frame_callback as *const () as usize);
    ptr::write(callbacks.add(3), hyper_noop as *const () as usize);
    ptr::write(callbacks.add(4), platform_message_callback as *const () as usize);
    let mut index = 5usize;
    while index <= 12 {
        ptr::write(callbacks.add(index), hyper_noop as *const () as usize);
        index += 1;
    }
    ptr::write(callbacks.add(13), 0x18);
    ptr::write(callbacks.add(14), hyper_noop as *const () as usize);
    ptr::write(callbacks.add(15), hyper_noop as *const () as usize);
}

unsafe fn initialize_engine_args() {
    let args = ptr::addr_of_mut!(ENGINE_ARGS) as *mut HyperString;
    ptr::write(
        args.add(0),
        HyperString {
            ptr: ARG_AOT.as_ptr(),
            len: ARG_AOT.len(),
        },
    );
    ptr::write(
        args.add(1),
        HyperString {
            ptr: ARG_ICU.as_ptr(),
            len: ARG_ICU.len(),
        },
    );
    ptr::write(
        args.add(2),
        HyperString {
            ptr: ARG_IMPELLER.as_ptr(),
            len: ARG_IMPELLER.len(),
        },
    );
    ptr::write(
        args.add(3),
        HyperString {
            ptr: ARG_IMPELLER.as_ptr(),
            len: ARG_IMPELLER.len(),
        },
    );
}

unsafe fn update_window_size(state: *mut RouterState) {
    if state.is_null() || (*state).holder.is_null() || (*state).window.is_null() {
        return;
    }
    let width = ANativeWindow_getWidth((*state).window);
    let height = ANativeWindow_getHeight((*state).window);
    let metrics = ptr::addr_of_mut!(VIEWPORT_METRICS);
    (*metrics).physical_width = width as f64;
    (*metrics).physical_height = height as f64;

    // Match the verified OS4 host ordering before its first frame:
    // display attributes -> viewport metrics -> surface changed.
    call_update_display_attributes((*state).holder, 0.0, 0.0, 0.0, 2.75);
    call_set_viewport_metrics((*state).holder, metrics as *const HyperViewportMetrics);
    call_set_window_size((*state).holder, width, height);
    log_static(ANDROID_LOG_INFO, b"Snow viewport metrics pushed\0");
}

unsafe fn initialize_engine() {
    let state = state_ptr();
    if !(*state).runtime.is_null() || (*state).activity.is_null() {
        return;
    }

    initialize_hyper_callbacks();
    initialize_engine_args();

    let activity = (*state).activity;
    let data_path = if (*activity).internal_data_path.is_null() {
        EMPTY_CSTR.as_ptr() as *const c_char
    } else {
        (*activity).internal_data_path
    };
    let data_path_len = c_strlen(data_path);
    let runtime_function = interface_entry(0x08);
    let runtime = snow_runtime_create_call(
        runtime_function,
        0x0200_0000,
        data_path as *const u8,
        data_path_len,
        ptr::addr_of!(ENGINE_ARGS) as *const HyperString,
        3,
    );
    (*state).runtime = runtime;
    if runtime.is_null() {
        log_static(ANDROID_LOG_WARN, b"runtime_create failed\0");
        return;
    }

    let holder = call_create_holder(
        runtime,
        state,
        ptr::addr_of!(HYPER_CALLBACKS) as *const usize,
    );
    (*state).holder = holder;
    if holder.is_null() {
        log_static(ANDROID_LOG_WARN, b"create_shell_holder failed\0");
        return;
    }

    // The original OS4 host sets locales before shell_holder_launch. Without
    // this, the Flutter side observes und/null and selects ar_eg.
    call_set_locales(holder);
    log_static(ANDROID_LOG_INFO, b"Snow locales set zh-CN\0");

    let launch_function = interface_entry(0x58);
    snow_shell_launch_call(
        launch_function,
        holder,
        ENTRYPOINT.as_ptr(),
        ENTRYPOINT.len(),
        (*activity).asset_manager,
        (ptr::addr_of!(HYPER_CALLBACKS) as *const usize).add(13),
    );

    log_static(ANDROID_LOG_INFO, b"Snow router engine launched\0");
    if (*state).resumed != 0 {
        send_lifecycle(LIFECYCLE_RESUMED);
    }
    send_on_ready();
    if !(*state).window.is_null() {
        call_surface_create(holder, (*state).window);
        update_window_size(state);
    }
}

unsafe extern "C" fn on_resume(_: *mut ANativeActivity) {
    let state = state_ptr();
    (*state).resumed = 1;
    initialize_engine();
    send_lifecycle(LIFECYCLE_RESUMED);
    send_on_ready();
}

unsafe extern "C" fn on_pause(_: *mut ANativeActivity) {
    (*state_ptr()).resumed = 0;
    send_lifecycle(LIFECYCLE_INACTIVE);
}

unsafe extern "C" fn on_stop(_: *mut ANativeActivity) {
    send_lifecycle(LIFECYCLE_PAUSED);
}

unsafe extern "C" fn on_destroy(_: *mut ANativeActivity) {
    let state = state_ptr();
    send_lifecycle(LIFECYCLE_DETACHED);
    if !(*state).input_queue.is_null() {
        AInputQueue_detachLooper((*state).input_queue);
    }
    if !(*state).input_looper.is_null() {
        ALooper_release((*state).input_looper);
    }
    (*state).activity = ptr::null_mut();
    (*state).window = ptr::null_mut();
    (*state).input_queue = ptr::null_mut();
    (*state).input_looper = ptr::null_mut();
    (*state).resumed = 0;
    // The HyperOS engine destroys AndroidNativeWindow asynchronously. On this
    // legacy host, immediately destroying holder/runtime after the Java window
    // is removed dereferences the expired ANativeWindow and SIGBUSes. Retain the
    // engine until process exit; normal force-stop still performs kernel cleanup.
    log_static(ANDROID_LOG_INFO, b"Snow retained engine until process exit\0");
}

unsafe extern "C" fn on_window_focus_changed(_: *mut ANativeActivity, focused: c_int) {
    let state = state_ptr();
    if (*state).resumed != 0 {
        if focused != 0 {
            send_lifecycle(LIFECYCLE_RESUMED);
        } else {
            send_lifecycle(LIFECYCLE_INACTIVE);
        }
    }
}
unsafe extern "C" fn input_queue_callback(
    _: c_int,
    _: c_int,
    data: *mut c_void,
) -> c_int {
    let state = if data.is_null() {
        state_ptr()
    } else {
        data as *mut RouterState
    };
    let queue = (*state).input_queue;
    if queue.is_null() {
        return 1;
    }
    loop {
        let mut event: *mut AInputEvent = ptr::null_mut();
        if AInputQueue_getEvent(queue, ptr::addr_of_mut!(event)) < 0 || event.is_null() {
            break;
        }
        if AInputQueue_preDispatchEvent(queue, event) != 0 {
            continue;
        }
        // v17 establishes a real native input consumer so InputDispatcher no
        // longer times out. PointerDataPacket translation is the next layer;
        // until then events are explicitly acknowledged instead of abandoned.
        AInputQueue_finishEvent(queue, event, 1);
    }
    1
}

unsafe extern "C" fn on_input_queue_created(
    _: *mut ANativeActivity,
    queue: *mut c_void,
) {
    let state = state_ptr();
    let queue = queue as *mut AInputQueue;
    if queue.is_null() {
        return;
    }
    if !(*state).input_queue.is_null() {
        AInputQueue_detachLooper((*state).input_queue);
    }
    if !(*state).input_looper.is_null() {
        ALooper_release((*state).input_looper);
    }
    let mut looper = ALooper_forThread();
    if looper.is_null() {
        looper = ALooper_prepare(1);
    }
    if looper.is_null() {
        log_static(ANDROID_LOG_WARN, b"Snow input looper unavailable\0");
        return;
    }
    ALooper_acquire(looper);
    (*state).input_queue = queue;
    (*state).input_looper = looper;
    AInputQueue_attachLooper(
        queue,
        looper,
        0,
        Some(input_queue_callback),
        state as *mut c_void,
    );
    log_static(ANDROID_LOG_INFO, b"Snow input queue attached\0");
}

unsafe extern "C" fn on_input_queue_destroyed(
    _: *mut ANativeActivity,
    queue: *mut c_void,
) {
    let state = state_ptr();
    let queue = queue as *mut AInputQueue;
    if !queue.is_null() {
        AInputQueue_detachLooper(queue);
    }
    if !(*state).input_looper.is_null() {
        ALooper_release((*state).input_looper);
    }
    (*state).input_queue = ptr::null_mut();
    (*state).input_looper = ptr::null_mut();
    log_static(ANDROID_LOG_INFO, b"Snow input queue detached\0");
}

unsafe extern "C" fn on_native_window_created(
    _: *mut ANativeActivity,
    window: *mut ANativeWindow,
) {
    let state = state_ptr();
    // initialize_engine() binds the first window itself. Only an already
    // initialized holder needs an explicit rebind for a recreated window.
    let holder_was_ready = !(*state).holder.is_null();
    (*state).window = window;
    initialize_engine();
    if holder_was_ready && !(*state).holder.is_null() && !window.is_null() {
        call_surface_create((*state).holder, window);
        update_window_size(state);
    } else if !holder_was_ready {
        log_static(ANDROID_LOG_INFO, b"Snow skipped duplicate initial surface bind\0");
    }
    if (*state).resumed != 0 {
        send_lifecycle(LIFECYCLE_RESUMED);
    }
    send_on_ready();
}

unsafe extern "C" fn on_native_window_resized(
    _: *mut ANativeActivity,
    window: *mut ANativeWindow,
) {
    let state = state_ptr();
    (*state).window = window;
    update_window_size(state);
}

unsafe extern "C" fn on_native_window_destroyed(
    _: *mut ANativeActivity,
    _: *mut ANativeWindow,
) {
    let state = state_ptr();
    // Android 14 may destroy/recreate the NativeActivity surface while the
    // display is asleep. Do not synchronously notify Flutter here: the engine
    // can still have raster/IO work queued against the old window. The next
    // created-window callback rebinds the holder safely.
    (*state).window = ptr::null_mut();
    log_static(ANDROID_LOG_INFO, b"Snow deferred surface destroy during rebind\0");
}

#[no_mangle]
pub unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut ANativeActivity,
    _: *mut c_void,
    _: usize,
) {
    if activity.is_null() {
        return;
    }
    let state = state_ptr();
    ptr::write(state, RouterState::empty());
    (*state).activity = activity;
    (*activity).instance = state as *mut c_void;

    let callbacks = (*activity).callbacks;
    if !callbacks.is_null() {
        (*callbacks).on_resume = Some(on_resume);
        (*callbacks).on_pause = Some(on_pause);
        (*callbacks).on_stop = Some(on_stop);
        (*callbacks).on_destroy = Some(on_destroy);
        (*callbacks).on_window_focus_changed = Some(on_window_focus_changed);
        (*callbacks).on_native_window_created = Some(on_native_window_created);
        (*callbacks).on_native_window_resized = Some(on_native_window_resized);
        (*callbacks).on_native_window_destroyed = Some(on_native_window_destroyed);
        (*callbacks).on_input_queue_created = Some(on_input_queue_created);
        (*callbacks).on_input_queue_destroyed = Some(on_input_queue_destroyed);
    }

    log_static(ANDROID_LOG_INFO, b"SnowWeatherRouter onCreate\0");
    initialize_engine();
}
