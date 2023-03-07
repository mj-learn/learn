fn main() {
    let mut nums = vec![1,2,3];
    nums.push(4);
    println!("nums = {:?}", nums);
    nums.pop();
    println!("nums = {:?}", nums);
    println!();

    let mut nums2 = Vec::new();
    println!("nums2 = {:?}", nums2);
    nums2.push(7);
    println!("nums2 = {:?}", nums2);
    println!();

    let mut nums3 = vec![];
    println!("nums3 = {:?}", nums3);
    nums3.push(77);
    println!("nums3 = {:?}", nums3);

    let nums4: Vec<i32> = (0..5).collect();
    println!("nums4 = {:?}", nums4);
    println!();

    let nums5: Vec<i32> = (0..=5).collect();
    println!("nums4 = {:?}", nums5);
    println!();
}
