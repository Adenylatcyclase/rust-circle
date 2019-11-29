use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub struct Circle<T>{
    head: Option<Rc<Node<T>>>,
}

struct Node<T>{
    val: T,
    next: RefCell<Option<Rc<Self>>>,
    last: RefCell<Option<Rc<Self>>>,
}

impl<T> Circle<T>{
    pub fn new() -> Self{
        Circle{head: None}
    }

    pub fn push_after(&mut self, val:T){
        let node = Node::new(val);
        match self.head{
            Some(ref x) => {
                    match &*x.next.borrow() {
                        Some(ref y) =>Node::combine(&node, &y),
                        None => Node::combine(&node, &x),
                    }
                    Node::combine(x, &node)
                },
            None => {self.head = Some(node);
                    ()},
        }
    }

    pub fn push_after_step(&mut self, val: T){
        self.push_after(val);
        match self.head.as_ref().unwrap().next(){
            Some(x) => self.head = Some(x),
            None => ()
        }
        self.next();
    }

    pub fn push_before(&mut self, val:T){
        let node = Node::new(val);
        match self.head{
            Some(ref x) => {
                    match &*x.last.borrow() {
                        Some(ref y) =>Node::combine(&node, &y),
                        None => Node::combine(&node, &x),
                    }
                    Node::combine(x, &node)
                },
            None => {self.head = Some(node);
                    ()},
        }
    }

    pub fn push_before_step(&mut self, val: T){
        self.push_before(val);
        self.last();
    }

    pub fn next(&mut self) -> &T{
        match self.head.as_ref().unwrap().next(){
            Some(x) => self.head = Some(x),
            None => ()
        }
        return self.head.as_ref().unwrap().value();
    }

    pub fn last(&mut self) -> &T{
        match self.head.as_ref().unwrap().last(){
            Some(x) => self.head = Some(x),
            None => ()
        }
        return self.head.as_ref().unwrap().value();
    }

    pub fn value(& self) -> Option<&T> {
        match self.head.as_ref(){
            Some(ref x) => Some(x.value()),
            None => None,
        }
    }
}

impl<T> Node<T>{
    pub fn new(val: T) -> Rc<Self>{
        return Rc::new(Node{val: val, next: RefCell::new(None), last: RefCell::new(None)});
    }

    pub fn value(&self) -> &T{
        &self.val
    }

    pub fn combine(node1: &Rc<Self>, node2: &Rc<Self>){
        node1.next.replace(Some(node2.clone()));
        node2.last.replace(Some(node1.clone()));
    }

    pub fn next(&self) -> Option<Rc<Self>> {
        match *self.next.borrow(){
            Some(ref x) => Some(x.clone()),
            None => None
        }
    }

    pub fn last(&self) -> Option<Rc<Self>> {
        match *self.last.borrow(){
            Some(ref x) => Some(x.clone()),
            None => None
        }
    }
}