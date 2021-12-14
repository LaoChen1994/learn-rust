fn main() {
    let number = 5;
    if number < 5 {
        println!("number is less than 5")
    } else if number == 5 {
        println!("number is equal to 5")
    } else {
        println!("number is greater than 5")
    }

    let a = if number < 3 { 10 } else { 20 };

    println!("a -> {}", a);

    loopExample();
    loopExit();
    let rlt: u32 = loop_return_value(30);
    println!("rlt -> {}", rlt);

    while_loop();
    for_loop();
    let fibonacci_rlt = fibonacci(10);

    println!("rlt -> {}", fibonacci_rlt)
}

fn loopExample() {
    let mut b = 1;
    loop {
        println!("b =>{}", b);
        b += 1;

        if b > 5 {
            break;
        }
    }
}

fn loopExit() {
    let mut count = 0;
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

        count += 1;
    }
}

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

fn loop_return_value(num: u32) -> u32 {
    let mut count = 1;

    let loop_value = loop {
        count += 1;
        if count == num {
            break count * 2;
        }
    };

    return loop_value;
}

fn while_loop() {
    let mut number = 3;

    while number > 0 {
        println!("number -> {}", number);
        number -= 1;
    }

    println!("while loop end");
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5, 6];

    for elem in a {
        println!("loop for -> {}", elem);
    }

    for elem in (1..=100) {
        println!("item -> {}", elem)
    }
}

fn fibonacci(number: u32) -> u32 {
    if number == 1 {
        return 1;
    } else if number == 2 {
        return 1;
    } else {
        return fibonacci(number - 2) + fibonacci(number - 1)
    }
}
