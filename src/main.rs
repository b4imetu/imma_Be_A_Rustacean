include!("funcs.rs");
include!("structs.rs");
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
    // The value of x is: 12

    let a = [3; 5];
    for elem in a.iter() {
        print!("{} ", elem);
    }
    println!();
    // 3 3 3 3 3

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    println!("Hello, world!");

    // The value of spaces is: 3
    // Hello, world!

    another_function(5, 6);
    // The value of x is: 5
    // The value of y is: 6

    // *loops*
    // loop {
    //     println!("again!");
    // }
    // ctrl-c，来终止一个陷入无限循环的程序

    //
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50

    //
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    // 3!
    // 2!
    // 1!
    // LIFTOFF!!!

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
                       // hello, world!

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // The length of 'hello' is 5.

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);

    // 可变引用
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // hello and hello
    // hello

    // 结构体 struct User
    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
    // error[E0106]: missing lifetime specifier

    // 一个使用结构体的示例程序
    // 示例 5-8：通过分别指定长方形的宽和高的变量来计算长方形面积
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
    // The area of the rectangle is 1500 square pixels.

    // 示例 5-10：定义 Rectangle 结构体
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // The area of the rectangle is 1500 square pixels.
}
