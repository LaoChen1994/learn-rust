fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    // add_str(&mut s1);

    s1.push_str(", world");

    println!("s1 -> {}", s1);

    // let mut s2 = &mut s1;
    // println!("s2 -> {}", s2);
    
    {
        let s2 = &mut s1;
        println!("s2 -> {}", s2);
    }

    let s2 = &s1;
    let s3 = &s1;
    println!("the str is {}, {}", s2, s3);

    let mut s4 = &mut s1;


    s4.push_str(", rust");

    // println!("The length of '{}' is {}", s1, len);

    println!("the str -> {}", s4);


    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_str(s: &mut String) {
    s.push_str(", rust");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}