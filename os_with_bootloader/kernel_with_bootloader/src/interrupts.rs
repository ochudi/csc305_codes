use crate::print;
use crate::println;
use crate::FRAME_BUFFER_WRITER;
use core::fmt::Write;
use pc_keyboard::KeyCode;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n Stack Frame:\n {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n Stack Frame:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!(
        "EXCEPTION: GENERAL PROTECTION\n Error Code: {:#?}\n Stack Frame:\n{:#?}",
        _error_code, stack_frame
    );
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!(
        "EXCEPTION: INVALID OPCODE\n Stack Frame:\n {:#?}",
        stack_frame
    );
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

fn init_pics() {
    unsafe { PICS.lock().initialize() };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, //offset 0 is reserved for timer
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
//Add a handler for Timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!("."); //You can uncomment this to see that timer interrupt is on.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

//Add a handler for keyboard
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                //DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::Unicode(character) => {
                    if character == '\u{0008}' {
                        // Check for backspace character
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().backspace();
                        }
                    } else if character == '\u{0009}' {
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().tab();
                        }
                    } else {
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    if key == KeyCode::ArrowUp {
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().arrow_up();
                        }
                    } else if key == KeyCode::ArrowDown {
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().arrow_down();
                        }
                    } else if key == KeyCode::ArrowRight {
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().arrow_right();
                        }
                    } else if key == KeyCode::ArrowLeft {
                        unsafe {
                            FRAME_BUFFER_WRITER.unwrap().as_mut().arrow_left();
                        }
                    }
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

//setup the IDT and make entries of all the handlers
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

//Below function to be called from init() at the bottom of this
//this module, to init IDT.
fn init_idt() {
    IDT.load();
}

//init all interrupts
pub fn init() {
    init_idt(); //IDT
    init_pics(); //PICS
    x86_64::instructions::interrupts::enable(); //enable hardware interrupts. Without handler for timer interrupt, which is on by default, there will be a double fault
}
