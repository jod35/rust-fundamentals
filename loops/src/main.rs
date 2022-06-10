fn loop_through_list_using_while() {
    let _list_ = [10, 12, 13, 14, 15, 16];

    let mut index = 0;

    while index < 6 {
        println!("The value for index is {}", _list_[index]);

        index += 1;
    }
}

fn loop_through_list_using_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}", element);
    }
}

fn main() {
    println!("Hello, world!");

    loop_through_list_using_while();
    loop_through_list_using_for();
}
