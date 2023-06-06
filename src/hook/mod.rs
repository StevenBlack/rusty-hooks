use std::option::Option;

#[macro_export]
macro_rules! hook {
    () => {
        Hook {
        ..Default::default()
        }
    };
    ($name:expr) => {
        Hook {
            name: $name.to_string(),
            ..Default::default()
        }
    };
    ($name:expr, $description:expr) => {
        Hook {
            name: $name.to_string(),
            description: $description.to_string(),
            ..Default::default()
        }
    };
    ($name:expr, $description:expr, $hook:expr) => {
        Hook {
            name: $name.to_string(),
            description: $description.to_string(),
            hook: $hook,
            ..Default::default()
        }
    };
    ($name:expr, $description:expr, $hook:expr, $zethook:expr) => {
        Hook {
            name: $name.to_string(),
            description: $description.to_string(),
            hook: $hook,
            zethook: $zethook,
            ..Default::default()
        }
    };
}

#[derive(Debug)]
pub struct Hook<'a> {
    pub name: String,
    pub description: String,
    pub hook: Option<&'a mut Hook<'a>>,
    pub hooks: Vec<&'a mut Hook<'a>>,
}

impl<'a> Default for Hook<'a> {
    fn default() -> Hook<'a> {
        Hook {
            // default name is empty String
            name: "".to_string(),
            // default description is empty String
            description: "".to_string(),
            // no downstream hooks, by default
            hook: None,
            // no additional hooks, by default
            hooks: Vec::new(),
        }
    }
}

pub trait Describing<'a> {
    type Thing;
    fn describe(&mut self) {}
}

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
            "Hook name: '{}' description: '{}' next: '{}'",
            self.name, self.description, next
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

pub trait Executing<'a>: Hooking<'a> {
    type Thing;
    fn execute(&mut self, thing: <Self as Executing<'a>>::Thing) -> <Self as Executing<'a>>::Thing {
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
        let ret = format!("{} - {} pre\n", thing, self.name);
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
        return format!("{} - {} execute\n", thing, self.name);
    }

    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        return format!("{} - {} post\n", thing, self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hook_default_test() {
        // assert_eq!(2 + 2, 4);
        let h = Hook {
            ..Default::default()
        };
        assert_eq!(h.name, "");
        assert_eq!(h.description, "");
    }

    #[test]
    fn hook_one_parameter_test() {
        let tst = hook!("some name");
        assert_eq!(tst.name, "some name");
    }

    #[test]
    fn hook_two_parameter_test() {
        let tst = hook!("some name", "some description");
        assert_eq!(tst.name, "some name");
        assert_eq!(tst.description, "some description");
    }

}
