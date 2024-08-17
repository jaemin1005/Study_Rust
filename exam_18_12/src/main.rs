struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x:0, y: 7};

    // let Point {x: a, y: b} = p;
    let Point { x, y} = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y:0 } => println!("On the x axis at {x}"),
        Point { x: 0, y} => println!("On the y axis at {y}"),
        Point { x, y} => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
