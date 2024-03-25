use std::mem;

struct List {
    head: Link
}

#[derive(Clone)]
enum Link {
    Empty,
    To(Box<Node>)
}

#[derive(Clone)]
struct Node {
    element: i32,
    next: Link
}

impl List {
    fn new() -> Self {
       List {
           head: Link::Empty
       }
    }

    pub fn push(&mut self, element: i32) {
        let node = Node {
            element,
            next: self.head.clone()
        };

        self.head = Link::To(Box::new(node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head = self.head.clone();

        let next_node = match &self.head {
            Link::Empty => Link::Empty,
            Link::To(node) => node.next.clone()
        };

        self.head = next_node;

        match head {
            Link::Empty => None,
            Link::To(node) => Some(node.element)
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::To(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
