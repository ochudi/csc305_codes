#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::fmt::{Arguments, Write};
mod writer;
use writer::FrameBufferWriter;

use bootloader_api::{
    config::Mapping,
    info::{MemoryRegion, MemoryRegionKind},
};
use x86_64::instructions::hlt;

extern crate alloc;

use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//my_entry_point can be any name.
//optionally pass a custom config
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

    /*
    //failed experiment... left here for review
    lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: FrameBufferWriter = FrameBufferWriter::default();
    }
    FRAME_BUFFER_WRITER.init(buffer, frame_buffer_info);
    */

    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    /*let mut write_str = |s:&str| {
    frame_buffer_writer
    .write_str(s)
    .unwrap() ;
    };*/

    //use core::fmt::Write;
    writeln!(
        frame_buffer_writer,
        "Testing testing {} and {}",
        1,
        4.0 / 2.0
    )
    .unwrap();

    //let memory_regions_count = boot_info.memory_regions.iter().size_hint();
    //println!("{}", memory_regions_count.0);

    //Let's get the usable memory
    let last_memory_region = boot_info.memory_regions.last().unwrap();
    //println!("{:X}", last_memory_region.end);

    //get the first bootload memory
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
    //println!("{:X} {:X} {:?}", boot_loader_memory_region.start, boot_loader_memory_region.end, boot_loader_memory_region.kind);

    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    //let heap_start = 0x3E000 + physical_memory_offset;
    //let heap_size = 0x7FC2000;
    let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    //println!("{:X} {:X}", heap_start as usize, heap_size as usize);

    unsafe {
        ALLOCATOR.init(heap_start as usize, heap_size as usize);
    }

    use alloc::boxed::Box;

    let x = Box::new(33);

    writeln!(frame_buffer_writer, "Value in X is {}", x).unwrap();

    println!("Hello, world!");
    println!("The answer is {}", 42);

    //Let's examine our memory
    //Go through memory regions passed and add usable ones to our global allocator
    /*let mut counter = 0 as u8;
    for memory_region in boot_info.memory_regions.iter() {
    counter += 1;
    frame_buffer_writer
    .write_fmt(format_args!("{}. ", counter)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
    .unwrap();
    //print!("{}. ", counter);
    frame_buffer_writer
    .write_fmt(format_args!("{:X} ", memory_region.start)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
    .unwrap();
    //print!("{:X}. ", memory_region.start);
    frame_buffer_writer
    .write_fmt(format_args!("{:X}, ", memory_region.end))
    .unwrap();
    //print!("{:X}. ", memory_region.end);
    frame_buffer_writer
    .write_fmt(format_args!(
    "size = {:X}, ",
    memory_region.end - memory_region.start
    ))
    .unwrap();
    //print!("size = {:X}, ", memory_region.end - memory_region.start);
    match memory_region.kind {
    MemoryRegionKind::Usable => write!(frame_buffer_writer, "Usable; ").unwrap(),
    MemoryRegionKind::Bootloader => write!(frame_buffer_writer, "Bootload;").unwrap(),
    MemoryRegionKind::UnknownUefi(_) => {
    write!(frame_buffer_writer, "UnknownUefi;").unwrap();
    }
    MemoryRegionKind::UnknownBios(_) => {
    write!(frame_buffer_writer, "UnknownBios;").unwrap();
    }
    _ => write!(frame_buffer_writer, "Unknown;").unwrap(),
    }
    }*/

    let mut _write_fmt = |args: Arguments| {
        frame_buffer_writer.write_fmt(args).unwrap();
    };

    //write_str("hello world\nTesting another line\n");

    //print(write_str, "testing");

    //let test_string = "Test string";
    //print_fmt(write_fmt, format_args!("{}", test_string));
    init_idt();
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}

/*pub fn print(mut write_str: impl FnMut(&str), s: &str){
 write_str(s);
}*/

/*pub fn print_fmt(mut write_fmt: impl FnMut(Arguments), args:Arguments){
 write_fmt(args);
}*/

//We need to handle interrupts
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

/*pub fn init_idt() {
 IDT.load();
}*/

pub fn init_idt() {
    //init_idt();
    IDT.load();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
