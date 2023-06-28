#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupt;
mod writer;

extern crate alloc;

use crate::interrupt::init;
use bootloader_api::{
    config::Mapping,
    info::{MemoryRegion, MemoryRegionKind},
};
use core::fmt::{Arguments, Write};
use good_memory_allocator::SpinLockedAllocator;
use lazy_static::lazy_static;
use spin::Mutex;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

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
            frame_buffer_writer.set_write_position(frame_buffer_writer.x_pos, frame_buffer_writer.y_pos);
            write!(frame_buffer_writer, $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        {
            $crate::print!("{}\n", format_args!($($arg)*));
        }
    };
}

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

    writeln!(
        frame_buffer_writer,
        "Testing testing {} and {}",
        1,
        4.0 / 2.0
    )
    .unwrap();

    let last_memory_region = boot_info.memory_regions.last().unwrap();

    let mut boot_loader_memory_region = MemoryRegion::empty();

    for memory_region in boot_info.memory_regions.iter() {
        match memory_region.kind {
            MemoryRegionKind::Bootloader => {
                boot_loader_memory_region = *memory_region;
                break;
            }
            _ => continue,
        }
    }

    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    unsafe {
        ALLOCATOR.init(heap_start as usize, heap_size as usize);
    }

    use alloc::boxed::Box;

    let x = Box::new(33);

    writeln!(frame_buffer_writer, "Value in X is {}", x).unwrap();

    let mut _write_fmt = |args: Arguments| {
        frame_buffer_writer.write_fmt(args).unwrap();
    };

    init();

    println!("Did not crash after breakpoint exception");

    loop {
        hlt();
    }
}

lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> = {
        let boot_info = unsafe { &mut *BOOT_INFO };
        let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
        let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
        Mutex::new(FrameBufferWriter::new(buffer, frame_buffer_info))
    };
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
