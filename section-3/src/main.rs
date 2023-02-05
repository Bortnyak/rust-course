fn main() {
    // ownership
    // let my_var = 1; // on the stack;
    // let my_str = String::from("String"); // on the heap;
    let x = vec!["tyler".to_string()];
    let y = x;
    let z = y;
    let z_moved_here = z;
    println!("{:?}", z_moved_here);

    // values that are stored on the stack be default implements Copy
    let mut my_int = 3;
    let my_int_2 = my_int;
    my_int = 5;
    println!("{}", my_int);
    println!("{}", my_int_2);
}
