mod fluxos_ctrl;//declarando q name space vai usar
use fluxos_ctrl::sentenca;//declara qual função vai usar do arquivo importado

fn main() {
    let mut result: i64 = sentenca(&mut 4, &mut 4);

    println!("É >= : {:?}!", result);
}
