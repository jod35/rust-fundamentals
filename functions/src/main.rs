fn main() {
    return_name();
}


fn return_name(){
    println!("Hello world");

    add_two(2,3);
}


fn add_two(x:i32,y:i32){
    let total:i32 = x + y;

    println!("{} + {} = {}",x,y,total);
}

