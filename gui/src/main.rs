#![no_std]
#![no_main]

use shelllib::framebuffer::Framebuffer;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _door(fb_addr: *mut u32, width: usize, height: usize, pitch: usize) -> ! {
    unsafe {
        let mut fb = Framebuffer::new(fb_addr, width, height, pitch);

        fb.clear(0x00112233);

        fb.draw_rect(100, 100, 200, 150, 0x00FF00FF);

    }
    loop {}
}