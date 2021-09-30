mod list;
mod stack;
pub use crate::stack::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(3);
    stack.push(5);

    println!("{:?}", stack);
}
