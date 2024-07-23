fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");

    //* 매개변수의 소유권을 가지고 가지 앟는다. */
    let s = format!("{s1}-{s2}-{s3}");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let hello = String::from("Здравствуйте");
    
    for c in "Зд".chars(){
        println!("{c}");
    }

    for b in "Зд".bytes(){
        println!("{b}");
    }
}
