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

pub trait Describing<'a> {
    type Thing;
    fn describe(&mut self) {}
}

pub trait Executing<'a>: Hooking<'a> {
    type Thing;
    fn execute(&mut self, thing: <Self as Executing<'a>>::Thing) -> <Self as Executing<'a>>::Thing {
        thing
    }
}

pub trait Processing<'a> {
    type Thing;
    fn process(&mut self, thing: Self::Thing) -> Self::Thing {
        thing
    }
}

pub trait Preprocess<Process> {
    fn preprocess(&self);
}

pub trait Preprocesspost<Preprocess> {
    fn preprocess(&self);
}
