pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box - is smasrt pointer
    // what is dyn do?
    // dynamic dispatch
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

/*

// generic - implementation
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl <T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

*/

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
    }
}
