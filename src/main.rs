#![allow(unused)]

// плотно упакованная структура?
//#[repr(packed(1))]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

enum Event {
    None,
    One,
    Two,
    Three,
    Point { x: i32, y: i32 },
}

impl Event {
    fn new() -> Event {
        Event::None
    }
}

//type Callback =
fn check_event(event: Event, callback: fn(i8) -> i8) {
    match event {
        Event::None => {
            println!("callback returned {}", callback(10));
        }
        Event::Point { x, y } => {}
        _ => {
            println!("неизвестный");
        }
    }
}

use core::ffi::c_void;
use std::ptr;
use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
    fn mmap(addr: *mut c_void, size: usize, prot: i32, flags: i32, fildes: i32, off: i32) -> *mut c_void;
    //fn munmap();
}

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const PROT_EXEC: i32 = 0x4;

fn main() {
    unsafe {
        println!("abs(-10) {}", abs(-10));
        let addr: *mut c_void = ptr::null_mut();
        let size = 1024 * 1024 * 1024 * 6;
        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        let p: *mut i8 = mmap(addr, size, prot, 0, 0, 0) as *mut i8;
        //*p[1] = 0;
        let slice = unsafe {
            slice::from_raw_parts_mut(p as *mut i8, 1);
        };
        //println!("mmap returned {}", p.as_ptr());
        // Как обратиться к памяти выделенной на указатель p?
    }
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("");
}
