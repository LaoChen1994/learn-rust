# Rust学习笔记

## 0. 写在前面

### 0.1 相关资料

1.  [Rust Programing Language](https://doc.rust-lang.org/book/title-page.html)
2.  [源码地址](https://github.com/LaoChen1994/learn-rust)https://doc.rust-lang.org/book/title-page.html)

### 0.2 本章大纲

1. rust环境的安装
2. 第一个hello rust程序
3. cargo的简单使用（包的创建，打包，运行，检测更新）
4. toml文件一些配置项的使用

## 1. 环境安装

### 1.1 windows安装

+ 步骤： 

  + C++环境安装：根据文档中，windows可以先进入[visual-cpp-build-tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)下载对应的工具，安装完成后，重启电脑即可
  + Rust安装：[rust下载](https://forge.rust-lang.org/infra/other-installation-methods.html)选择对应的版本下载即可
  + 测试：使用`rustc --version`能看到对应的版本号就是安装成功啦

### 1.2 MacOS安装过程

+ 步骤：
    + 运行安装命令
    ```bash
        # 运行命令
        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
    + 查看提示，成功安装后，命令行会提示 **Rust is installed now. Great!**


## 2. Hello Rust

### 2.1 第一个Rust代码

```rust
fn main() {
    println!("hello Rust");
}
```

+ 关键分析
  + 第一行的fn用于声明一个函数
  + main函数在rust函数中是作为第一行代码执行的
  + println后面的`!`，这是macro的宏，如果加了`!`代表你调用的是rust的宏而不是普通函数
  + `;`代表该行语句结束

### 2.2 编译rust

```bash
rustc ./helloRust.ts
```

**结果**: 编译之后我们会拿到两个文件`helloRust.exe`和`helloRust.pdb`

其中这个`exe`文件就是在windows上直接可以执行的文件


### 2.3 和动态语言的区别

动态语言：js，python，ruby等需要我们在运行的平台安装对应的解释器，通过解释器将代码转换为字节码，通过字节码解释器来读取并执行字节码

编译型语言：将代码编译成二进制可执行文件，平台上可以不安装任何解释器


## 3. Hello Cargo

### 3.1 cargo是什么

  1. cargo是一个构建系统和包管理工具
  2. 用来管理rust工程和处理许多任务（构建代码、下载包）
  3. cargo在安装rust的环境时，会自动安装，所以这里的话，直接调用
     ```bash
      # 可以直接看到对应的cargo版本
      cargo --version
     ```
  > 类似nodejs中的npm，通过scrips来帮我们执行相应脚本，通过install来安装包

### 3.2 cargo工程的创建

#### 3.2.1 创建

使用`cargo new [projectName]`来创建一个新的工程
```bash
cargo new hello_cargo
```

![](./images/02-hello-rust.png)

#### 3.2.2 目录解析

创建的目录包括：

+ cargo.toml：用于定义包和信息`[package]`和依赖`[dependencies]`，其他还有别的配置如下：

  ![](.\images\03-hello-cargo-toml.png)

+ 源文件目前是放在src的目录下的

#### 3.2.3 toml文件中需要注意的点

`[package]`下面的一些配置:

+ build：用于指定build来指定这个包用源码的方式编译

  > 比如在rust中引用了比如C的库，这些东西编译就放在一个build.rs中，会通过源码的方式进行编译

+ include和exclude：用来指定那些包需要被打包或发布，可以通过`carge package --list`来查看

+ publish: 用来指定包是否发布到包仓库，==保护包的私有性==



### 3.3 使用cargo构建和跑rust工程

#### 3.3.1 构建rust工程

```bash
cargo build
```

#### 3.3.2 运行cargo工程

构建成功之后的文件都在==target/debug/hello_cargo==下，直接运行目录下的`exe`文件也可以运行

```bash
.\target\debug\hello_cargo.exe
```

也可以通过cargo来运行对应的exe打包文件

```bash
cargo run
```

在执行cargo run的时候有两个比较有意思的点：

1. 源文件没有变化，直接执行cargo build出来的文件

2. 如果源文件产生了变化，会先编译，然后再执行

   ![](.\images\03-cargo-build.png)

3. 这里可以通过`cargo check`来检验当前版本的编译后的代码，是否还有更新为编译的版本

#### 3.3.3 构建发布版本

通过`--release`来指定构建的版本为发布版本，通过`release`来发布，会做很多优化，使rust代码运行更快，但也会让==构建时间更长==，另外构建发布的时候，这个地方生成的文件是在target下的。

## 4. 一个石头剪刀布的游戏

### 写在前面

我看原书里面把这一张放在安装之后，但是我觉得有点困难了，因为本章涉及到的知识点包括：

1. 模块的引入与安装
2. 标准输入输出的使用
3. 变量定义
4. 变量转换
5. 模块中方法的使用
6. trait、modules、Struct、Function概念
7. 循环的使用

**我的想法**对于一个rust的小白来说，其实我对基本的语法都不太清楚，看这一章确实只能看个热闹，但是作为一个小白这章的笔记还是先拿上来，很多都是自己根据js迁移过来的一些理解，可能并不正确，后期学完相应的知识后，应该也会做相应的调整。

### 4.1 需求分析



+ 这里其实想做的一件事是：
  + 随机产生一个数字
  + 让用户输入一个数字
  + 比对两个数字的结果
  + 根据对比结果输出相应的对比结果，一直循环上述`步骤二`和`步骤三`直到用户输入和随机数相同
  + 猜中数字，结束循环

既然看起来很简单的需求，那么就来coding把

### 4.2 随机生成一个数

#### 4.2.1 模块的安装

可以通过引入`rand`模块的方式，来实现随机数的创建，由于这里rand是个第三方模块，所以需要我们先安装。

**安装步骤**：

1. 在catgo.toml的packages依赖中，增加rand模块，版本为0.8.3
2. 使用cargo update更新下镜像
3. 使用cargo run的时候模块会被自动安装

**换源步骤**（[参考文档](https://www.cnblogs.com/baby123/p/13260212.html)）

1. 在根目录下的.cargo创建config文件
2. 根据上述网址中对应的内容复制过去就行
3. cargo update一下



#### 4.2.2 代码编写

```rust
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number_1 = rand::thread_rng().gen_range(1..10);
}
```

**FAQ**：

*Q1*: 为什么use rand::Rng不导入的时候，这个gen_range就报错？

*A1*: 因为`Rng`这个==trait==定义了随机数方法的实现，所以在这个地方现有了实现的引用，然后这边调用gen_range才能调用



*Q2*: `1..10`写法的意思？

*A2*:这里有两种写法`1..10`和`1..=10`，前者是`[1, 10)`这个区间，后者是`[1, 10]`这个区间



*Q3*:如何去看这些函数怎么调用？

*A3*: 可以通过`cargo doc --open`方法来打开帮助文档根据文档使用对应的包



*Q4*：如何调用该函数方法？

*A4*: 使用`::`的语法



> 这里，我们使用rand::tread_rng()生成了一个区间1~10间的数字

### 4.3 获取用户输入

#### 4.3.1 代码编写



```rust
use rand::Rng;
// 使用第三方库std中的io模块
// 在js里面相当于 import { io } from 'std' 这个类比是我自己理解的不一定正确
use std::io;

fn main() {
    // 省略
    println!("Please input your guess");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Fail to read line");
}
```

**FAQ**：

*Q1*: `mut`是什么意思？有mut和没有有什么区别？

*A1*: 在rust中所有变量默认是immutable的，所以你应该懂了

​		有mut定义：代表参数是mutable的

​		无mut定义：代表参数是immutable的



*Q2*: 实例如何创建？

*A2*:  通过类`::`方法的方式，表示调用了类的一个方法，这里调用的是一new的一个构造函数，new我理解是rust中一个通用的名称，用来创建某个类新的值



*Q3*：这里的`&mut guess`是什么意思？

*A4*: 在rust中，这里通过`&`来表明拿的数，是对应数在内存中的**引用**，`可以保证在代码中多个地方拿同一个值，不需要拷贝多份`，这是一种非常复杂的特性。



*Q4*: `expect`是用来干什么的

*A3*: 在rust中，很多方法返回的类型是Result，这个类型的意思是，如果成功就返回一个Ok的枚举值，如果失败就返回一个Error的枚举值，那么这里失败的时候，如果我们expect传入了一个字符串，失败的msg就会是我们传入的字符串，可用来定位错误信息



*Q5*：Result中的几种形式

*A5*：

类型一：支持通过except方法，当执行结果为成功的时候，直接跳过，错误的时候抛出expect中给出的错误信息

类型二：通过Ok和Err两个变量，来对成功和失败分别做操作，**但是目前实测这种用法，就是Err和Ok的写法要配合match关键词才能生效**具体例子如下:

```rust
fn main {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", guess);
        }
        Err(error) => println!("error: {}", error),
    };
}
```

> 我理解是，类似Promise一样，成功通过then的回调做一些操作，失败通过catch来对异常做一些操作，当然只是这个回调的概念，便于理解，其实两者是完全不能划等号做对比的~

### 4.4 变量类型转换

#### 4.4.1 类型比较

目前，因为我们从命令行中获取的guess类型是String，但是我们实际通过rand生成的数字是Number类型，这两个类型不匹配是无法比较的，所以必须进行类型的转换，**静态语言对于类型定义的严谨性**



#### 4.4.2 代码编写

```rust
fn main {
    // 省略
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        // 这个continue是为了后续做循环的时候，直接跳过后续操作用的
        // 先忽略
        Err(_) => continue,
    };
}
```

**FAQ**：

*Q1*: 如何定义一个变量的类型？

*A1*: 通过类似TS中的`:`告诉rust，这个数字变量的类型，这里的`u32`代表的是一个32位的变量，类似一个数字变量的声明。



*Q2*: 为什么use rand::Rng不导入的时候，这个gen_range就报错？

*A2*: 因为`Rng`这个==trait==定义了随机数方法的实现，所以在这个地方现有了实现的引用，然后这边调用gen_range才能调用

【理解】[参考文章](https://zhuanlan.zhihu.com/p/106251259)（==这个地方对于模块化的理解不一定对，后续会改==）

现在我看doc中有这么几个模块的概念：

modules：模块，这里就是我们用到的库的概念，比如rand、std等，这些本身在声明的时候是通过`mod`来声明的，引入的如果是子模块，也可以通过`use mod::sub_mod`这样的方式来引入子模块

traits：是一个**方法的集合**（这个是我自己的理解），因为在[Rust——理解trait （一）](https://zhuanlan.zhihu.com/p/237560100)这里给出的例子中，其实是在实现上会更加灵活，即可以在特定的条件下表现出特定的特性，更加灵活

Functions：通过`somemod::method`的方法来引用，相应模块下的方法



*Q3*: trim和parse的用法

*A3*: trim用于去除前后的空格和js中的trim用法一样，parse用于将string转换成对应的数字类型。parse这里返回的也是一个`Result`类型，这里看上去是不是特别像Promise.then和catch呢。（说的很小声，很心虚）

### 4.5 比较

#### 4.5.1 match关键字

对比多个数值，类似switch...case的这种语法，在rust中和match有点类似，其作用是，在某种场景下可以返回某些值。后续应该会详细的介绍（至少文中是这么说的）

其具体语法是: 

```rust
// 这里的xxx是一个操作返回的结果
match xxxx() {
    A1(a1) => XXX,
    A2(a2) => XXX,
    A3(a3) => XXX,
}
```



#### 4.5.2 代码编写

```rust
use std::cmp::Ordering;

