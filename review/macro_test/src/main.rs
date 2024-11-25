macro_rules! my_vec {
    // :expr : 타입 지정자, 러스트에서 사용 가능한 값이나 연산식을 받는다.
    // $() : 바깥의 $()는 반복 블록을 나타낸다.
    // ,* : 0개 이상의 반복을 허용 ( +일 경우 1개 이상을 뜻함 :)
    ($($x:expr),*) => {
        // 스코프 제한 
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3, 4];
    println!("{:?}", v);
}
