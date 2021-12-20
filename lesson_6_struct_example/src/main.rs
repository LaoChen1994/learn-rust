#[derive(Debug)]
struct Dimension {
    width: u32,
    height: u32,
}

impl Dimension {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    fn area(&self) -> u32 {
        let mut area: u32 = 0;

        if self.width() && self.height() {
            area = self.width * self.height;
        }

        area
    }

    fn square(width: u32) -> Dimension {
        Dimension {
            width,
            height: width,
        }
    }
}

impl Dimension {
    fn can_hold(&self, dimension: &Dimension) -> bool {
        &self.width > &dimension.width && &self.height > &dimension.height
    }
}

fn main() {
    let width = 30;
    let height = 40;

    let areaRlt = area(&width, &height);
    let areaRlt2 = areaTuples((&width, &height));
    let demension = Dimension { width, height };
    let areaRlt3 = areaStuct(&demension);

    println!("area -> {}", areaRlt);
    println!("area2 -> {}", areaRlt2);
    println!("area3 -> {}", areaRlt3);
    println!("debug -> {:#?}", demension);
    println!("area -> {}", demension.area());

    let rect1 = Dimension {
        width: 30,
        height: 50,
    };

    let rect2 = Dimension {
        width: 20,
        height: 15,
    };

    let rect3 = Dimension {
        width: 25,
        height: 10,
    };

    println!("Can rect 1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect 2 hold rect3 ? {}", rect2.can_hold(&rect3));

    println!("square area -> {}", Dimension::square(3).area())
}

fn area(x: &u32, y: &u32) -> u32 {
    x * y
}

fn areaTuples(dimesion: (&u32, &u32)) -> u32 {
    let (width, height) = dimesion;
    width * height
}

fn areaStuct(prop: &Dimension) -> u32 {
    let width = prop.width;
    let height = prop.height;

    width * height
}
