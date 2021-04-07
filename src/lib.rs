use std::collections::VecDeque;

pub fn feature_gating_on_rust_1_48() {
    let mut deque = VecDeque::with_capacity(10);
    deque.push_front(1);
    deque.push_front(2);
    deque.push_front(3);
    deque.push_back(8);
    deque.push_back(9);

    assert_eq!(deque.as_slices().0.len(), 3);
    assert_eq!(deque.as_slices().1.len(), 2);

    // stable since Rust 1.48
    let _ = deque.make_contiguous();

    assert_eq!(deque.as_slices().0.len(), 5);
    assert_eq!(deque.as_slices().1.len(), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        feature_gating_on_rust_1_48();
    }
}
