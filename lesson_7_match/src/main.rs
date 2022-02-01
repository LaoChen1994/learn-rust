use rand::Rng;

#[derive(Debug)]
enum ScenicSpot {
    // 长城
    GreatWall,
    /// 西湖
    WestLake,
    // 乐山大佛
    LeshanGiantBuddha,
}

#[derive(Debug)]
struct Car {
    brand: String,
    version: String, 
    max_speed: u32,
    name: String
}

#[derive(Debug)]
struct Airplane {
    company: String,
    max_speed: u32,
    price: u32
}

#[derive(Debug)]
enum Vehicle {
    Walk,
    LandTraffic(Car),
    AirTraffic(Airplane)
}

fn main() {
    let spot = ScenicSpot::WestLake;
    let city = get_spot_city(&spot);

    println!("city -> {}", city);
    println!("game res -> {}", match_number());

    let x = None;
    get_add_result(&x)
}

fn match_number() -> String {
    let rand_num = rand::thread_rng().gen_range(1..10);
    
    let rlt_message = match rand_num {
        5 => String::from("二等奖"),
        3 => String::from("一等奖"),
        _ => String::from("你没中奖")
    };

    rlt_message
}

fn get_add_result (x: &Option<u32>) {

    if let Some(state) = x {
        let s = 6;
        println!("输入的值加6后是 -> {}", s + state);
    } else {
        println!("数字是无效的")
    }
}

fn get_spot_city(spot: &ScenicSpot) -> String {
    match spot {
        ScenicSpot::GreatWall => String::from("北京"),
        ScenicSpot::LeshanGiantBuddha => {
            String::from("四川")
        },
        ScenicSpot::WestLake => String::from("杭州"),
    }
}

fn get_traffic_tool(tool: &Vehicle) -> String {
    match tool {
        Vehicle::AirTraffic(state) => {
            println!("空运公司为 -> {}", state.company);
            String::from("坐飞机啦~")
        },
        Vehicle::LandTraffic(state) => {
            println!("今天坐 -> {}", state.name);
            String::from("坐车啦~")
        },
        Vehicle::Walk => String::from("走路锻炼身体~也不错")
    }
}
