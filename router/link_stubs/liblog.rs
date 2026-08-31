#![no_std]
#[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
#[no_mangle] pub extern "C" fn __android_log_write(_: i32, _: *const i8, _: *const i8) -> i32 { 0 }
