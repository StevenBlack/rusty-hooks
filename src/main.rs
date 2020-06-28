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
    fn describe(&mut self);
    fn sethook(&mut self, t: &'a mut Self) -> &mut Self;
    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing);
    fn process(&mut self, thing: Self::Thing) -> Self::Thing;
    fn execute(&mut self, thing: Self::Thing) -> Self::Thing;
    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing;
}

impl<'a> Hooking<'a> for Hook<'a> {
    type Thing = String;

    fn describe(&mut self) {
        println!("{}", self.description);
        match self.hook {
            Some(ref mut h) => {
                h.describe();
            }
            None => {}
        }
    }

    fn sethook(&mut self, hook_passed: &'a mut Self) -> &mut Self {
        match self.hook {
            Some(ref mut h) => h.sethook(hook_passed),
            None => {
                self.hook = Some(hook_passed);
                self.hook.as_mut().unwrap()
            }
        }
    }

    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        let ret = format!("{} - {} pre", thing, self.name);
        (true, ret)
    }

    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        let (ok, mut ret) = self.preprocess(thing);
        if ok {
            ret = self.execute(ret);
        }
        match self.hook {
            Some(ref mut h) => {
                ret = h.process(ret);
            }
            None => {}
        }
        ret = self.postprocess(ret);
        ret
    }
    fn execute(&mut self, thing: Self::Thing) -> Self::Thing {
        return format!("{} - {}", thing, self.name);
    }

    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        return format!("{} - {} post", thing, self.name);
    }
}

fn main() {
    let mut h1 = Hook {
        name: "hook 1".to_string(),
        description: "The first hook.".to_string(),
        hook: None,
    };
    let mut h2 = Hook {
        name: "hook 2".to_string(),
        description: "The second hook.".to_string(),
        hook: None,
    };
    let mut h3 = Hook {
        name: "hook 3".to_string(),
        description: "The third hook.".to_string(),
        hook: None,
    };
    let mut h4 = Hook {
        name: "hook 4".to_string(),
        description: "The forth hook.".to_string(),
        hook: None,
    };
    h3.sethook(&mut h4);
    h1.sethook(&mut h2);
    h1.sethook(&mut h3);
    h1.describe();
    let ret = h1.process("The quick brown fox".to_string());
    println!("{}", ret);
}
