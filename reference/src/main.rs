fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let reference_to_nothing = dangle();
}


fn dangle() -> String {
    let s = String::from("hello");

    //* dangling pointer */
    //*! this function's return type contains a borrowed value, but there is no value */
    //&s

    s
}