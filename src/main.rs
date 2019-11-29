
struct Circle{
    head: Rc<RefCell<Node>>,
}

struct Node{
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    last: Option<Rc<RefCell<Node>>>,
}

impl Circle{

    pub fn remove(&mut self) -> i32 {
        let ret = self.value();
        let mut next = match &self.head.borrow().next{
            Some(x) => x.clone(),
            None => panic!("can't remove last element"),
        };
        let mut last = self.head.borrow().last.as_ref().unwrap().clone();
        Node::combine(&mut last, &mut next);
        self.head = next;
        return ret;
    }

    pub fn new(val: i32) -> Circle {
        return Circle{head: Node::new(val)}
    }

    pub fn value(&self) -> i32 {
        return self.head.borrow().value();
    }

    pub fn next(&mut self) -> i32{
        match &self.head.clone().borrow().next{
            Some(x) => self.head = x.clone(),
            None => (),
        }
        self.head.borrow().value()
    }

    pub fn last(&mut self) -> i32{
        match &self.head.clone().borrow().last{
            Some(x) => self.head = x.clone(),
            None => (),
        }
        self.head.borrow().value()
    }

    pub fn insert_after(&mut self, val: i32){
        let mut node = match &self.head.clone().borrow().next{
            Some(x) => x.clone(),
            None => self.head.clone(),
        };
        let mut new_node = Node::new(val);
        Node::combine(&mut self.head, &mut new_node);
        Node::combine(&mut new_node, &mut node);
    }

    pub fn insert_after_step(&mut self, val: i32){
        self.insert_after(val);
        self.next();
    }

    pub fn insert_before(&mut self, val: i32){
        let mut node = match &self.head.clone().borrow().last{
            Some(x) => x.clone(),
            None => self.head.clone(),
        };
        let mut new_node = Node::new(val);
        Node::combine(&mut new_node, &mut self.head);
        Node::combine(&mut node, &mut new_node);
    }

    pub fn insert_before_step(&mut self, val: i32){
        self.insert_before(val);
        self.next();
    }
}
impl Node{
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        return Rc::new(RefCell::new(Node{val:val, next: None, last: None}));
    }

    pub fn combine(node1: &mut Rc<RefCell<Node>>, node2: &mut Rc<RefCell<Node>>){
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().last = Some(node1.clone());
    }

    pub fn value(&self) -> i32 {
        return self.val;
    }
fn main() {
}

fn advent09(){
    let mut max: i32 = 0;
    let mut scores: [i32; 413] = [0; 413];
    let mut circle = Circle::new(0);
    for i in 0..71082{
        let v : i32 = i + 1;
        if v % 23 == 0 {
            let p : usize= (i % 413) as usize;
            scores[p] += v;
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            scores[p] += circle.remove();
        }else{
            circle.next();
            circle.insert_after_step(v);
        }
    }
    
    for i in 0..413 as usize{
        if scores[i] > max{
            max = scores[i];
        }
    }
    println!("{}", max);
}
