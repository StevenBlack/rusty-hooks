// Experimenting with hooks in Rust

use std::option::Option;

pub struct Hook<'a> {
    pub name: String,
    pub hook: Option<Box<&'a Hook<'a>>>,
}

impl Hook<'_> {
    pub fn sethook(&mut self, hook: Option<Box<Hook>>) {
        match &self.hook {
            Some(h) => {
                h.sethook(hook);
            },
            None => {
                self.hook = None;
            },
        }
    }
}


fn main() {
    println!("Hello, world!");
}
