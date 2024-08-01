use std::collections::VecDeque;
fn main() {
    let mut my_deque: VecDeque<&str> = VecDeque::new();
    my_deque.push_back("a");
    my_deque.push_back("b");

    println!("my_deque[0]: {}", my_deque[0]);
    println!("my_deque[10]: {}", my_deque[10]);
}
