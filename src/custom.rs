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

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

}

pub fn setup() {
    // do nothing
}
