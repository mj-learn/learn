fn main() {
    let a = Some(1);
    dbg!(a);
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);
    let a_filtered = a.filter(|num| num == &1); // keep data if true
    dbg!(a_filtered);
    let a_or_else = a.or_else(|| Some(5)); // return Some(5) if a is None
    dbg!(a_or_else);
    let a_unwrapped = a.unwrap_or_else(|| 0); // return not option data or 0 if is None 
    dbg!(a_unwrapped);
}
