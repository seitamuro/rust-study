mod list;
mod mybox;

use list::List::{Cons, Nil};
use mybox::MyBox;

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

    // Deref｡参照外し演算子の動作を定義している｡
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}