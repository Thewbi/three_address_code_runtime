use std::fmt::Debug;

#[derive(Clone)]
pub struct Node<T> {

    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,

    // this flag is used in visitor.process_instruction() to differentiate
    // between registers, immediate values and expressions
    pub expression: bool,

    // the unary * operator is applied to this node
    pub deref: bool,
    
}

impl<T> Node<T> {

    // USAGE: 
    // let mut op_node: Node<String> = Node::new(String::from("sqrt"));
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            expression: false,
            deref: false,
        }
    }

    pub fn left(mut self, node: Node<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

impl<T: Debug> std::fmt::Debug for Node<T> {

    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(formatter, "Node[{:?} expr:{:?} deref:{:?} LHS:{:?} RHS:{:?}]", self.value, self.expression, self.deref, self.left, self.right)
    }

}