fn main () {
    // 省略上述代码
    // 这里的&secret_number_1只是为了取他的引用值
    match guess.cmp(&secret_number_1) {
        // 对比结果是less的话
        Ordering::Less => println!("Too Small!"),
        // 对比结果是greater的话
        Ordering::Greater => println!("Too Big!"),
        // 相等怎么操作
        Ordering::Equal => {
            println!("You win!");
            // 这里是做loop循环的时候，猜对了直接跳出循环用的
            break;
        }
    }
}
```

**FAQ**：

*Q1*: Ordering是啥？

*A1*:只是一个枚举值~但是也要引入才能用



### 4.6 完整代码

```rust
// 使用第三方库std中的io模块
// 在js里面相当于 import { io } from 'std' 这个类比是我自己理解的不一定正确
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 这里有个语法没弄明白
    // 为什么use rand::Rng不导入的时候，这个gen_range就报错
    // 不知道这个地方的导入关系是咋样
    // 后期要再看看
    // 文中写的是因为Rng是定义了随机数方法的实现
    // 所以在这个地方现有了实现的引用，然后这边调用gen_range才能调用

    // 这种写法指的是给与了一个range，[1, 10)
    let secret_number_1 = rand::thread_rng().gen_range(1..10);
    // 这种写法是闭区间[1, 10]
    // let secret_number_2 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess");
        // 变量定义
        // 和js不同的是这里有mut
        // 在rust中所有变量默认是immutable的，所以你应该懂了
        // 有mut定义：代表参数是mutable的
        // 无mut定义：代表参数是immutable的

        // 通过类::方法的方式，表示调用了类的一个方法
        // 这里调用的是一new的一个构造函数
        // new我理解是rust中一个通用的名称，用来创建某个类新的值
        let mut guess = String::new();

        // 调用io模块中的stdin方法
        // 这里也可以直接使用std::io::stdin
        match io::stdin()
            // 调用read_line方法
            // &代表是引用类型，给引用赋值，代码中的多个部分接入同一个引用就是同一个值
            // 因为之前定义的guess是mutable的所以这里要街上mut的修饰
            // 这里一定要是mut的类型，因为read_line要求就是mutable的
            .read_line(&mut guess) {
                Ok(n) => {
                    println!("{} bytes read", n);
                    println!("{}", guess);
                }
                Err(error) => println!("error: {}", error),
            };
            // 在expect的实现中，成功执行就会跳过，执行失败会返回expect传入的error message
            // expect用于判断上一个操作是否成功
            // 这个expect是io::Result定义的
            // .expect("Fail to read line");
        
        // 这里的parse的返回值也是一个Result
        // 也是可以通过expect来怕段是否满足了转换的预期
        // 转换不成功就报错
        // 通过类似ts的：来告诉rust，这个变量期待的类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number_1) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```

