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

/*
Создать указатель на фунцию по заданному адресу.
Вызвать фунцию.
 */

fn main() {

    unsafe {
        let addr: *mut c_void = ptr::null_mut();
        let size = 1024 * 1024 * 1024 * 33; // количество выделенных байт
        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        println!("errno {}", errno::errno());
        println!("addr {:?}", addr);
        let p: *mut i8 = mmap(addr, size, prot, MAP_PRIVATE | MAP_ANONYMOUS, 0, 0) as *mut i8;
        println!("errno {}", errno::errno());
        // хочу делать вот так
        //*p[1] = 0;
        //let slice8 = slice::from_raw_parts_mut(p, size);
        let slice64 = slice::from_raw_parts_mut(p as *mut i64, size / (64 / 8));
        println!("mmap returned {:?}", p);
        //let j = 1;
        //for i in 1 .. size {
        for i in 1 .. size - 8 {
            //slice[i] = 1;
            slice64[i] = 1;
            slice64[i + 1] = 1;
            slice64[i + 2] = 1;
            slice64[i + 3] = 1;
            slice64[i + 4] = 1;
            slice64[i + 5] = 1;
            slice64[i + 6] = 1;
            slice64[i + 7] = 1;
        }
        println!("v {}", slice64[1]);
        // Как обратиться к памяти выделенной на указатель p по индексу?
    }

    // wait for input
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("");

}
