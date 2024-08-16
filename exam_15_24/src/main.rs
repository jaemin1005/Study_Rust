use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         //* 자동 참조 해제 */
//         match self  {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    //* value의 borrow_mut의 호출 */
    //* 자동 역참조 기능이 사용되어 Rc<T>를 염차좀하여 안에 있는 RefCell의 값을 얻어온다 */
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