### 4.7 效果

![](.\images\guess.gif)



## 5. 通用的编程概念

### 5.1 变量和可变性



#### 5.1.1 概念

变量（variables）:用来存储值的一个地方（Storing values with Variables）,默认是immutable

常量（constants）:仅可被赋值一次的变量

可变性（mutability）：通过mut来定义，代表值可变的变量（类似let，可以被重新赋值）

【错误写法】

```rust
fn main() {
    // let mut a = 6;
    let a = 5;
    println!("the variable is {}", a);
    // 这里rust会报错
    a = 6;
    println!("the variable is {}", a);
}
```

【正确写法】当使用mut的时候，我们允许变量的值被更改

```rust
fn main() {
    // 通过mut来区分类似const和let的概念
    let mut a = 5;
    println!("the variable is {}", a);
    a = 6;
    println!("the variable is {}", a);
}

```

> It’s important that we get compile-time errors when we attempt to change a value that we previously designated as immutable because this very situation can lead to bugs. If one part of our code operates on the assumption that a value will never change and another part of our code changes that value, it’s possible that the first part of the code won’t do what it was designed to do. The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the value only *sometimes*
>
> 大体的意思就是，为了避免bug和代码中非预期的成分如果我们一个变量定义的时候假设他是不可变的，那么其实如果对其二次赋值，其实是非符合预期的，所以编译阶段会报错



