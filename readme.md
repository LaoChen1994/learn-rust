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

### 4. 一个石头剪刀布的游戏

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