mod list;
use list::List::{Cons, Nil};

fn main() {
    // Box型｡ヒープデータへのポインタ｡
    let b = Box::new(5);
    println!("{}", b);
    
    let conslist = Cons(1,
        Box::new(Cons(2, 
        Box::new(Cons(3,
        Box::new(Cons(4,
        Box::new(Nil))))))));
    println!("{:?}", conslist);
}