#### 5.1.2 变量和常量的区别

**Immutable变量**和**常量**之间的区别：

+ 【区别】mut关键词不能用来修饰const的变量，const一直是不可变的

+ 【区别】const关键字必须要注释类型，就是需要给出类型的说明

+ 【区别】const关键词可以在任意的作用域进行声明，包括全局作用域

+ 【区别】const的赋值，只能是一个常数的表达式，不能是只有在运行时才能被计算出的运行后的结果（就是不能是一个根据运行时环境参数不一样，动态计算得到的动态的结果）

+ 【非区别】建议定义常量时，通过`大写字母`和`_`的方式来定义，变量定义时建议使用小写，并且在单词间通过下划线的方式进行隔离的方式，**规范有效的命名方式让我们在写代码时对变量的识别会更加准确，利于代码的维护**。

  ```rust
  // 全局的声明是被允许的
  // 区别二：需要通过 `: `来告诉rust这个常量对应的类型
  // 区别三：可以在全局定义
  const C_C: &str = "_global cc";
  // 区别四：只能是一个常数计算结果的表达式
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  // 区别三：这个里使用let会报错，不能再全局定义
  // expected item, found keyword `let`
  let dd = 32;
  
  fn main() {
      const B_BIT: u32 = 3;
      println!("the constant is {}, {}", C_C, B_BIT);
  }
  
  ```



