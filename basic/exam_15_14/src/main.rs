struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartpointer with data '{}'!", self.data);
    }
}

fn main() {
    let mut c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    //* 에러 발생 */
    // c.drop();

    //* 일찍 값을 메모리 정리하고 싶다면 std::men:drop 함수를 이용 */
    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}
