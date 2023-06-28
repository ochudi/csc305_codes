#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

use crate::interrupts::init;
use bootloader_api::config::Mapping;
use core::{fmt::Write, ptr::NonNull};
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;
mod interrupts;
mod writer;

pub static mut FRAMEBUFFER_BUFFER: Option<NonNull<[u8]>> = None;
pub static mut FRAMEBUFFER_WRITER: Option<NonNull<FrameBufferWriter>> = None;
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

#[macro_export]
macro_rules! print {
    ($($stmt:tt)*) => {
        {
            let frame_buffer_writer = unsafe { FRAMEBUFFER_WRITER.as_mut().unwrap().as_mut() };

            write!(frame_buffer_writer, $($stmt)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($stmt:tt)*) => {
        {
            let frame_buffer_writer = unsafe { FRAMEBUFFER_WRITER.as_mut().unwrap().as_mut() };

            writeln!(frame_buffer_writer, $($stmt)*).unwrap();
        }
    };
}

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    unsafe {
        FRAMEBUFFER_BUFFER = Some(NonNull::new_unchecked(
            boot_info.framebuffer.as_mut().unwrap().buffer_mut(),
        ));
        FRAMEBUFFER_WRITER = Some(NonNull::new_unchecked(&mut FrameBufferWriter::new(
            FRAMEBUFFER_BUFFER.as_mut().unwrap().as_mut(),
            boot_info.framebuffer.as_mut().unwrap().info(),
        )));
    }

    let frame_buffer_writer = unsafe { FRAMEBUFFER_WRITER.as_mut().unwrap().as_mut() };

    write!(
        frame_buffer_writer,
        "Testing testing {} and {}",
        1,
        4.0 / 2.0
    )
    .unwrap();

    frame_buffer_writer.set_pos(200, 300);

    print!("Changing the position! ");
    print!("This sentence should follow... to test the print!() macro");
    println!("This should be printed in the next line, to test the println!() macro.");

    init();

    loop {
        hlt();
    }
}