#### 5.1.3 变量覆盖（阴影）

> TIPS： 这里我其实更倾向于将原文中的shadowing翻译成变量的覆写和作用域，因为看上去和js的作用域真的很像，当然可能会不符合rust中的叫法

**作用域的定义和特性**：通过`{}`围起来的代码块是单独的作用域，在内部作用域定义的变量，会覆盖掉上层作用域中有的参数，和JS表现相近

**变量的覆写**：通过`let`定义同名的变量来覆写上面已经通过`let`定义的变量，如果是变量的覆写是不要求前后同名变量类型相同的，但是`mut`变量的修改，需要前后有一定的类型

**注意点**：`let`定义的变量通过`let`在覆写和是不是`mut`的变量其实没有关系，因为这里是新创建一个变量，而不是对已经不可变的变量进行赋值（编译时不会报错），所以是两个维度的事情

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    
    // 之前是str类型，后面的number类型
    // 如果是定义的x_str 为mut
    // 后续通过赋值修改是不被允许的
	let x_str = "   ";
    let x_str = x_str.len();

    {
        let x = x * 2;
        // 这里的x是12
        println!("The inner x Number is {}", x);
    }

    // 这里的x是12
    println!("The x Number is {}", x);
}
```



### 5. 2 数据类型

在rust中的数据类型主要包括两种：标量（scalar）和组合类型（compound），因为rust本来是一个**静态类型语言**，所以需要在编译时告诉rust所有变量的类型，所以在类型转换的时候，也需要告诉编译器，需要转成的类型声明。

#### 5.2.1 标量类型

标量类型单表的是单一的值，不是组合的值，具体的类型如下：

+ 整型（integers）
+ 浮点型（float）
+ 布尔类型
+ 字符串类型

---

##### 整型类型

| 长度    | 有符号  | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

**注意点**

1. 有符号通过`i`开头，无符号通过`u`开头
2. 整型的区间为，有符号为 -2^n-1^ ~ 2^n-1^ - 1 ，无符号为0~2^n^ - 1
3. `arch`代表的是当前操作系统架构最大的位数，比如x64就是`u64`, `i64`
4. 表示不同进制的方法
   1. 十进制：1_000,1000都是十进制
   2. 十六进制：0xff
   3. 八进制：0o77
   4. 二进制：0b1111_0000
   5. 字节：b'A'
5. 对于溢出的处理，在编译时（debug模式）会报错，引起你的注意，但是如果是`--release`模式下并不会报错，他会把这个值置为你超过部分的值，这是因为他会自动在外部填充两位，来接收对应的值，所以虽然不会报错，但是也需要我们对加减和最大最小值来做一下校验，主流的库中都会有相应的方法：
   1. `wrapping_*`来做数的运算
   2. `checked_*`来判断是否有溢出，`None`返回值代表溢出了
   3. `overflowing_*`返回值和一个布尔值来判断是否溢出了
   4. `saturating_*`方法来代表改值在定义的区间内

---

##### 浮点类型

浮点类型的定义只有两个方法`f32`和`f64`

```rust
fn main () {
    // 数值类型
    // 浮点除法
    // 如果不是2.0 / 3.0返回的是0，默认为整型
    // let float_num = 2 / 3;  
    
    let floorded:f64 = 2.0 / 3.0;
    // 这样是0.66666
    println!("the float is {}", floorded)
}
```

**注意点**

1. 默认是`f64`为了保证浮点更高的精度
2. 如果是`2 / 3`的形式返回的是整型，也就是0，如果需要得到小数，那么一定是`2.0 / 3.0`保证右边都是浮点数，最后计算结果才会是浮点数

---

##### 布尔类型

类型通过`bool`来进行定义即可，用于条件控制，这个在`if`的控制流中会用到，和所有语言的都类似

```rust
fn main () {
    let f: bool = false;
}
```

---

##### 字符串类型

rust中的字符串类型是最原始的字母类型,包含中文、日文、韩文表情等字符（Unicode Scalar比ASCII码更丰富）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```



