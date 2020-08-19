fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}