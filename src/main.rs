//#![allow(unused)]

use errno;
use core::ffi::c_void;
use std::ptr;
use std::slice;

extern "C" {
    fn mmap(
        addr: *mut c_void,
        size: usize,
        prot: i32,
        flags: i32,
        fildes: i32,
        off: i32,
    ) -> *mut c_void;
    //fn munmap();
}

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const PROT_EXEC: i32 = 0x4;
const MAP_ANONYMOUS: i32 = 0x20; /* Don't use a file.  */
const MAP_PRIVATE: i32 = 0x02;  /* Changes are private.  */

fn main() {

    unsafe {
        let addr: *mut c_void = ptr::null_mut();
        //let size = 1024 * 1024 * 1024 * 6;
        let size = 1024;
        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        //let prot = PROT_EXEC;
        println!("errno {}", errno::errno());
        println!("addr {:?}", addr);
        let p: *mut i8 = mmap(addr, size, prot, MAP_PRIVATE | MAP_ANONYMOUS, 0, 0) as *mut i8;
        println!("errno {}", errno::errno());
        // хочу делать вот так
        //*p[1] = 0;
        let slice = slice::from_raw_parts_mut(p, 10);
        println!("mmap returned {:?}", p);
        slice[1] = 1;
        println!("v {}", slice[1]);
        // Как обратиться к памяти выделенной на указатель p по индексу?
    }

    // wait for input
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("");

    //let s: &str = "123";
    //let ptr: *const u8 = s.as_ptr();

    //unsafe {
        //println!("{}", *ptr.offset(1) as char);
        //println!("{}", *ptr.offset(2) as char);
    //}

    //let mut num = 5;
    //println!("{}", num);
    //let r1 = &num as *const i32;
    ////unsafe { *r1 = 2; }
    //println!("{}", num);
    //let r2 = &mut num as *mut i32;
    //unsafe { *r2 = 3; }
    //println!("{}", num);

    /*
     *let address = 0x010395usize;
     *let r = address as *mut i32;
     *unsafe {
     *    *r = 10;
     *}
     */

    //let address = 0x01230usize;
    //let r = address as *mut i8;

    //let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //let slice: &mut [i8] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //slice[1] = 1;
    //println!("{}", slice[1]);
}