#### 5.2.2 组合类型

##### 元组类型

元组类型是将多个其他类型放到一个组合类型中，元组的特点：

+ 固定的长度
+ 一次性的定义
+ 长度不可收缩
+ 逗号分隔的列表
+ 通过`()`来定义

```rust
fn main () {
    // 组合类型
    // 元组
    let tup: (i32, i8, f64) = (500, 22, 3.14);
    let (t1, t2, t3) = tup;
    
    println!("tup -> {}, {}, {}", tup.0, tup.1, tup.2);
    println!("tup -> {}, {}, {}", t1, t2, t3);
}
```

**注意点**：

1. 可以通过`.`语法来进行对应位置元组数据的获取
2. 可以通过解构的方式来回去对应位置上的值



---

##### 数组类型

数组是同一种类型的元素的集合，和元素的不同是**必须所有元素具有相同的类型**，其特点：

+ 元素类型相同
+ 通过`[]`来定义

```rust
fn main () {
    // 数组
    let array1 = [1, 2, 3, 4, 5];
    let array2: [i32;5] = [1, 2, 3, 4, 5]; 
    // 前两种比较好理解，这种的意思是生成一个整型的数组，长度为5，默认值为3
    let array3 = [3;5];
    
    println!("str -> {}", array2[1]);
}
```

**注意点**

1. 接入数组的方式通过索引来接入，例如`array2[1]`
2. 如果这个index是动态生成的，那么我们一定要判断index是否小于数组的长度，不然的话编译的时候不会报错，因为你的index是动态生成的，但是在运行过程中很有可能index超过了最长的数组长度报错（**越界报错**）

### 5.3 函数

#### 5.3.1 函数定义方式

函数的定义方式：

1. 通过`fn`关键字进行定义
2. 在`fn`后跟上相应的函数名
3. 通过`{}`来 定义函数体

```rust
fn main () {
    println!("this is main function");
    another_function();
}

fn another_function() {
    println!("this is another function, {}", get_params(3, 4))
}

fn get_params  (a: i32, b: i32) -> i32 {
    return a + b;
}
```

**注意点**：

1. 函数的输入可定义类型，输入通过`:`来定义类型
2. 默认的函数是不带返回值的，如果带上返回值需要通过`->`指定类型，不然会报错



#### 5.3.2 函数体包括声明和表达式

Rust是一个基于表达式的语言，表达式和声明的定义在这里就不叙述了，百度肯定比我专业。

**注意点**

1. 声明并不会返回值，所以不支持JS中的那种连等式赋值，目前是不被允许的~

```rust
fn main() {
    // 这里会报错`let` expressions in this position are experimental
    let x = (let y = 6);
}
```

2. 在rust中表达式是不需要包含尾分号的，一旦你使用了尾分号就把表达式变成了声明

```rust
fn main () {
    println!("this is main function");
    another_function();
    let y = {
        let x = 3;
        // 这里没有尾分号
        // 所以是个表达式，Y -> 4
        // 如果加上尾分号，就会报错，因为他其实还是没有返回值
        x+1
    };

    println!("y => {}", y)
}
```

3. 可以将函数的返回值和表达式结合，这样函数会变得更加简洁

   ```rust
   fn main () {
       println!("this is main function");
       another_function();
       let y = {
           let x = 3;
           addOne(x)
       };
   
       // y => 4
       println!("y => {}", y)
   }
   
   fn addOne(num: u32) -> u32 {
       num + 1
   }
   ```



### 5.4 注释

注释主要通过`//`来进行注释，如果是多行的话，就是每一行之气都需要通过`//`来进行注释

### 5.5 控制流

基本上所有的编程语言都逃不开相关的控制流的语法，例如`if`,循环`loop`等，这一小节我们一起来看看rust中的循环和判断语法

####  5.5.1 if表达式

和其他语言中的if表达式一样，通过`if`关键字后面的声明，指定条件匹配后的操作，同样也支持`else`和`else if`

