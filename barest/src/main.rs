#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

static HELLO: &[u8] = b"Hello World! This is just a quick illustration";
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


fn write_position(framebuffer: *mut u8, string: &[u8], x: usize, y: usize, colour: Colour) {
    if x > BUFFER_HEIGHT - 1 {
        //TODO: Implement Scroll
        panic!("Row should be from 0 to 24")
    }

    if y > BUFFER_WIDTH - 1 {
        //TODO: Implement Scroll
        panic!("Row should be from 0 to 24")
    }

    let initial_position = (x * BUFFER_WIDTH) + y;
    
    // let row = 80 * (x - 1);
    // let column = y + row;

    for (i, &byte) in string.iter().enumerate() {
        let count = (initial_position + i) as isize;

        unsafe {
            *framebuffer.offset(count * 2) = byte;
            *framebuffer.offset(count * 2 + 1) = colour as u8;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    write_position(framebuffer, HELLO, 0, 15, Colour::Blue);
    write_position(framebuffer, HELLO, 24, 15, Colour::White);
    loop {}
}
