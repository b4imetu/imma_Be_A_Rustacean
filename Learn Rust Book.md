{:toc}

# Learn Rust Book

**Learning Rust Lang**




```
Leetcode.com
Rust
take(), swap()和 mem 里
refcell rc 这几个智能指针，以及 borrowchecker 两大原则
```



# 1 [入门指南](https://kaisery.github.io/trpl-zh-cn/ch01-00-getting-started.html#入门指南)


首先 Rust 的缩进风格使用 4 个空格，而不是 1 个制表符（tab）。

第二，`println!` 调用了一个 Rust 宏（macro）。如果是调用函数，则应输入 `println`（没有`!`）。

## 1.3 [Hello, Cargo!](https://kaisery.github.io/trpl-zh-cn/ch01-03-hello-cargo.html#hello-cargo)

cargo new {工程名}



*Cargo.toml* 的内容

这个文件使用 [*TOML*](https://github.com/toml-lang/toml) (*Tom's Obvious, Minimal Language*) 格式，这是 Cargo 配置文件的格式。

最后一行，`[dependencies]`，是罗列项目依赖的片段的开始。在 Rust 中，代码包被称为 *crates*。这个项目并不需要其他的 crate，不过在第二章的第一个项目会用到依赖，那时会用得上这个片段。



rustup update

cargo build

我们刚刚使用 `cargo build` 构建了项目，并使用 `./target/debug/hello_cargo` 运行了程序

也可以使用 `cargo run` 在一个命令中同时编译并运行生成的可执行文件：

Cargo 还提供了一个叫 `cargo check` 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件：

### [发布（release）构建](https://kaisery.github.io/trpl-zh-cn/ch01-03-hello-cargo.html#发布release构建)

当项目最终准备好发布时，可以使用 `cargo build --release` 来优化编译项目。这会在 *target/release* 而不是 *target/debug* 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行 `cargo build --release` 并使用 *target/release* 下的可执行文件进行测试。

# 2 [猜猜看游戏教程](https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html)

[编写 猜猜看 游戏](https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html#编写-猜猜看-游戏)



# 3 [常见编程概念](https://kaisery.github.io/trpl-zh-cn/ch03-00-common-programming-concepts.html#常见编程概念)

## 3.1 [变量和可变性](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#变量和可变性)

声明常量使用 `const` 关键字

声明变量使用 `let` 关键字

通过 `mut`，允许把绑定到 `x` 的值从 `5` 改成 `6`。在一些情况下，你会想用可变变量，因为与只用不可变变量相比，它会让代码更容易编写。

除了防止出现 bug 外，还有很多地方需要权衡取舍。例如，使用大型数据结构时，适当地使用可变变量，可能比复制和返回新分配的实例更快。对于较小的数据结构，总是创建新实例，采用更偏向函数式的编程风格，可能会使代码更易理解，为可读性而牺牲性能或许是值得的。

Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性）：

```rust
const MAX_POINTS: u32 = 100_000;
```

在声明它的作用域之中，常量在整个程序生命周期中都有效，这使得常量可以作为多处代码使用的全局范围的值

### [隐藏（Shadowing）](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#隐藏shadowing)

定义一个与之前变量同名的新变量，而新变量会 **隐藏** 之前的变量。

隐藏与将变量标记为 `mut` 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 `let` 关键字，就会导致编译时错误。通过使用 `let`，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不变的。

`mut` 与隐藏的另一个区别是，当再次使用 `let` 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字。例如，假设程序请求用户输入空格字符来说明希望在文本之间显示多少个空格，然而我们真正需要的是将输入存储成数字（多少个空格）：

```rust
let spaces = "   ";
let spaces = spaces.len();
```

这里允许第一个 `spaces` 变量是字符串类型，而第二个 `spaces` 变量，它是一个恰巧与第一个变量同名的崭新变量，是数字类型。隐藏使我们不必使用不同的名字，如 `spaces_str` 和 `spaces_num`；相反，我们可以复用 `spaces` 这个更简单的名字。然而，如果尝试使用 `mut`，将会得到一个编译时错误，如下所示：

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

这个错误说明，我们不能改变变量的类型：





记住，Rust 是 **静态类型**（*statically typed*）语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如第二章的 [“比较猜测的数字和秘密数字”](https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) 使用 `parse` 将 `String` 转换为数字时，必须增加类型注解，像这样：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

这里 `: u32` 就是类型注解

## 3.2 [数据类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#数据类型)

### [标量类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#标量类型)

**标量**（*scalar*）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。

#### [整型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#整型)

**整数** 是一个没有小数部分的数字。我们在第二章使用过 `u32` 整数类型。该类型声明表明，它关联的值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 `i` 开头而不是 `u`）。表格 3-1 展示了 Rust 内建的整数类型。在有符号列和无符号列中的每一个变体（例如，`i16`）都可以用来声明整数值的类型。

表格 3-1: Rust 中的整型

| 长度    | 有符号  | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

每一个变体都可以是有符号或无符号的



另外，`isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

可以使用表格 3-2 中的任何一种形式编写数字字面值。注意除 byte 以外的所有数字字面值允许使用类型后缀，例如 `57u8`，同时也允许使用 `_` 做为分隔符以方便读数，例如`1_000`。

表格 3-2: Rust 中的整型字面值

| 数字字面值                    | 例子          |
| ----------------------------- | ------------- |
| Decimal (十进制)              | `98_222`      |
| Hex (十六进制)                | `0xff`        |
| Octal (八进制)                | `0o77`        |
| Binary (二进制)               | `0b1111_0000` |
| Byte (单字节字符)(仅限于`u8`) | `b'A'`        |

Rust 的默认类型通常就很好，数字类型默认是 `i32`：它通常是最快的，甚至在 64 位系统上也是。`isize` 或 `usize` 主要作为某些集合的索引。

#### [浮点型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#浮点型)

Rust 也有两个原生的 **浮点数**（*floating-point numbers*）类型，它们是带小数点的数字。Rust 的浮点数类型是 `f32` 和 `f64`，分别占 32 位和 64 位。默认类型是 `f64`，因为在现代 CPU 中，它与 `f32` 速度几乎一样，不过精度更高。

这是一个展示浮点数的实例：

文件名: src/main.rs

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

浮点数采用 IEEE-754 标准表示。`f32` 是单精度浮点数，`f64` 是双精度浮点数。

#### [数值运算](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#数值运算)

Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。下面的代码展示了如何在 `let` 语句中使用它们：

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;
}
```

这些语句中的每个表达式使用了一个数学运算符并计算出了一个值，然后绑定给一个变量。附录 B 包含 Rust 提供的所有运算符的列表。

#### [布尔型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#布尔型)

正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：`true` 和 `false`。Rust 中的布尔类型使用 `bool` 表示。例如：

```rust
fn main() {
    let t = true;

    let f: bool = false; // 显式指定类型注解
}
```

使用布尔值的主要场景是条件表达式，例如 `if` 表达式。在 [“控制流”（“Control Flow”）](https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html#control-flow) 部分将介绍 `if` 表达式在 Rust 中如何工作。

#### [字符类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#字符类型)

目前为止只使用到了数字，不过 Rust 也支持字母。Rust 的 `char` 类型是语言中最原生的字母类型，如下代码展示了如何使用它。（注意 `char` 由单引号指定，不同于字符串使用双引号。）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust 的 `char` 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 `char` 值。Unicode 标量值包含从 `U+0000` 到 `U+D7FF` 和 `U+E000` 到 `U+10FFFF` 在内的值。不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符” 可能与 Rust 中的 `char` 并不符合。第八章的 [“使用字符串存储 UTF-8 编码的文本”](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) 中将详细讨论这个主题。

### [复合类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#复合类型)

**复合类型**（*Compound types*）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

#### [元组类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#元组类型) tuple

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。**元组长度固定**：一旦声明，其长度不会增大或缩小。

我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。这个例子中使用了可选的类型注解：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

`tup` 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值，像这样：

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

程序首先创建了一个元组并绑定到 `tup` 变量上。接着使用了 `let` 和一个模式将 `tup` 分成了三个不同的变量，`x`、`y` 和 `z`。这叫做 **解构**（*destructuring*），因为它将一个元组拆成了三个部分。最后，程序打印出了 `y` 的值，也就是 `6.4`。

除了使用模式匹配解构外，也可以使用点号（`.`）后跟值的**索引**来直接访问它们。例如：

文件名: src/main.rs

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

这个程序创建了一个元组，`x`，并接着使用索引为每个元素创建新变量。跟大多数编程语言一样，元组的第一个索引值是 0。

#### [数组类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#数组类型) array

另一个包含多个值的方式是 **数组**（*array*）。与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，因为 **Rust 中的数组是固定长度的**：一旦声明，它们的长度不能增长或缩小。

数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 **允许** 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，你可能应该使用 vector。第八章会详细讨论 vector。

一个你可能想要使用数组而不是 vector 的例子是，当程序需要知道一年中月份的名字时。程序不大可能会去增加或减少月份。这时你可以使用数组，因为我们知道它总是包含 12 个元素：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

这里，`i32` 是每个元素的类型。分号之后，数字 `5` 表明该数组包含五个元素。

这样编写数组的类型类似于另一个初始化数组的语法：如果你希望创建一个每个元素都相同的数组，可以在中括号内指定其初始值，后跟分号，再后跟数组的长度，如下所示：

```rust
let a = [3; 5];
// 3 3 3 3 3 
```

##### [访问数组元素](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#访问数组元素)

数组是一整块分配在栈上的内存。可以使用索引来访问数组的元素，像这样：

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

## 3.3 [函数](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html#函数)

Rust 代码中的函数和变量名使用 *snake case* 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。这是一个包含函数定义示例的程序：

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust 中的函数定义以 `fn` 开始并在函数名后跟一对圆括号。大括号告诉编译器哪里是函数体的开始和结尾。

可以使用函数名后跟圆括号来调用我们定义过的任意函数。因为程序中已定义 `another_function` 函数，所以可以在 `main` 函数中调用它。注意，源码中 `another_function` 定义在 `main` 函数 **之后**；也可以定义在之前。**Rust 不关心函数定义于何处**，只要定义了就行。

### [函数参数](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html#函数参数)

函数也可以被定义为拥有 **参数**（*parameters*），参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。技术上讲，这些具体值被称为参数（*arguments*），但是在日常交流中，人们倾向于不区分使用 *parameter* 和 *argument* 来表示函数定义中的变量或调用函数时传入的具体值。

下面被重写的 `another_function` 版本展示了 Rust 中参数是什么样的：

在函数签名中，**必须** 声明每个参数的类型。这是 Rust 设计中一个经过慎重考虑的决定：要求在函数定义中提供类型注解，意味着编译器不需要你在代码的其他地方注明类型来指出你的意图。

当一个函数有多个参数时，使用逗号分隔，像这样：

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

这个例子创建了有两个参数的函数，都是 `i32` 类型。


### [包含语句和表达式的函数体](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html#包含语句和表达式的函数体)

函数体由一系列的语句和一个可选的结尾表达式构成。目前为止，我们只介绍了没有结尾表达式的函数，不过你已经见过作为语句一部分的表达式。因为 Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。

实际上，我们已经使用过语句和表达式。**语句**（*Statements*）是执行一些操作但不返回值的指令。表达式（*Expressions*）计算并产生一个值。让我们看一些例子：

使用 `let` 关键字创建变量并绑定一个值是一个语句。在列表 3-1 中，`let y = 6;` 是一个语句。

```rust
fn main() {
    let y = 6;
}
```

列表 3-1：包含一个语句的 `main` 函数定义

函数定义也是语句，上面整个例子本身就是一个语句。

语句不返回值。因此，不能把 `let` 语句赋值给另一个变量，比如下面的例子尝试做的，会产生一个错误：

```rust
fn main() {
    let x = (let y = 6);
}
```

当运行这个程序时，会得到错误：

`let y = 6` 语句并不返回值，所以没有可以绑定到 `x` 上的值。这与其他语言不同，例如 C 和 Ruby，它们的赋值语句会返回所赋的值。在这些语言中，可以这么写 `x = y = 6`，这样 `x` 和 `y` 的值都是 `6`；Rust 中不能这样写。

表达式会计算出一些值，并且你将编写的大部分 Rust 代码是由表达式组成的。考虑一个简单的数学运算，比如 `5 + 6`，这是一个表达式并计算出值 `11`。表达式可以是语句的一部分：在示例 3-1 中，语句 `let y = 6;` 中的 `6` 是一个表达式，它计算出的值是 `6`。函数调用是一个表达式。宏调用是一个表达式。我们用来创建新作用域的大括号（代码块），`{}`，也是一个表达式，例如：

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

这个表达式：

```rust
{
    let x = 3;
    x + 1
}
```

是一个代码块，它的值是 `4`。这个值作为 `let` 语句的一部分被绑定到 `y` 上。注意结尾没有分号的那一行 `x+1`，与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。在接下来探索具有返回值的函数和表达式时要谨记这一点。

### [具有返回值的函数](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html#具有返回值的函数)

函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（`->`）后**声明它的类型**。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。使用 `return` 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。这是一个有返回值的函数的例子：

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
```

在 `five` 函数中没有函数调用、宏、甚至没有 `let` 语句——只有数字 `5`。这在 Rust 中是一个完全有效的函数。注意，也指定了函数返回值的类型，就是 `-> i32`。尝试运行代码；输出应该看起来像这样：

```text
The value of x is: 5
```

`five` 函数的返回值是 `5`，所以返回值类型是 `i32`。让我们仔细检查一下这段代码。有两个重要的部分：首先，`let x = five();` 这一行表明我们使用函数的返回值初始化一个变量。因为 `five` 函数返回 `5`，这一行与如下代码相同：

```rust
let x = 5;
```

其次，`five` 函数没有参数并定义了返回值类型，不过函数体只有单单一个 `5` 也没有分号，因为这是一个表达式，我们想要返回它的值。

让我们看看另一个例子：

```rust
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

运行代码会打印出 `The value of x is: 6`。但如果在包含 `x + 1` 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

主要的错误信息，“mismatched types”（类型不匹配），揭示了代码的核心问题。函数 `plus_one` 的定义说明它要返回一个 `i32` 类型的值，不过语句并不会返回值，使用空元组 `()` 表示不返回值。因为不返回值与函数定义相矛盾，从而出现一个错误。在输出中，Rust 提供了一条信息，可能有助于纠正这个错误：它建议删除分号，这会修复这个错误。

## 3.4 [注释](https://kaisery.github.io/trpl-zh-cn/ch03-04-comments.html#注释)

这是一个简单的注释：

```rust
// hello, world
```

在 Rust 中，注释必须以两道斜杠开始，并持续到本行的结尾。对于超过一行的注释，需要在每一行前都加上 `//`，像这样：

不过你更经常看到的是以这种格式使用它们，也就是位于它所解释的代码行的上面一行：

```rust
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```

Rust 还有另一种注释，称为文档注释，我们将在 14 章的 “将 crate 发布到 Crates.io” 部分讨论它。