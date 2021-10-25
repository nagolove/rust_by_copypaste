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
        let size = 1024 * 1024 * 1024 * 33; // количество выделенных байт
                                            //let size = 1024 * 1024 * 1024 * 2; // количество выделенных байт
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

fn wr2file(data: Vec<u8>) {

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("out.txt").unwrap();
    //write!(&mut file, "hello\n");
//file.write_all(&buf.to_vec());
//file.write_all(&output.stdout);
//let s = std::str::from_utf8(&output.stdout).expect("конвертация в строку сломалась");
//file.write_all(&output.stdout);

//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
//file.write_all(&output.stdout);
}

use std::process::Command;
fn print_command_stdout(cmd: &str) {
    let output = Command::new(cmd)
        .arg("")
        .arg("")
        .output()
        .expect("failed");
    //let buf = output.stdout;
    //let s = std::str::from_utf8(&output.stdout).expect("конвертация в строку сломалась");
    let s = std::str::from_utf8(&output.stdout).expect("конвертация в строку сломалась");
    print!("stdout {:?}", s);
    let s = std::str::from_utf8(&output.stderr).expect("конвертация в строку сломалась");
    print!("stderr {:?}", s);
    //let hello = String::from_utf8_lossy(&output.stdout);
}

fn run_fasm() {
    /*
     *let output = if cfg!(target_os = "windows") {
     *    //Command::new("cmd").arg
     *} else {
     *    Command::new("fasm")
     *        .arg("")
     *        .arg("").output().expect("failed");
     *};
     */
    //let b = output.stdout;
    //let sout = std::process::ChildStdout;
    println!("------------------");
    print_command_stdout("fasm");
    println!("------------------");
    print_command_stdout("lsblk");
    println!("------------------");
    print_command_stdout("ls");
    println!("------------------");
}

#[test]
fn test_divide() {
    match divide(5., 3.) {
        Some(..) => println!("Some"),
        None => println!("None"),
        _ => println!("other") ,
    }
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[test]
fn test_check_optional() {
    let optional = None;
    check_optional(optional);

    // В чем разница между let и let mut?
    let optional = Some(Box::new(10));
    check_optional(optional);
}

fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(p) => println!("value is {}", p),
        None => println!("value is none"),
    }
}

fn external_command(cmd: &str) {
    use std::process::{Command, Stdio};

    let child = Command::new(cmd)
        .arg("1.asm")
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    let s = std::str::from_utf8(&output.stdout).expect("конвертация в строку сломалась");
    println!("{}", s);
}

fn main() {
    external_command("fasm");
    //test_external_command("ls");
    //test_external_command("lsblk");

    //test_recurse();
    //memory_alloc();
    //run_fasm();

    // wait for input
    //let mut guess = String::new();
    //std::io::stdin().read_line(&mut guess).expect("");

    //foo(300_000);
    //foo(10_000);
    //println!("hello");
}

// Heapless vector type
struct ArrayVec<T, const N: usize> {
    values: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    fn try_push(&mut self, t: T) -> Result<(), T> {
        if self.len == N {
            return Err(t);
        }
        self.values[self.len] = Some(t);
        self.len += 1;
        return Ok(());
    }
}

#[test]
fn iterate_over_an_static_array_rev() {
    let a = [1, -1, 1, -1, 0, 1, -1];
    for i in a.iter().rev() {
        println!("{}", i);
    }
}

#[test]
fn iterate_over_an_static_array() {
    let a = [1, -1, 1, -1, 0, 1, -1];
    for i in a {
        println!("{}", i);
    }
}

#[test]
fn iterate_over_vector_rev() {
    let v = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    for i in v.iter().rev() {
        println!("{}", i);
    }
}

#[test]
fn iterate_over_vector() {
    let v = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    for i in v {
        println!("{}", i);
    }
}

#[test]
fn iterate_over_hashmap() {
    use std::collections::*;
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert("name", "den");
    hm.insert("age", "30");
    for (key, value) in hm {
        println!("{} => {}", key, value);
    }
}

#[test]
fn iterate_over_hashmap_iter() {
    //iter: Iterator;
    use std::collections::*;
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert("name", "den");
    hm.insert("age", "30");
    for (key, value) in hm {
        println!("{} => {}", key, value);
    }
}
