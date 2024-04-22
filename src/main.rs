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
        // self.value.print();
    }
}
//
// trait Print {
//     fn print(&self);
// }
//
// impl Print for f64 {
//     fn print(&self) {
//         println!("result: {self}");
//     }
// }
//
// impl Print for Vec<usize> {
//     fn print(&self) {
//         println!("result: {self:?}");
//     }
// }


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
}
