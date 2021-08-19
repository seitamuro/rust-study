mod list;
mod mybox;
mod drop;
mod mock;

use list::List::{Cons, Nil};
use list::RcList::{RCons, RNil};
use list::RRList::{RRCons, RRNil};
use mybox::MyBox;
use drop::CustomSmartPointer;
use std::mem::drop;
use std::rc::Rc;
use std::cell::RefCell;

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

    let c = CustomSmartPointer {data: String::from("my stuff")};
    let _d = CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Rc<T>
    let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = RCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Rc<RefCell<T>>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RRCons(Rc::clone(&value), Rc::new(RRNil)));

    let b = RRCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RRCons(Rc::new(RefCell::new(16)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}