//#![allow(unused)]

use core::ffi::c_void;
use std::ptr;
use std::slice;

extern "C" {
    fn mmap(addr: *mut c_void, size: usize, prot: i32, flags: i32, fildes: i32, off: i32) -> *mut c_void;
    //fn munmap();
}

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const PROT_EXEC: i32 = 0x4;

fn main() {
    unsafe {
        let addr: *mut c_void = ptr::null_mut();
        let size = 1024 * 1024 * 1024 * 6;
        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        let p: *mut i8 = mmap(addr, size, prot, 0, 0, 0) as *mut i8;
        //*p[1] = 0;
        let slice = slice::from_raw_parts_mut(p as *mut i8, 1);
        //println!("mmap returned {}", p.as_ptr());
        // Как обратиться к памяти выделенной на указатель p по индексу?
    }

    // wait for input
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("");
}
