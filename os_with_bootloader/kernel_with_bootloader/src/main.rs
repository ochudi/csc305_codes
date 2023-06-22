#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

mod writer;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut frame_buffer_writer = {
                let boot_info = unsafe { &mut *$crate::BOOT_INFO };
                let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
                let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
                FrameBufferWriter::new(buffer, frame_buffer_info)
            };
            frame_buffer_writer.set_write_position(150, 150);
            write!(frame_buffer_writer, $($arg)*).unwrap();
        }
    };
}

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

extern crate alloc;
use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

static mut BOOT_INFO: *mut bootloader_api::BootInfo = core::ptr::null_mut();

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    unsafe {
        BOOT_INFO = boot_info;
    }

    let frame_buffer_info = unsafe {
        BOOT_INFO
            .as_mut()
            .unwrap()
            .framebuffer
            .as_mut()
            .unwrap()
            .info()
    };
    let buffer = unsafe {
        BOOT_INFO
            .as_mut()
            .unwrap()
            .framebuffer
            .as_mut()
            .unwrap()
            .buffer_mut()
    };

    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    frame_buffer_writer.set_write_position(150, 150);

    /*
    frame_buffer_writer._set_text_color([0, 0, 255, 0]);
    write!(frame_buffer_writer, "My name is Chudi").unwrap();
    frame_buffer_writer.set_write_position(300, 300);
    frame_buffer_writer._set_text_color([0, 255, 255, 0]);
    write!(frame_buffer_writer, "Testing testing {} and {}", 1, 4.0 / 2.0).unwrap();
    */

    print!("Chukwudis-MacBook-Pro:kernel_with_bootloader ochudi$");

    loop {
        hlt();
    }
}
