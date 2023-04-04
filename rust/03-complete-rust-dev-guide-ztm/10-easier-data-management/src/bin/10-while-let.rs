fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("{:?}", i);
        data = None;
    }

    let numbers = vec![0, 1, 2, 3];
    let mut numbers_iter = numbers.iter();
    println!("{:?}", numbers_iter.next());

    while let Some(num) = numbers_iter.next() {
        println!("{:?}", num);
    }

    println!("done");
}
