#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn rustapi_test() {
    let i = 5678 as *mut i32;
    unsafe {
        i.write(456);
    }
}
