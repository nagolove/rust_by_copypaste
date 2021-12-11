struct CustomIter {
    v: Vec<String>,
    counter: usize,
}

impl CustomIter {
    fn new() -> Self {
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

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Hi");
    }
}

enum Optione<T> {
    Some(T),
    None,
}

// зачем нужен следущий аттрибут?
#[cfg(test)]
mod custom {
    use super::*;

    type Her = Option<i8>;

    fn plus_one(x: Her) -> Her {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    //use proc_macro;
    //use hello_macro::HelloMacro;
    //use hello_macro_derive::HelloMacro;

    //#[derive(HelloMacro)]
    //struct Pancake;

    // This macro is not support trailing comma in declaration list. Why?
    #[macro_export]
    // Что такое macro_rules ??
    macro_rules! MASSIV {
        ( $( $v:expr ),* ) => {
            {
                let mut temp = Vec::new();
                $(
                    temp.push($v);
                )*
                temp
            }
        }; // Does this comma is necessary? And why?
    }

    #[test]
    fn decl_macros_test() {
        fn s() -> i32 {
            //return 1;
            1
        };

        let q = {
            10
        };
        println!("q = {}", q);

        let v1 = MASSIV![1, 2, 2];
        let v2 = vec![vec![1, 2, 2], ];
        let v3: Vec<char> = vec!['a', ];
        let v4: Vec<&str> = vec!["a", ];
        //let v5 = MASSIV![vec![1, 2, 2], ];
        //let v6 = MASSIV![1, 2, 2, ];
        //let v5: Vec<&String> = vec!["a", ];
    }

    #[test]
    fn size_of_test() {
        println!("size_of<char> = {}", std::mem::size_of::<char>());
    }

    #[test]
    fn plus_one_test() {
        let one = Some(1);
        let two = plus_one(one);
        let none = plus_one(None);
    }

    #[test]
    fn option_test_2() {
        let some_number = Some(-1);
        let some_string = Some("A string?");
        let absent_value: Option<i64> = None;
        let x: i8 = -1;
        let y: Option<i8> = Some(4);

        let sum = x + y.unwrap();
        println!("sum = {}", sum);

        let sum = x + y.unwrap_or(-1);
        println!("sum = {}", sum);

        let sum = x + y.unwrap_or_else(|| -10);
        println!("sum = {}", sum);

        //let sum = x + y;
    }

    #[test]
    fn enum_with_data_test() {

        //let m: Message = Message::Quit;
        //let m: Message = Message::Move { x: 1, y: 1 };

        for m in Message::iter() {
            use Message::*;
            match m {
                Quit => println!("quit"),
                Move { x, y } => println!("data is ({}, {})", x, y),
                Write(text) => println!("write \"{}\"", text),
                ChangeColor(r, g, b) => println!("color is ({}, {}, {})", r, g, b),
                _ => println!("Any variant"),
            }
        }
    }

    #[test]
    fn enum_impl_functioin_test() {
        Message::some_function();
    }

    #[test]
    fn enum_test() {
        //m::Quit();
        //m.some_function();

        enum IpAddrKind {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        //use IpAddrKind::*;
        let localhost = IpAddrKind::V4(127, 0, 0, 1);
    }

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
    fn enum_match_test() {
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
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }

    #[test]
    fn panic_test() {
        if false {
            panic!("паника");
        }
    }

    #[test]
    fn option_test_1() {
        let mut o: Option<u32> = Some(10);
        o = None;
        //let Option<bool> o;
        fn sub(n: i64) -> Option<bool> {
            if (n < 0) {
                return Some(false);
            } else {
                return None;
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
