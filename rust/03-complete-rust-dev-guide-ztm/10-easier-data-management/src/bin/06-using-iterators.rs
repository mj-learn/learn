fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    // map
    let plus_one: Vec<_> = numbers
        .iter()
        .map(|x| x + 1)
        .collect();
    println!("map +1: {:?}", plus_one);

    // filter
    let filter: Vec<_> = numbers
        .iter()
        .filter(|&x| *x <= 3)
        .collect();
    println!("filter <= 3: {:?}", filter);

    // find
    let find = numbers
        .iter()
        .find(|&&x| x == 3);
    println!("find 3: {:?}", find);

    // count
    let count = numbers
        .iter()
        .count();
    println!("count: {:?}", count);
    

    // last
    let last = numbers.iter().last();
    println!("last: {:?}", last);

    // min
    let min = numbers.iter().min();
    println!("min: {:?}", min);

    // max
    let max = numbers.iter().max();
    println!("max: {:?}", max);

    // take
    let take: Vec<_> = numbers.iter().take(3).collect();
    println!("take 3: {:?}", take);
}
