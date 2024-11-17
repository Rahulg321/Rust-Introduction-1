mod x {
    include!("basic_one.rs");
    include!("basic_two.rs");
}

struct Sqaure {
    side: u32,
}

impl Sqaure {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }

    fn perimeter(&self) -> u32 {
        return 4 * self.side;
    }
}

struct Rect {
    widht: u32,
    height: u32,
}

// similar to how we define classes in js
impl Rect {
    fn area(&self) -> u32 {
        return self.widht * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.widht + self.height);
    }

    fn debug() -> bool {
        println!("this is a static method, can be called direcly on the struct");
        println!("Debugging");
        return true;
    }
}

#[derive(Debug)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    age: u32,
    is_active: bool,
}

struct City {
    name: String,
    population: u32,
    country: String,
    sign_in_count: u64,
    is_g7: bool,
    is_brics: bool,
}

// a simple enum
enum Direction {
    North,
    East,
    South,
    West,
}

// enums with associated values
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn find_larger_shape(shape1: Shape, shape2: Shape) -> Shape {
    // we are passing a reference to avoid ownership
    let area_shape_1 = calculate_shape_area(&shape1);
    let area_shape_2 = calculate_shape_area(&shape2);

    if (area_shape_1 >= area_shape_2) {
        return shape1;
    } else {
        return shape2;
    }
}

fn calculate_shape_area(shape: &Shape) -> f64 {
    // calculate the area of the shape

    match shape {
        Shape::Circle(radius) => return 3.1459 * radius * radius,
        Shape::Rectangle(width, height) => return width * height,
        Shape::Square(side) => return side * side,
    }
}

fn main() {
    let my_shape = Shape::Rectangle(2.2, 3.4);
    let my_shape1 = Shape::Square(2.2);

    println!(
        "the bigger of the two shapes is {:#?}",
        find_larger_shape(my_shape, my_shape1)
    );

    let my_direction = Direction::East;

    let rect1 = Rect {
        widht: 32,
        height: 42,
    };

    println!("area of rectagle is {}", rect1.area());

    println!("calling a static debug function {}", Rect::debug());

    println!("learning structs");

    x::greet();
    x::greet2();

    let user1 = User {
        username: String::from("rg5353"),
        first_name: String::from("Rahul"),
        last_name: String::from("Gupta"),
        email: String::from(""),
        age: 0,
        is_active: false,
    };

    let city = City {
        name: String::from("New Delhi"),
        population: 1000000,
        country: String::from("India"),
        sign_in_count: 0,
        is_g7: false,
        is_brics: true,
    };

    println!("User1: {}", user1.first_name);
    println!("User1 username : {:?}", user1.first_name);

    println!("CityName : {:?}", city.name);

    // we can make mut objects
    let mut user2 = User {
        username: String::from("_rahulgupta312_"),
        first_name: String::from("Surbhi"),
        last_name: String::from("Bhandari"),
        email: String::from(""),
        age: 4,
        is_active: true,
    };

    user2.email = String::from("anotheremail@example.com");

    println!("User2: {:?}", user2);
}
