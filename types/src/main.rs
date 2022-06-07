fn main() {
    let c = "c";
    let e = "e";


    println!("c -> {}",c);
    println!("e -> {}",e);


    let x: (i32,f64,u8) =(500,5.4,1);

    // let (x,y,z) = tup;

    println!("The value of x is {}",x.0);
    println!("The value of y is {}",x.1);
    println!("The value of z is {}",x.2);


    let a =[1,2,3,4,5,6,7];

    let v:[i32 ; 5]=[1,2,3,45,8];

    let first = v[0];
    let second =v[1];

    println!("The value of the first value is {first}");

    println!("The value of the second value is {second}");

    let index = 1;

    println!("The value of element is {}",a[index]);
    
}
