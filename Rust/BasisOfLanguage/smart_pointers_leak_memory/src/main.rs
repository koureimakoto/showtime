use std::{
    cell::{RefCell, Ref},
    rc::{
        Rc,
        Weak
    }, str, borrow::Borrow
};

use crate::List::{Cons, Nil};


#[derive(Debug)]
enum
List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn
    tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}
    


fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("Contagem incial de ponteiros para A: {}", Rc::strong_count(&a));
    println!("O próximo item = {:?}", a.tail());

    let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("Contagem incial de ponteiros para A: {}", Rc::strong_count(&a));
    println!("Contagem incial de ponteiros para B: {}", Rc::strong_count(&b));
    println!("O próximo item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("Contagem incial de ponteiros para A: {}", Rc::strong_count(&a));
    println!("Contagem incial de ponteiros para B: {}", Rc::strong_count(&b));

    // println!("a next item = {:?}", a.tail()); 

    // Tree

    let leaf = create_node(3);
    let branch = create_node(5);
    add_in_branch(&branch, &leaf);

    println!("Contagem incial de ponteiros para leaf  : {}", Rc::strong_count(&leaf));
    println!("Contagem incial de ponteiros para branch: {}", Rc::strong_count(&branch));


    let weak_branch = create_weak_node(2);
    let weak_leaf = create_weak_node(5);
    println!("Contagem incial de ponteiros para leaf  : forte: {}  -  fraco: {}", Rc::strong_count(&weak_leaf), Rc::weak_count(&weak_leaf));
    println!("Contagem incial de ponteiros para branch: forte: {}  -  fraco: {}", Rc::strong_count(&weak_branch), Rc::weak_count(&weak_branch));
    println!("Pai da folha : {:?}", weak_leaf.parent.borrow().upgrade());
    println!("Pai da branch: {:?}", weak_branch.parent.borrow().upgrade());

    add_in_weak_branch(&weak_branch, &weak_leaf);
    println!("Contagem incial de ponteiros para weak leaf  : {}", Rc::strong_count(&weak_leaf));
    println!("Contagem incial de ponteiros para weak branch: {}", Rc::strong_count(&weak_branch));
    println!("Pai da folha : {:?}", weak_leaf.parent.borrow().upgrade());
    println!("Pai da branch: {:?}", weak_branch.parent.borrow().upgrade());


    println!("\n teste em escopo diferent\n");
    let weak_leaf = create_weak_node(10);  
    println!("Contagem incial de ponteiros para leaf  : forte: {}  -  fraco: {}", Rc::strong_count(&weak_leaf), Rc::weak_count(&weak_leaf));
    println!("Pai da folha : {:?}", weak_leaf.parent.borrow().upgrade());
    
    {
        let weak_branch = create_weak_node(5);
        add_in_weak_branch(&weak_branch, &weak_leaf);

        println!("Contagem incial de ponteiros para branch: forte: {}  -  fraco: {}", Rc::strong_count(&weak_branch), Rc::weak_count(&weak_branch));
        println!("Contagem incial de ponteiros para leaf  : forte: {}  -  fraco: {}", Rc::strong_count(&weak_leaf), Rc::weak_count(&weak_leaf));

    }
    println!("Pai da folha : {:?}", weak_leaf.parent.borrow().upgrade());
    println!("Contagem incial de ponteiros para leaf  : forte: {}  -  fraco: {}", Rc::strong_count(&weak_leaf), Rc::weak_count(&weak_leaf));



}



#[derive(Debug)]
pub struct
Node {
    value: i32,
    /*
    "
    We want a Node to own its children, and we want to share that ownership with variables so we can
    access each  Node in the tree directly.  To do this, we define the  Vec<T> items to be values of
    type Rc<Node>.  We also want to modify  which nodes are  children of  another node, so we have a 
    RefCell<T> in children around the Vec<Rc<Node>>.
    "
    */
    pub children: RefCell< Vec< Rc< Node > > >,
}

pub fn create_node(value: i32) -> Rc<Node> {
    Rc::new(
        Node { value, children: RefCell::new(vec![]) }
    )
} 

pub fn add_in_branch(branch: &Rc<Node>, leaf: &Rc<Node>) {
    branch.children.borrow_mut().push(Rc::clone(leaf))
}


// --
#[derive(Debug)]
pub struct
WeakNode{
    value: i32,
    parent: RefCell<Weak<WeakNode>>,
    children: RefCell<Vec<Rc<WeakNode>>>
}

pub fn create_weak_node(value: i32) -> Rc<WeakNode> {
    Rc::new(
        WeakNode {
            value,
            parent  : RefCell::default(),
            children: RefCell::new(vec![]),
        }
    )
}

pub fn add_in_weak_branch(weak_branch: &Rc<WeakNode>, weak_leaf: &Rc<WeakNode>) {
    *weak_leaf.parent.borrow_mut() = Rc::downgrade(&weak_branch);
    weak_branch.children.borrow_mut().push(Rc::clone(weak_leaf))
}