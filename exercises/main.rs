//function that calculates the area of a circle
fn calculate_area_of_circle(radius: f32) {
    const PI: f32 = 3.14;

    let area = PI * (radius * radius);

    println!(
        "The area of a triangle with radius of {} is {}",
        radius, area
    );
}

/*
This is the main function
*/

fn calculate_family_age() {
    let age1 = 32;
    let age2 = 44;
    let age3 = 44;

    let age4 = 50;

    let total = age1 + age2 + age3 + age4;

    let average = total / 4;

    if average > 60 {
        println!(
            "The family average age is {} and The family is so old.",
            average
        );
    } else {
        println!(
            "The family average age is {} and the Family is young.",
            average
        );
    }
}

fn print_hello() {
    let mut count = 0;

    loop {
        println!("Hello!");

        count += 1;

        if count >= 5 {
            break;
        }
    }
}

fn main() {
    let number = 44;

    let name = "Ssali Jonathan";

    calculate_area_of_circle(2.0);

    println!("I am {} years old.", number);

    println!("My name is {}", name);

    calculate_family_age();

    print_hello();
}
