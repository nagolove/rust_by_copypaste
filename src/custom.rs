struct CustomIter {
    v: Vec<String>,
    counter: usize,
}
impl CustomIter {
    fn new()-> Self {
        CustomIter {
            v: Vec::new(),
            counter: 0,
        }
    }

    fn add(&mut self, val: String) {
        self.v.push(val)
    }
}

impl Iterator for CustomIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v.get(self.counter) {
            Some(s) => {
                self.counter += 1;
                Some(s.to_owned())
            }
            None => None,
        }
    }
}

use std::fmt;

// Tuple struct
struct Wrapper(Vec<String>);

// Реализация вывода через макрос форматирования.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 - обращение к первому элементу кортежа
        // что возвращает макром write! ?
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod custom {
    use super::*;

    #[test]
    fn wrapper_test() {
        let w = Wrapper(
            vec![String::from("hello"), String::from("world")]
            );
        println!("w = {}", w);
    }

    #[test]
    fn panic_test() {
        if false {
            panic!("паника");
        }
    }
    
    #[test]
    fn option_test() {
        let mut o: Option<u32> = Some(10);
        o = None;
        //let Option<bool> o;
        fn sub(n: i64) -> Option<bool> {
            if (n < 0) {
                return Some(false)
            } else {
                return None
            }
        }
    }

    #[test]
    fn dynamically_sized_type_test() {
        // & - borrowed version of str
        let s1: &str = "hi1";
        use std::mem;
        //println!("size_of({}) = {}", size_of(s1), s1);
    }
}

pub fn setup() {
    // do nothing
}

// type alias
type Kilometers = u32;

// Never return type(wtf?)
fn foo() -> ! {
    loop {
        //return continue;
        return continue;
    }
}
