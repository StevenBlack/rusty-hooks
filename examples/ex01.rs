// Experimenting with hooks in Rust

use hox::*;
use hox::hook::*;
use hox::traits::*;

fn main() {
    let mut h1 = hook!("hook 1", "The first hook.");
    let mut h2 = hook!("hook 2", "The second hook.");
    let mut h3 = hook!("hook 3", "The third hook.");
    let mut h4 = hook!("hook 4", "The forth hook.");
    let mut h5 = hook!("hook 5");

    h3.sethook(&mut h4);
    h1.sethook(&mut h2);

    h1.sethook(&mut h3);
    h1.zethook(&mut h5);

    h1.describe();

    let ret = h1.process("Starting:\n".to_string());
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
