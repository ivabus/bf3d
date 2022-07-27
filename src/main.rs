
use std::{env};
mod bf3d;

fn main() {
    if &env::args().len() < &2usize {
        println!("Give a file with bf3d code as argument");
        std::process::exit(255);
    }
    let input = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]);
    let input = match input {
        Ok(file) => file,
        Err(error) => {
            println!("{}", error.to_string());
            std::process::exit(255);
        },
    };
    bf3d::bf3d(input, true);
}

#[cfg(test)]
mod tests {
    use crate::bf3d;
    #[test]
    fn basic_hello_world(){
        assert_eq!(bf3d::bf3d(
            String::from("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.-------------------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.------.--------.-------------------------------------------------------------------.-----------------------."),
                              false),
                   vec!['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n'])
    }
    #[test]
    fn optimized_hello_world(){
        assert_eq!(bf3d::bf3d(
            String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."),
                              false),
                   vec!['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n'])
    }
    #[test]
    fn optimized_hello_world_on_oy(){
        assert_eq!(bf3d::bf3d(
            String::from("++++++++++[^+++++++^++++++++++^+++^+____-]^++.^+.+++++++..+++.^++.__+++++++++++++++.^.+++.------.--------.^+.^."),
            false),
                   vec!['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n'])
    }
    #[test]
    fn optimized_hello_world_on_oz(){
        assert_eq!(bf3d::bf3d(
            String::from("++++++++++[/+++++++/++++++++++/+++/+\\\\\\\\-]/++./+.+++++++..+++./++.\\\\+++++++++++++++./.+++.------.--------./+./."),
            false),
                   vec!['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n'])
    }
}