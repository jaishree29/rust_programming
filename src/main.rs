// use std::fs::read_to_string;
// use chrono::{Local, Utc};

fn main() {
    // println!("Hello, world!");

    // let now = Utc::now();
    // println!("the Utc time is {}", now);

    // let local = Local::now();
    // println!("the local time is {}", local);

    // let mut s1 = String::from("Jaishree ");
    // do_something(&mut s1);
    // println!("{}", s1);


    //Vectors are stored in the heap not on the stack
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);

    println!("{:?}", even_vec(vec));


    // let circle = Shape::Circle(5.0);
    // let square = Shape::Square(4.0);
    // let rectangle = Shape::Rectangle(10.0, 20.0);
    // let area_of_circle = calculate_area(circle);
    // let area_of_square = calculate_area(square);
    // let area_of_rectangle = calculate_area(rectangle);
    
    // println!("The area of the circle, square and rectangle is: {}, {}, {}", area_of_circle, area_of_square, area_of_rectangle);


    // let rect = Rect {
    //     width: 40,
    //     height: 50
    // };

    // println!("The area of the rectangle is: {}", rect.area());

    // println!("The perimeter of the rectangle is: {}", rect.perimeter(2, 4));

    // println!("Debugging the rectangle struct: {}", Rect::debug());


    // let user = User {
    //     first_name: String::from("Jaishree"),
    //     last_name: String::from("Tiwari"),
    //     age: 21,
    // };

    // print!("First Name: {}, Last Name: {}, Age: {}\n", 
    //     user.first_name, user.last_name, user.age
    // );
    
    // let my_string = String::from("Jaishree");
    // let length = get_str_length(my_string);
    // println!("The length of the string is: {}", length);

    // println!("{}", is_even(4));
    // println!("{}", fib(4))

    // let color = my_color::Red;

    // let message = choose_color(color);

    // println!("{}", message);

    // let index = find_first_a(String::from("shree"));
    // match index {
    //     Some(value) => println!("The first 'a' is at index: {}", value),
    //     None => println!("No 'a' found in the string"),
    // }

    // let result = read_to_string("example.txt");
    // match result {
    //     Ok(value) => println!("File content: {}", value),
    //     Err(e) => println!("Error reading file: {}", e),
    // }
}

fn even_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in vec {
        if i%2 == 0 {
            new_vec.push(i);
        }
    }

    return new_vec;

}

// fn do_something(s2: &mut String) {
//     s2.push_str("Tiwari");
//     println!("{}", s2);
//     // return s2;
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }

//     return None;
// }

// enum Shape {
//     Circle(f64), // f64 is a 64-bit floating point number for radius
//     Square(f64), // f64 for side length
//     Rectangle(f64, f64), // Tuple struct with two f64 values for width and height
// }

// fn calculate_area(shape: Shape) -> f64 {
//    let result = match shape {
//     Shape::Circle(r) => 3.14 * r * r,
//     Shape::Square(s) => s * s,
//     Shape::Rectangle(w, h) => w * h,
//    };
//    return result;
// }
// enum my_color {
//     Red,
//     Green,
//     Blue,
// }

// fn choose_color(color: my_color) -> String {
//     match color {
//         my_color::Red => String::from("You chose Red!"),
//         my_color::Green => String::from("You chose Green!"),
//         my_color::Blue => String::from("You chose Blue!"),
//     }
// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn perimeter(&self, num1: u32, num2: u32) -> u32 {
//         2 * (num1 + num2)
//     }

//     // This is a static method, it can be called without an instance of the struct
//     fn debug() -> u32{
//         return 1;
//     }
// }

// // Structs are used to create custom data types in Rust
// struct User {
//     first_name: String,
//     last_name: String,
//     age: u32,
// }

// fn get_str_length(str: String) -> usize {
//     str.chars().count()
// }


//i32 is a signed 32-bit integer


// fibonacci series
// fn fib(num : u32) -> u32 {
//     let mut first = 0; // mut is a keyword that allows us to change the value of a variable
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     } else if num == 1 {
//         return second;
//     } else {
//         for _ in 0..num - 1 {
//             let temp = second;
//             second = second + first;
//             first = temp;
//         }
//         return second;
//     }
// }