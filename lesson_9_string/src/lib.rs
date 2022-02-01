pub mod StringDemo {
    pub fn string_create() {
        let mut s = String::new();

        let data = "new string";

        let s1 = data.to_string();

        let s2 = String::from(data);

        println!("s1 -> {}, s2 -> {}", s1, s2)
    }

    pub fn string_update() {
        let mut s = String::from("hello");
        let s2 = String::from(" world\n");

        s += &s2;

        s.push_str(" are you ok?");

        s += " pidan";

        println!("s -> {}", s);

        let s4 = "tic";
        let s5 = "tac";
        let s6 = "toe";

        let concat = format!("{}-{}-{}", s4, s5, s6);

        println!("{}", concat);
    }

    pub fn slice_string() {
        let mut s = String::from("Hello world");
        let s2 = &s[1..2];

        println!("s2 -> {}", s2); // e
        println!("s2 -> {}", &s[0..1]); // e

        let firstChar = s.get(0..1);

        if let Some(pat) = firstChar {
            println!("char -> {}", pat);
        }
    }
}
