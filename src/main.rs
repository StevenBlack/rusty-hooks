// Experimenting with hooks in Rust

use std::option::Option;

#[derive(Debug, Clone)]
pub struct Hook<'a> {
    pub name: String,
    pub description: String,
    pub hook: Option<Box<&'a Hook<'a>>>,
}

pub trait Hooking {
    type Thing;
    fn preprocess<'a>(&self, t: &'a mut Self::Thing) -> bool;
    fn process<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    fn execute<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    fn postprocess<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing;
    // fn sethook<'a>(&self, t: &'a mut Self) -> &'a mut Self;
}

impl Hooking for Hook<'_> {
    type Thing = String;
    fn preprocess<'a>(&self, _t: &'a mut Self::Thing) -> bool {
        true
    }

    fn process<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        if self.preprocess(t) {
            self.execute(t);
        }
        match &self.hook {
            Some(h) => {
                h.process(t);
            }
            None => {
                // hook chain ends here, no-op.
            }
        }
        self.postprocess(t);
        t
    }

    fn execute<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        t
    }
    fn postprocess<'a>(&self, t: &'a mut Self::Thing) -> &'a mut Self::Thing {
        t
    }

    // fn sethook<'a>(&self, hook: &'a mut Self) -> &'a mut Self {
    //     match &self.hook {
    //         Some(h) => {
    //             h.sethook(hook);
    //         }
    //         None => {
    //             self.hook<'a> = Some(Box::new(hook<'a>));
    //         }
    //     }
    //     hook
    // }
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
    h1.hook = Some(Box::new(&h2));
    h1.process(&mut x);
}
