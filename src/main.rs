use std::fmt::Debug;

struct Res<T: Debug> {
    value: T,
}

impl<T: Debug> Res<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn print(&self) {
        println!("result: {:?}", self.value);
    }
}

fn median(input: &[usize]) -> Res<f64> {
    let mut values = input.to_vec();
    values.sort();
    let len = values.len();

    if len % 2 != 0 {
        let result = values[(len - 1) / 2] as f64;
        Res::new(result)
    } else {
        let result = (values[len / 2] + values[(len / 2) - 1]) as f64 / 2.0;
        Res::new(result)
    }
}

fn largest(input: &[usize], count: usize) -> Res<Vec<usize>> {
    let mut values = input.to_vec();
    values.sort();
    Res::new(values[(values.len() - count)..].to_vec())
}

fn rotate_left(input: &[usize], offset: usize) -> Res<Vec<usize>> {
    let len = input.len();
    let mut res = vec![0; len];
    let offset = offset % len;
    for (i, v) in input.iter().enumerate() {
        if i < offset {
            res[len - offset + i] = *v;
        } else {
            res[i - offset] = *v;
        }
    }
    Res::new(res)
}

struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T: Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList ")?;
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{:?} -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, data: T) {
        let next_node = Box::new(ListNode {
            data,
            next: self.head.take(),
        });
        self.head = Some(next_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

fn list(input: &[usize]) -> LinkedList<usize> {
    let mut list = LinkedList::new();
    for v in input {
        list.push(*v);
    }
    list
}

fn start(name: &str, input: &[usize]) {
    println!("");
    println!("----------- {name}");
    println!("input: {input:?}");
}

fn main() {
    let input = vec![3, 2, 5, 123, 4, 2, 1, 234];

    start(&"MEDIAN", &input);
    median(&input).print();

    start(&"LARGEST", &input);
    largest(&input, 2).print();

    start(&"ROTATE_LEFT", &input);
    rotate_left(&input, 10).print();

    start(&"LIST", &input);
    println!("{:?}", list(&input));
}
