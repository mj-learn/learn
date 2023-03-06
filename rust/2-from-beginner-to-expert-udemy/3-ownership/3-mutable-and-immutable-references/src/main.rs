// Mutable and Immutable References
//    - One mutable reference for a variable in a scope
//    - Many immutable references
//    - Mutable and immutable can not coexist
//    - Data should not change when immutable references are in scope
//    - Scope of a reference

fn main() {
    let mut _nums = vec![1,2,3];
    let ref1 = &mut _nums;
    // let ref2 = &mut _nums;
    // let ref3 = &_nums;
    println!("ref1 = {:?}", ref1);
    println!();

    let mut _nums2 = vec![4,5,6];
    let ref1 = &_nums2;
    let ref2 = &_nums2;
    let ref3 = &_nums2;
    println!("ref1 = {:?}, ref2 = {:?}, ref3 = {:?}", ref1, ref2, ref3);
    println!();

    // Scoop sample
    let mut _nums3 = vec![7,8,9];
    let ref1 = &_nums3;
    // let ref2 = &mut _nums3; // Can't have it here
    println!("ref1 = {:?}", ref1);
    let ref2 = &mut _nums3; // Can have it here
    // let ref3 = &_nums3; // Can't have it here
    println!("ref2 = {:?}", ref2);
    let ref3 = &_nums3; // Can have it here
    println!("ref3 = {:?}", ref3);
    let ref1 = &_nums3;
    println!("ref1 = {:?}", ref1);
}
