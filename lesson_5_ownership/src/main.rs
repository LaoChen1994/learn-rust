fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("s -> {}", s);
    
    // 没有增加字符串长度的办法
    let a = "hello";

    let x= 5;
    let y = x;

    println!("x -> {}, y -> {}", x, y);

    let s2 = s;
    let s3 = s2.clone();
    println!("s2 -> {}, {}", s2, s3);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("x -> {}", x)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here
