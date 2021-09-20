#![allow(unused)]

// плотно упакованная структура?
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

enum Event {
    None,
    One,
    Two,
    Three,
    Point { x: i32, y : i32},
}

//type Callback = 
fn check_event(event: Event, callback: fn(i8) -> i8) {
    match event {
        Event::None => {
            println!("callback returned {}", callback(10));
        },
        Event::Point { x, y } => {

        },
        _ => {
        }
    }
}

fn main() {
}
