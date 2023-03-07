fn main() {
    
    // vectors
    // * Identical values
    // * Can grow or shrink

    let num_vector_1: Vec<i32> = vec![4,5,6,7,8,9,10,11,12,16,10];
    println!("num_vector_1 = {:?}", num_vector_1);
    println!("num_vector_1[0] = {}", num_vector_1[0]);
    println!();
    
    let mut num_vector_2 = vec![1,2,3,4,5,6,73,4,5,2,11];
    println!("num_vector_2 = {:?}", num_vector_2);
    num_vector_2[0] = 11_0000;
    println!("num_vector_2[0] = {}", num_vector_2[0]);
    println!();

    let _num_vector_3 = vec![0; 10];
    println!("_num_vector_3 = {:?}", _num_vector_3);
    let char_vector = vec!['a','v','c','d','e'];
    println!("char_vector = {:?}", char_vector);
    let subset_char_vector = &char_vector[0..3];
    println!("subset_char_vector = {:?}", subset_char_vector);
    println!("char_vector.len() = {}", char_vector.len());
    println!("char_vector.get(100) = {:?}", char_vector.get(100));
    println!("char_vector.get(5) = {:?}", char_vector.get(5));
    println!();

    num_vector_2.push(22);
    println!("num_vector_2 = {:?}", num_vector_2);

    num_vector_2.remove(5); // Remove 6-th element
    println!("num_vector_2 = {:?}", num_vector_2);
}
