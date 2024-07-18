fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    let tuple = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is : {x}");

    let number = 6;

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn five() -> i32 {
    5
}