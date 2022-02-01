mod lib;

fn main() {
    println!("Hello, world!");
    lib::StringDemo::string_create();
    lib::StringDemo::string_update();
    lib::StringDemo::slice_string();
}
