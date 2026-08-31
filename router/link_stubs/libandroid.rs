#![no_std]
#[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
#[no_mangle] pub extern "C" fn ANativeWindow_getWidth(_: *mut core::ffi::c_void) -> i32 { 0 }
#[no_mangle] pub extern "C" fn ANativeWindow_getHeight(_: *mut core::ffi::c_void) -> i32 { 0 }
#[no_mangle] pub extern "C" fn ALooper_forThread() -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[no_mangle] pub extern "C" fn ALooper_prepare(_: i32) -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[no_mangle] pub extern "C" fn ALooper_acquire(_: *mut core::ffi::c_void) {}
#[no_mangle] pub extern "C" fn ALooper_release(_: *mut core::ffi::c_void) {}
#[no_mangle] pub extern "C" fn AInputQueue_attachLooper(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: i32, _: *const core::ffi::c_void, _: *mut core::ffi::c_void) {}
#[no_mangle] pub extern "C" fn AInputQueue_detachLooper(_: *mut core::ffi::c_void) {}
#[no_mangle] pub extern "C" fn AInputQueue_getEvent(_: *mut core::ffi::c_void, _: *mut *mut core::ffi::c_void) -> i32 { -1 }
#[no_mangle] pub extern "C" fn AInputQueue_preDispatchEvent(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 { 0 }
#[no_mangle] pub extern "C" fn AInputQueue_finishEvent(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: i32) {}
