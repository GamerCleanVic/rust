pub fn sentenca(num1: &mut i64, num2: &mut i64) -> i64 {
    let verifica: bool = num1 > num2;

    if verifica {
        println!("{} > {}", num1, num2);
        1
    } else if num1 < num2 {
        println!("{} < {}", num1, num2);
        -1
    } else {
        println!("{} == {}", num1, num2);
        0
    }
}