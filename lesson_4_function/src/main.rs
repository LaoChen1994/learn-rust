fn main () {
    println!("this is main function");
    another_function();
    let y = {
        let x = 3;
        addOne(x)
    };

    println!("y => {}", y)
}

fn addOne(num: u32) -> u32 {
    num + 1
}

fn another_function() {
    println!("this is another function, {}", get_params(3, 4))
}

fn get_params  (a: i32, b: i32) -> i32 {
    return a + b;
}

