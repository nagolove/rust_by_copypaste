#![allow(unused)]

use core::ffi::c_void;
use errno;
use std::ptr;
use std::slice;

//#[warn(dead_code)]
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
const MAP_PRIVATE: i32 = 0x02; /* Changes are private.  */

/*
Создать указатель на фунцию по заданному адресу.
Вызвать фунцию.
 */

static mut COUNTER: u128 = 0;

fn foo(n: u64) -> u64 {
    if n > 0 {
        unsafe {
            COUNTER += 1;
        }
        foo(n - 1);
    }
    0
}

use std::thread;

const STACK_SIZE: usize = 2 * 1024 * 1024 * 1024; // стек пару гигабайт

fn run() {
    println!("run");
    foo(40_500_000);
    println!("end")
}

fn test_recurse() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();
    child.join().unwrap();
}

fn memory_alloc() {
    unsafe {
        let addr: *mut c_void = ptr::null_mut();
        //let size = 1024 * 1024 * 1024 * 33; // количество выделенных байт
        let size = 1024 * 1024 * 1024 * 2; // количество выделенных байт
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
        for i in 0..size - 10 - 1 {
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
        //println!("v {}", slice64[1]);
        // Как обратиться к памяти выделенной на указатель p по индексу?
    }
}

fn main() {
    //test_recurse();
    memory_alloc();

    // wait for input
    //let mut guess = String::new();
    //std::io::stdin().read_line(&mut guess).expect("");

    //foo(300_000);
    //foo(10_000);
    println!("hello");
}
