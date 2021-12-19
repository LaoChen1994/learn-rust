fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("word -> {}", word);

    let hello = &s[..5];
    let world = &s[6..];

    println!("hello -> {}, world -> {}", hello, world);
    s.clear(); 


    let mut s2 = "Hello World";

    s2 = "Rust Hello World";

    let s3 = &s2[..5];
    let s4 = get_word(&s2[..11]);

    println!("s3 -> {}, s4 -> {}", s3, s4);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}