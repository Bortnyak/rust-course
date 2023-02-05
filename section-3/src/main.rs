use std::{ptr::eq, vec};

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

    // vector task
    let mut vec_vals = vec![1, 3, 5, 7];
    let is_eq = is_first_eq_one(&vec_vals);
    println!("Is first equal to one = {:?}", is_eq);

    add_15_to_vec(&mut vec_vals);
    println!("vecotr after mutation = {:?}", vec_vals);
}

fn is_first_eq_one(val: &Vec<i32>) -> bool {
    let is_equal_to_one = &val[0] == &1;
    if !is_equal_to_one {
        return false;
    }
    true
}

fn add_15_to_vec(val: &mut Vec<i32>) {
    val.push(15)
}
