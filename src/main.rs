use std::fmt::Debug;

struct Res<T: Debug> {
    value: T,
    ticks: usize,
}

impl<T: Debug> Res<T> {
    fn new(value: T, ticks: usize) -> Self {
        Self { value, ticks }
    }

    fn print(&self) {
        println!("result: {:?}", self.value);
        println!("ticks: {}", self.ticks);
    }
}

fn median(input: &[usize]) -> Res<f64> {
    let mut values = input.to_vec();
    values.sort();
    let len = values.len();

    if len % 2 != 0 {
        let result = values[(len - 1) / 2] as f64;
        Res::new(result, 1)
    } else {
        let result = (values[len / 2] + values[(len / 2) - 1]) as f64 / 2.0;
        Res::new(result, 1)
    }
}

fn largest(input: &[usize], count: usize) -> Res<Vec<usize>> {
    let mut values = input.to_vec();
    values.sort();
    Res::new(values[(values.len() - count)..].to_vec(), 1)
}

fn rotate_left(input: &[usize], offset: usize) -> Res<Vec<usize>> {
    let len = input.len();
    let mut res = vec![0; len];
    let offset = offset % len;
    let mut ticks = 0;
    for (i, v) in input.iter().enumerate() {
        ticks += 1;
        if i < offset {
            res[len - offset + i] = *v;
        } else {
            res[i - offset] = *v;
        }
    }
    Res::new(res, ticks)
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

fn factors(input: i32) -> Res<Vec<i32>> {
    let mut result = vec![];
    let end = f64::sqrt(input.into()).round() as i32;
    let mut ticks = 0;
    for i in 1..=end {
        ticks += 1;
        if input % i == 0 {
            result.push(i);
            if i != end {
                result.push(input / i);
            }
        }
    }
    result.sort();
    Res::new(result, ticks)
}

fn primes_until(input: u32) -> Res<Vec<u32>> {
    let mut ticks = 0;
    let mut primes = vec![true; input as usize];
    primes[0] = false; // 0 is not included in the prime list

    for p in 2..((input as f64).sqrt() as u32 + 1) {
        if primes[p as usize] {
            let mut multiple = p * p;
            while multiple < input {
                primes[multiple as usize] = false;
                multiple += p;
                ticks += 1;
            }
        }
    }

    // this is my initial wrong implementation that I couldn't figure my way out of it.
    // I guess I misunderstood the idea of how to appraoch it.
    //
    //
    // let mut next_val = 2;
    // println!("primes:: {:?}", primes);
    // while next_val < input / 2 {
    //     for i in next_val..input {
    //         ticks += 1;
    //         println!("deep::: {}, position: {}", i, next_val);
    //         if input % i == 0 {
    //             primes[(i - 1) as usize] = false;
    //         }
    //     }
    //     println!("primes:: {:?}", primes);
    //     if let Some(pos) = &primes
    //         .iter()
    //         .enumerate()
    //         .position(|(i, v)| i >= next_val as usize && *v == true)
    //     {
    //         println!("next val {pos}");
    //         next_val = (*pos + 1) as u32;
    //     } else {
    //         break;
    //     }
    // }

    let result = primes
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i as u32)
        .collect();

    Res::new(result, ticks)
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

    let value = 2398;
    start(&"FACTOR", &[value]);
    factors(value as i32).print();

    let value = 36;
    start(&"PRIMES_UNTIL", &[value]);
    primes_until(value as u32).print();
}
