#[derive(Debug)]
enum EngineCylinder {
    V4,
    V6,
    V8,
    V12,
}

struct Car {
    brand: String,
    engine: EngineCylinder,
    version: String,
    price: f64,
}

struct Engine {
    cylinderNumber: EngineCylinder,
    power: u32,
    cc: u32,
    name: String
}

impl Car {
    fn create(brand: String, engine: EngineCylinder, version: String, price: f64) -> Car {
        Car {
            brand,
            engine,
            version,
            price,
        }
    }

    fn introduce(&self) {
        println!(
            "这是一辆{}的{}, 他搭载{:?}引擎，售价为{}万元",
            &self.brand, &self.version, &self.engine, &self.price
        )
    }
}

#[derive(Debug)]
enum CarBrand {
    Geely(String),
    LINCO(String),
    NIO(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn invoke (&self) {
        println!("123")
    }
}

enum OptionValue<T> {
    None,
    Some(T)
}

fn main() {
    let v6Engine = EngineCylinder::V6;
    let v8Engine = EngineCylinder::V8;

    let engine = Engine {
        cylinderNumber: EngineCylinder::V6,
        power: 261,
        name: String::from("3.0T"),
        cc: 2956
    };

    checkEngine(&engine);

    let aviator = Car::create(
        String::from("LINCOLN"),
        EngineCylinder::V6,
        String::from("AVIATOR"),
        50.98,
    );

    let geelyIcon = CarBrand::Geely(String::from("Icon"));
    let linco05 = CarBrand::LINCO(String::from("05"));
    let linco01 = CarBrand::LINCO(String::from("01"));

    println!("05 -> {:?}, 01 -> {:?}", linco05, linco01);

    aviator.introduce();

    let write_message = Message::Write(String::from("123"));

    write_message.invoke();

    let x: i32 = 5;
    let some_number: Option<i32> = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let rlt: Option<i32> = match some_number {
        None => None,
        Some(state) => Some(state + x)
    };

    println!("rlt -> {:?}", rlt);
}

fn checkEngine(engine: &Engine) {
    println!("engine is -> {:?}", engine.cylinderNumber)
}