#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: Link<T>
}

pub type Link<T> = Option<Box<Node<T>>>;
