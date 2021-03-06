use crate::hook::*;

struct UppercaseParserHook<'a> {
    pub hook: Option<&'a mut Hook<'a>>,
    pub hooks: Vec<&'a mut Hook<'a>>,
}

impl<'a> Hooking<'a> for UppercaseParserHook<'a> {
    type Thing = String;

    fn sethook(&mut self, _t: &'a mut Self) -> &mut Self {
        self
    }

    fn addhook(&mut self, _t: &'a mut Self) -> () {}

    fn preprocess(&mut self, thing: Self::Thing) -> (bool, Self::Thing) {
        (true, thing)
    }
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        self.execute(thing)
    }
    fn execute(&mut self, thing: Self::Thing) -> Self::Thing {
        thing.to_uppercase()
    }
    fn postprocess(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

struct ItalicBoldParserHook;
struct TagCloseParserHook;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uppercase_parser_hook_test() {
        // assert_eq!(2 + 2, 4);
        let mut h = UppercaseParserHook {
            hook: None,
            hooks: Vec::new(),
        };
        assert_eq!(h.process("hello".to_string()), "HELLO".to_string());
    }
}
