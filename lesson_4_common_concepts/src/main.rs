const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const C_C: &str = "_global cc";

fn main() {
    const B_BIT: u32 = 3;
    // 通过mut来区分类似const和let的概念
    let mut a = 5;
    println!("the variable is {}", a);
    a = 6;
    println!("the variable is {}", a);

    println!(
        "the constant is {}, {}, {}",
        B_BIT, C_C, THREE_HOURS_IN_SECONDS
    );
    let x = 5;
    let x = x + 1;

    let x_str = "   ";

    let x_str = x_str.len();

    {
        let x = x * 2;
        println!("The inner x Number is {}", x);
    }

    println!("The x Number is {}, len = {}", x, x_str);

    // 标量类型
    let eight: u8 = 254;
    let floorded: f64 = 2.0 / 3.0;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!(
        "the float is {}, {}, {}, {}, {}, {}",
        floorded, eight, f, c, z, heart_eyed_cat
    );

    // 组合类型
    // 元组
    let tup: (i32, i8, f64) = (500, 22, 3.14);
    let (t1, t2, t3) = tup;
    println!("tup -> {}, {}, {}", tup.0, tup.1, tup.2);
    println!("tup -> {}, {}, {}", t1, t2, t3);

    // 数组
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [3;5];
    let array3: [u8; 5] = [1, 2, 3, 4, 5];

    

    println!("str -> {}", array2[1]);
}
