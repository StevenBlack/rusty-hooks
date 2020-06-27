// Experimenting with hooks in Rust

use std::option::Option;

#[derive(Debug)]
pub struct Hook<'a> {
    pub name: String,
    pub description: String,
    pub hook: Option<&'a mut Hook<'a>>,
}

pub trait Hooking<'a> {
    type Thing;
    fn preprocess(&self, t: &'a mut Self::Thing) -> bool;
    fn process(&mut self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    fn execute(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    fn postprocess(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    fn sethook(&mut self, t: &'a mut Self);
}

impl<'a> Hooking<'a> for Hook<'a> {
    type Thing = String;
    fn preprocess(&self, _t: &'a mut Self::Thing) -> bool {
        true
    }

    fn process(&mut self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        if self.preprocess(t) {
            self.execute(t);
        }
        match self.hook {
            Some(ref mut h) => {
                h.process(t);
            }
            None => {
                // hook chain ends here, no-op.
            }
        }
        self.postprocess(t);
        t
    }

    fn execute(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        t
    }
    fn postprocess(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        t
    }

    fn sethook(&mut self, hook_passed: &'a mut Self) {
        match self.hook {
            Some(ref mut h) => h.sethook(hook_passed),
            None => {
                self.hook = Some(hook_passed);
            }
        };
        &self;
    }
}

impl Hook<'_> {
    pub fn preprocess<'a>(&self, _t: &'a mut String) -> bool {
        println!("{}: {}", self.name, "preprocess");
        true
    }

    pub fn execute<'a>(&self, t: &'a mut String) -> &'a mut String {
        println!("{}: {}", self.name, "execute");
        t
    }

    pub fn postprocess<'a>(&self, t: &'a mut String) -> &'a mut String {
        println!("{}: {}", self.name, "postprocess");
        t
    }
}

fn main() {
    let mut x = "xyz".to_string();
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
    h1.sethook(&mut h2);
    h1.process(&mut x);
}
