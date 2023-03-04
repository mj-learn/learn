fn main() {
    // Tuples
    let tuple = ("My salary", 40_000);
    println!("{} is {} year", tuple.0, tuple.1);
    println!("Print the whole tuple: {:?}", tuple);
    println!();

    // Destracting
    let (val1, val2) = tuple;
    println!("val1 = {} and val2 = {}", val1, val2);
    println!();

    // Nested tuples
    let nested_tuple = (40_000, "String", ("monjofn",20_000, true), 's');
    println!("nested_tuple = {:?}", nested_tuple);
    let element = nested_tuple.2.1; // element = 20_000
    println!("element = {}", element);

    // Empty tuple
    let empty_tuple = ();
    println!("empty_tuple = {:?}", empty_tuple);
    println!();

    // Arrays - collection of elements in same type
    let mut number_array: [i32;5] = [4,5,6,7,8];
    println!("number_array = {:?}", number_array);
    println!("number_array[0] = {}", number_array[0]);

    number_array[3] = 1;
    println!("number_array = {:?}", number_array);
    println!();

    let array_with_same_elements = [0; 5];
    println!("array_with_same_elements = {:?}", array_with_same_elements);
    println!();

    let mut string_array = ["apple", "orange", "tomato"];
    println!("string_array = {:?}", string_array);
    let unknow_array = ["unknow"; 4];
    println!("unknow_array = {:?}", unknow_array);
    string_array[1] = "MOnjofn";
    println!("string_array = {:?}", string_array);
    let char_array = ['a', 'b', 'c'];
    println!("char_array = {:?}", char_array);
    println!();

    // Common operation
    let arr1 = [4,5,6,7,8,9];
    println!("arr1 = {:?}", arr1);
    println!("arr1.len() = {}", arr1.len());
    println!("std::mem::size_of_val(&arr1) = {}", std::mem::size_of_val(&arr1));

    // slice
    let subset_arr1 = &arr1[2..4]; // NOT include last
    println!("subset_arr1 = {:?}", subset_arr1);
    let subset_arr2 = &arr1[2..=4]; // include last
    println!("subset_arr2 = {:?}", subset_arr2);

    // get
    let check_index_1 = arr1.get(0);
    let check_index_2 = arr1.get(100);
    println!("check_index_1 = {:?}", check_index_1);
    println!("check_index_2 = {:?}", check_index_2);
}
