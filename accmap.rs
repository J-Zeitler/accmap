fn main() {
    let v = [1, 1, 1].iter()
        .scan(0, |acc, x| { *acc = *acc + x; Some(*acc) })
        .collect::<Vec<i32>>();
    println!("{:?}", v)
}