```rust
fn main() {
    let number = 5;
    if number < 5 {
        println!("number is less than 5")
    } else if number == 5 {
        println!("number is equal to 5")
    } else {
        println!("number is greater than 5")
    }
}
```

**注意点**

1. `if`作为表达式，可以在let中使用达到三元表达式的效果

   ```rust
   fn main() {
       let number = 5;
       let a = if number < 3 {10} else {20};
   
       println!("a -> {}", a)
   }
   ```

2. `1`中这种表达式要可以使用，有个前提就是每个表达式返回值的类型，需要相同，不然在编译时，rust编译器会报错



#### 5.5.2 使用Loop来实现重复

rust中提供了三种循环的方式，包括`loop`, `while`和`for`，接下来看看每一种的用法和写法



##### loop循环

loop循环的特点是，不会主动结束循环，直到使用人手动停止（调用`break`指令）或者直接运行`ctrl + C`才会停止该循环，具体代码示例如下：

```rust
fn main() {
    let mut b = 1;
    loop {
        println!("b =>{}", b);
        b += 1;

        if b > 5 {
            break;
        }
    }
}
```

**注意点**：

1. 和其他语言一样，rust中提供了`break`和`continue`关键字，用来停止循环和跳过后续代码这两个操作

2. 如果需要退出指定的循环，可以将loop的的返回值拿到，并通过break来中中断

   ```rust
   fn loopExit() {
       let mut  count = 0;
       'loop_rlt: loop {
           println!("count -> {}", count);
           let mut inner_count = 10;
           
           loop {
               println!("inner count ->{}", inner_count);
   
               if inner_count == 8 {
                   break;
               }
   
               if inner_count == 9 && count == 2 {
                   break 'loop_rlt;
               }
               
               inner_count -= 1;
           }
   
           count +=1;
       }
   }
   // 运行结果
   // count -> 0
   // inner count ->10
   // inner count ->9
   // inner count ->8
   // count -> 1
   // inner count ->10
   // inner count ->9
   // inner count ->8
   // count -> 2
   // inner count ->10
   // inner count ->9
   ```

3. loop返回的值需要通过`'` 加上变量名加上`:`来标注这是一个循环标记，这是一个规范，例如上述的`'loop_rlt`，这样通过`continue`和`break`来退出和继续循环

4. 通过`let`和`break`可以获取break最后我们想要输出的函数值

   ```rust
   fn loop_return_value(num: u32) -> u32 {
       let mut count = 1;
   
       let loop_value = loop {
           count+=1;
           
           if count == num {
               break count * 2;
           }
       };
   
       return loop_value
   }
   
   fn main () {
       let rlt: u32 = loop_return_value(30);
       // rlt -> 60
       println!("rlt -> {}", rlt);
   }
   ```

##### while循环

`while`循环和`loop`中，一样可以通过`break`来进行中断，其写法也和我们熟悉的其他语言中的`while`相似

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

**注意点**：

1. 如果是使用`while`循环遍历数组等有界的结构，可能需要对动态生成的index做出判断

##### for循环

**for...in循环**

通过while循环来便利数组可能会产生越界的问题，为了避免越界可以通过`for...in`**的方式来进行**遍历，便不会有越界的问题，因为在while中我们控制index是动态的不稳定。

```rust
fn for_loop () {
    let a = [1, 2, 3, 4, 5, 6];

    for elem in a {
        println!("loop for -> {}", elem);
    }
}
```

**利用Range和for..in的组合来进行循环**

通过`for...in`除了实现对集合类型的数据结构进行遍历，如果我们想要实现对while那样的多次循环，我们可以使用`range`加`for...in`循环的方式

```rust
fn for_loop () {
    for elem in (1..100) {
        println!("item -> {}", elem)
    }
    // item -> 1
    // item -> 2
    // ...
    /// item -> 99
}
```

> 上述方式因为是1..100所以区间是[1, 100)，如果想要双闭的区间，写法应该是(1..=100)



#### 5.5.3 练习题

**Fibonacci**

```rust
fn fibonacci(number: u32) -> u32 {
    if number == 1 {
        return 1;
    } else if number == 2 {
        return 1;
    } else {
        return fibonacci(number - 2) + fibonacci(number - 1)
    }
}
```



