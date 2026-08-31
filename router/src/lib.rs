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
pub static SNOW_WEATHER_ROUTER_WATERMARK: [u8; b"SnowWeatherRouter|Snownight|v2\0".len()] =
    *b"SnowWeatherRouter|Snownight|v2\0";

#[no_mangle]
#[used]
#[link_section = ".rodata.snow"]
pub static SNOW_WEATHER_ROUTER_BUILD: [u8; b"Snow full-source HyperOS router v2\0".len()] =
    *b"Snow full-source HyperOS router v2\0";

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

const LIFECYCLE_RESUMED: &[u8] = b"AppLifecycleState.resumed";
const LIFECYCLE_INACTIVE: &[u8] = b"AppLifecycleState.inactive";
const LIFECYCLE_PAUSED: &[u8] = b"AppLifecycleState.paused";
const LIFECYCLE_DETACHED: &[u8] = b"AppLifecycleState.detached";
const ON_READY: &[u8] = b"{\"method\":\"onReady\",\"args\":null}";

const JSON_NULL: &[u8] = b"[null]";
const JSON_TRUE: &[u8] = b"[true]";
const JSON_FALSE: &[u8] = b"[false]";
const JSON_ZERO: &[u8] = b"[0]";
const JSON_FALSE_STRING: &[u8] = b"[\"false\"]";
const JSON_EMPTY_MAP: &[u8] = b"[{}]";
const JSON_EMPTY_LIST: &[u8] = b"[[]]";
const JSON_ZERO_CORNERS: &[u8] = b"[[0,0,0,0]]";
const JSON_GLASS_UNSUPPORTED: &[u8] =
    b"[{\"isSupportMaterial\":false,\"isSupportGlass\":false}]";
const JSON_SHARED_OPEN: &[u8] = b"[\"SnowWeatherPrefs\"]";
const JSON_DEVICE_FLAGSHIP: &[u8] =
    b"[{\"cpu_level\":3,\"gpu_level\":3,\"ram_level\":3}]";
const JSON_PACKAGE_INFO: &[u8] = b"[{\"versionName\":\"1\",\"versionCode\":180000231,\"lastUpdateTime\":0,\"applicationInfo\":{\"flags\":0,\"enabled\":true}}]";

const STANDARD_NULL: &[u8] = &[0x00, 0x00];
const STANDARD_TRUE: &[u8] = &[0x00, 0x01];
const STANDARD_FALSE: &[u8] = &[0x00, 0x02];
const STANDARD_EMPTY_MAP: &[u8] = &[0x00, 0x0d, 0x00];

const ARG_AOT: &[u8] = b"--aot-shared-library-name=libapp.so";
const ARG_ICU: &[u8] = b"--icu-symbol-prefix=_binary_icudtl_dat";
// Preserve the working Xiaomi bridge's observed 17-byte argument length.
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
struct RouterState {
    activity: *mut ANativeActivity,
    runtime: *mut c_void,
    holder: *mut c_void,
    window: *mut ANativeWindow,
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

static mut STATE: RouterState = RouterState::empty();
static mut HYPER_CALLBACKS: [usize; 16] = [0; 16];
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
            reply(state, reply_id, JSON_FALSE_STRING);
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
            reply(state, reply_id, JSON_SHARED_OPEN);
        } else {
            reply(state, reply_id, JSON_NULL);
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
            reply(state, reply_id, JSON_EMPTY_MAP);
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
    ptr::write(callbacks.add(2), hyper_noop as *const () as usize);
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
            len: 17,
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
    call_set_window_size((*state).holder, width, height);
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
    if !(*state).holder.is_null() {
        call_destroy_holder((*state).holder);
    }
    if !(*state).runtime.is_null() {
        call_runtime_destroy((*state).runtime);
    }
    ptr::write(state, RouterState::empty());
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

unsafe extern "C" fn on_native_window_created(
    _: *mut ANativeActivity,
    window: *mut ANativeWindow,
) {
    let state = state_ptr();
    (*state).window = window;
    initialize_engine();
    if !(*state).holder.is_null() && !window.is_null() {
        call_surface_create((*state).holder, window);
        update_window_size(state);
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
    if !(*state).holder.is_null() {
        call_surface_destroy((*state).holder);
    }
    (*state).window = ptr::null_mut();
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
    }

    log_static(ANDROID_LOG_INFO, b"SnowWeatherRouter onCreate\0");
    initialize_engine();
}
