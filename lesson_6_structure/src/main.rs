struct User{
    active: bool,
    username: String,
    age: u32,
    email: String,
    sign_in_count: u32
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    let mut tom = User {
        active: true,
        username: String::from("Tom"),
        email: String::from("tom@163.com"),
        age: 18,
        sign_in_count: 1
    };

    let jerry = build_user(String::from("jerry"), 18, String::from("jerry@163.com"));

    tom.email = String::from("loveJerry@163.com");

    let dog = User {
        username: String::from("Dog"),
        ..jerry
    };

    println!("user email -> {}", tom.email);
    // println!("jerry name -> {}, jerry email -> {}", jerry.username, jerry.email);
    println!("dog name -> {}, dog email -> {}", dog.username, dog.email);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let Color(black_r, black_g, black_b) = black;

    let origin = Point(0, 0, 0);
    let x = origin.0;

    println!("r -> {}, g -> {}, b -> {}", black_r, black_g, black_b);
}

fn build_user(username: String, age: u32, email: String) -> User {
    User {
        username,
        email,
        age,
        active: true,
        sign_in_count: 1
    }
}