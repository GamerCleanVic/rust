mod tipos_bool_nums;
use tipos_bool_nums::my_boolean;

fn main() {
    let mut num1_main: bool = true;

    println!("Tipos de nºs: {:?}!", my_boolean(&mut num1_main));
}
