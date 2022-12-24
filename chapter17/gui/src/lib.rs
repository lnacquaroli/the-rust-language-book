pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vector of trait object Box<dyn Draw>: any type inside Box that implements Draw
    pub components: Vec<Box<dyn Draw>>,
}

// This works differently from defining a struct that uses a generic type parameter
// with trait bounds. A generic type parameter can only be substituted with one
// concrete type at a time. Trait objects allow for multiple concrete types
// to fill in for the trait object at runtime: One Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        // code to actually draw a text field (can be different than that of button)
    }
}
