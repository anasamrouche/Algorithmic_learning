use std::rc::{Rc, Weak};

pub struct Node {
    parent: Option<Rc<Node>>,
    children: Option<Vec<Weak<Node>>>
}

pub struct Tree<T>{
    nodes: Vec<(Node, T)>,
}