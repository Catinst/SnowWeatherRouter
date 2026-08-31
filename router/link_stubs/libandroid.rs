#![no_std]
#[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
#[no_mangle] pub extern "C" fn ANativeWindow_getWidth(_: *mut core::ffi::c_void) -> i32 { 0 }
#[no_mangle] pub extern "C" fn ANativeWindow_getHeight(_: *mut core::ffi::c_void) -> i32 { 0 }
