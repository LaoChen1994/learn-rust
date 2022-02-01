mod lib;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);

    let mut v2 = vec![1, 2, 3];

    {
        let thrid = &v2[2];
        println!("{}", thrid);
    }

    v2.push(4);


    for i in &mut v2 {
        *i = *i * 100 / 2;
    }

    for item in &v2 {
        println!("value => {}", item);
        // value => 51
        // value => 52
        // value => 53
        // value => 54
        // value -> 53
    }

    

    let fourth = v2.get(2);

    match fourth {
        Some(value) => println!("value -> {}", value),
        None => println!("no value"),
    }

    lib::EnumTypesTest::useMultiTypes();
}
