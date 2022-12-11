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

pub trait Process<Executing> {
    fn process(&self);
}

pub trait Preprocess<Process> {
    fn preprocess(&self);
}

pub trait Preprocesspost<Preprocess> {
    fn preprocess(&self);
}
