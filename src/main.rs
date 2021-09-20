#![allow(unused)]

// плотно упакованная структура
//#[repr(packed(1))]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }
}

const LEN: usize = 5;

struct ArrayOfPoints {
    z: [Point; LEN],
}

//
use std::mem;

fn anal_slice(slice: &[i32]) {
    println!("first: {}", slice[0]);
    println!("len: {}", slice.len());
}

// выровненная структура
#[repr(C, align(128))]
struct Packed {
    f: i8,
    s: i8,
    // поле смещенное на сколько?
    _pad: [u8; 3],
    t: i64
}

type Instruction = u32;

struct Foo<T> {
    data: T,
}

fn generic<A, B, C, D>(x: A) {
    //let f = Foo<i8>();
}

use std::fmt::Debug;
//fn foo<T>(x: T): T where T: Debug {
//}

//use Packed as Aligned;
//type Aligned = Packed;
struct PointTuple(f32, f32);
//let p = Point
struct Cookie;
fn main() {
    // безымянная структура??
    Point { x: 10, y: 10 };
    let p = Point::new();
    println!("size of Point is {}", mem::size_of_val(&p));
/*
 *
 *    let tpl = PointTuple(1., 1.);
 *
 *    let c = [Cookie, Cookie {}, Cookie, Cookie {}];
 *
 *    println!("Hello, world!");
 */
}
