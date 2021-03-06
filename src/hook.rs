use std::option::Option;

#[derive(Debug)]

/// Hook structure
pub struct Hook<'a> {
    pub name: String,
    pub hook: Option<&'a mut Hook<'a>>,
    pub hooks: Vec<&'a mut Hook<'a>>,
}

/// Default template for Hook
impl<'a> Default for Hook<'a> {
    fn default() -> Hook<'a> {
        Hook {
            name: "".to_string(),
            hook: None,
            hooks: Vec::new(),
        }
    }
}

/// Trait that describes an instance
pub trait Describing<'a> {
    type Thing;
    fn describe(&mut self) {}
}

/// Trait implementation for Hook
impl<'a> Describing<'a> for Hook<'a> {
    type Thing = String;
    fn describe(&mut self) {
        let mut next = "not hooked".to_string();
        match self.hook {
            Some(ref mut h) => {
                next = h.name.clone();
            }
            None => {}
        }

        println!(
            "Hook name: '{}' next: '{}'",
            self.name, next
        );
        match self.hook {
            Some(ref mut h) => {
                h.describe();
            }
            None => {}
        }

        // here iterate self.hooks[]
        for h in self.hooks.as_mut_slice() {
            h.describe();
        }

    }
}

pub trait Processing<'a> {
    type Thing;
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

pub trait Preprocessing<'a> {
    type Thing;
    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        (true, thing)
    }
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

pub trait Prepostprocessing<'a> {
    type Thing;
    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        (true, thing)
    }
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

pub trait Hooking<'a> {
    type Thing;
    fn sethook(&mut self, _t: &'a mut Self) -> &mut Self {
        self
    }

    fn zethook(&mut self, _t: &'a mut Self) -> () {}

    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        (true, thing)
    }
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
    fn execute(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

/// Generic executing trait
pub trait Executing<'a> {
    type Thing;
    fn execute(
        &mut self,
        thing: <Self as Executing<'a>>::Thing
    ) -> <Self as Executing<'a>>::Thing {
        thing
    }
}

impl<'a> Hooking<'a> for Hook<'a> {
    type Thing = String;

    fn sethook(&mut self, hook_passed: &'a mut Self) -> &mut Self {
        match self.hook {
            Some(ref mut h) => h.sethook(hook_passed),
            None => {
                self.hook = Some(hook_passed);
                self.hook.as_mut().unwrap()
            }
        }
    }

    fn zethook(&mut self, hook_passed: &'a mut Self) -> () {
        self.hooks.push(hook_passed);
    }

    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        let ret = format!("{} - {} pre", thing, self.name);
        (true, ret)
    }

    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        let (ok, mut ret) = self.preprocess(thing);
        if ok {
            // allot will hapen here
            ret = self.execute(ret);
        }

        match self.hook {
            Some(ref mut h) => {
                ret = h.process(ret);
            }
            None => {}
        }
        if ok {
            ret = self.postprocess(ret);
        }

        // here iterate self.hooks[]
        for h in self.hooks.as_mut_slice() {
            ret = h.process(ret);
        }
        ret
    }

    fn execute(&mut self, thing: Self::Thing) -> Self::Thing {
        return format!("{} - {} execute", thing, self.name);
    }

    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        return format!("{} - {} post", thing, self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hook_default_tests() {
        // assert_eq!(2 + 2, 4);
        let h = Hook {
            name: "test hook".to_string(),
            ..Default::default()
        };
        assert_eq!(h.name, "test hook");
    }
}
