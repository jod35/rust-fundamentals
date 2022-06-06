fn main() {
    let mut x = 5;

    let places = 2.0;

    //addition
    let addition = 5 + 10;

    println!("Addition {}",addition);

    //subtraction
    let subtraction = 23 - 10;

    println!("Subtract {}",subtraction);


    //multiplication
    let multiplication = 23 * 23;

    println!("Multiplication {}",multiplication);

    //division
    let division = 100 / 25;

    println!("Division {}",division);

    //remainder

    let remainder = 42 % 3;
    println!("Remainder {}",remainder);

    println!("Places {}",{places});

    let spaces = "jonathan";

    let _spaces = spaces.len();

    println!("The number of spaces is {}",_spaces);

    println!("The number is {}",x);

    x = 6;

    println!("The number is {}",x);
}
