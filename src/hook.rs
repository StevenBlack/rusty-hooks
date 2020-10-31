use std::option::Option;

#[derive(Debug)]
pub struct Hook<'a> {
    pub name: String,
    pub description: String,
    pub hook: Option<&'a mut Hook<'a>>,
}

impl<'a> Default for Hook<'a> {
    fn default() -> Hook<'a> {
        Hook {
            name: "No-name hook".to_string(),
            description: "No-description".to_string(),
            hook: None,
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
    // fn describe(&mut self) {}
    fn sethook(&mut self, _t: &'a mut Self) -> &mut Self {
        self
    }

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
            ..Default::default()
        };
        assert_eq!(h.name, "No-name hook");
        assert_eq!(h.description, "No-description");
    }
}
