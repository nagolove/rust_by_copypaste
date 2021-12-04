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

#[derive(Debug)]
enum Language {
    Russian,
    English,
    German,
    French,
    Chinese,
}

// Как передать ссылку на изменяемый кортеж?
fn print_coords(&(x, y): &(i64, i64)) {
    println!("Location: ({}, {})", x, y);
    //x = 1;
}

#[cfg(test)]
mod custom {
    use super::*;

    #[test]
    fn irrefurable_or_not_test() {
        // always match
        let x = 1;
        //if let x = x {
            //println!("{}", x);
        //}

        // not always match
        let x: Option<&str> = None;
        if let Some(x) = x {
            println!("{}", x);
        }
    }

    #[test]
    fn print_coords_test() {
        //let point = (-1, 0);
        let mut point = (-1, 0);
        print_coords(&point);
    }

    #[test]
    fn let_test() {
        let x: i32 = 5;
        //let (x, y, z) = (1, 2, 3);
        let (x, y, z, _) = (1, 2, 3, 4);
        let yy = y;
        let _ = z;
        let x = String::from("blah");
    }

    #[test]
    fn for_loop_test() {
        let v = vec!['a', 'c', 'd'];

        // Что делает enumerate() ?
        // tuple desctuction for index and value variables
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn while_let_test() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        // Проход по вектору с извлечением элементов
        while let Some(top) = stack.pop() {
            println!("{}", top)
        }
    }

    #[test]
    fn enum_test() {
        let language = Language::Russian;
        use Language::*;
        //use super::*;
        match language {
            //Language::German => println!("German"),
            German => println!("German"),
            Russian => println!("Русский"),
            //_ => println!("other"),
            // Что значит унаследоваться от Debug интерфейса?
            l => println!("other {:?}", l),
        }
    }

    #[test]
    fn if_let_test() {
        let authorization_status: Option<&str> = None;
        let is_admin = false;
        // Что значит второй параметр в Result<> ?
        let group_id: Result<u8, _> = "1111".parse();

        if let Some(status) = authorization_status {
            println!("Authorization status: {}", status);
        } else if is_admin {
            println!("Authorization status: admin");
        } else if let Ok(group_id) = group_id {
            if group_id > 10 {
                println!("Authorization status: privileged");
            } else {
                println!("Authorization status: basic");
            }
        }
    }

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
