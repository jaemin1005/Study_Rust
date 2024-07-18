fn main() {
    let s = String::from("hello"); //* s가 스코프안으로 들어옴 */
    
    takes_ownership(s); //* s의 값이 함수로 이동 */
    //* ... 여기서부터는 더이상 s의 값이 유효하지가 않음 */

    //*! borrow of moved value: 's' value borrowd here after move*/
    //println!("{s}"); 

    //* x가 스코프 안으로 들어옴 */
    let x = 5;

    //* x함수는 이동이 되겠지만 i32 형식은 copry 계속 사용이 가능하다 */
    makes_copy(x);
}
/**
 * * 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 됨.
 * * 하지만 s의 값이 이동되었음으로
 * * 별다른 일이 발생하지 않음
 */

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}
//* son_string이 스코프 밖으로 벗어나고 'drop'이 호출됨 */
//*! 메모리 해제 */

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
//* 여기서는 sone_integer가 스코프 밖으로 벗어남, 별다른일이 발생하지 않음 */