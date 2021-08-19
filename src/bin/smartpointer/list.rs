#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Debug)]
pub enum RRList {
    RRCons(Rc<RefCell<i32>>, Rc<RRList>),
    RRNil,
}

#[derive(Debug)]
pub enum CRList {
    CRCons(i32, RefCell<Rc<CRList>>),
    CRNil,
}

impl CRList {
    pub fn tail(&self) -> Option<&RefCell<Rc<CRList>>> {
        match *self {
            CRList::CRCons(_, ref item) => Some(item),
            CRList::CRNil => None,
        }
    }
}