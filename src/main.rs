fn main() {
    //变量默认是不可改变的（immutable）
    //不能对不可变变量 x 二次赋值
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //可以在变量名之前加 mut 来使其可变
    //除了允许改变值之外，mut 向读者表明了其他代码将会改变这个变量值的意图。
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //
    //定义一个与之前变量同名的新变量，而新变量会隐藏（Shadowing）之前的变量。
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let a = [3; 5];
    for elem in a.iter() {
        print!("{} ", elem);
    }
    println!();

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    println!("Hello, world!");
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
