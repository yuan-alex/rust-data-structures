struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn is_empty(&self) -> bool {
        return self.head.is_none();
    }

    fn print(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{}", node.data);
            current = &node.next;
        }
        println!("");
    }
}

fn main() {
    let mut list: LinkedList<i16> = LinkedList::new();

    list.push(12);

    list.print()
}
