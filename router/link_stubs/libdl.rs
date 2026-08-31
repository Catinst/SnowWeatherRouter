#![no_std]
#[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
#[no_mangle] pub extern "C" fn dlopen(_: *const i8, _: i32) -> *mut core::ffi::c_void { core::ptr::null_mut() }
