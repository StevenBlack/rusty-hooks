// Experimenting with hooks in Rust

use std::option::Option;

#[derive(Debug, Clone)]
pub struct Hook<'a> {
    pub name: String,
    pub description: String,
    pub hook: Option<Box<&'a Hook<'a>>>,
}

pub trait Hookable<T> {
    fn preprocess(&self, t: T) -> bool;
    fn process(&self, t: T) -> T;
    fn execute(&self, t: T) -> T;
    fn postprocess(&self, t: T) -> T;
    fn sethook(&self, h: Hook);
}


impl Hook<'_> {
    pub fn process(&self) {
        println!("{}: {}", self.name, self.description);
        match &self.hook {
            Some(h) => {
                h.process();
            }
            None => {
                println!("{}", "🏆");
            }
        }
    }

    // pub fn sethook(&mut self, hook: Option<Box<Hook>>) {
    //     match &self.hook {
    //         Some(h) => {
    //             h.sethook(hook);
    //         }
    //         None => {
    //             self.hook = None;
    //         }
    //     }
    // }
}


fn main() {
    let mut h1 = Hook {
        name: "Hook1".to_string(),
        description: "First hook".to_string(),
        hook: None,
    };
    let h2 = Hook {
        name: "Hook2".to_string(),
        description: "Second hook".to_string(),
        hook: None,
    };
    h1.hook = Some(Box::new(&h2));
    h1.process();
}
