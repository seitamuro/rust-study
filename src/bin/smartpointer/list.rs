use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub enum RcList {
    RCons(i32, Rc<RcList>),
    RNil,
}