// Experimenting with hooks in Rust
mod hook;
use crate::hook::*;

fn main() {
    let mut h1 = hook::Hook {
        name: "hook 1".to_string(),
        description: "The first hook.".to_string(),
        ..Default::default()
    };
    let mut h2 = hook::Hook {
        name: "hook 2".to_string(),
        description: "The second hook.".to_string(),
        ..Default::default()
    };
    let mut h3 = hook::Hook {
        name: "hook 3".to_string(),
        description: "The third hook.".to_string(),
        ..Default::default()
    };
    let mut h4 = hook::Hook {
        name: "hook 4".to_string(),
        description: "The forth hook.".to_string(),
        ..Default::default()
    };

    let mut h5 = hook::Hook {
        name: "hook 5".to_string(),
        description: "The fifth hook.".to_string(),
        ..Default::default()
    };

    h3.sethook(&mut h4);
    h1.sethook(&mut h2);

    h1.sethook(&mut h3);
    h1.zethook(&mut h5);

    h1.describe();

    let ret = h1.process("Starting: ".to_string());
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
