fn main() {
    println!("Variables");

    let mut x = 5;
    x = 6;
    let _y = x; // hm, interestin
    println!("x = {}", x);

    // Tuple
    let tup = (300, "Hello", true);
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    //Array
    let array = [1, 2, 3];
    println!("Array[0] = {}", array[0]);
    let array2: [i32; 3] = [4, 5, 6];
    println!("Array = {}", array2[0]);

    // Vector
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("Array = {:?}", nums);
    let popped = nums.pop();
    println!("popped = {:?}", popped);
    nums.reverse();
    println!("vec = {:?}", nums);

    // Slice
    let sv: &[i32] = &nums[0..2];
    println!("slice sv = {:?}", sv);

    //Strings
    let my_string = String::from("My string");
    println!("my_string = {}", my_string);
    let my_second_string = "My Secodn String".to_string();
    println!("my_second_string = {}", my_second_string);
}
