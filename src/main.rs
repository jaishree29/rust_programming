fn main() {
    println!("Hello, world!");

    let rect = Rect {
        width: 40,
        height: 50
    };

    println!("The area of the rectangle is: {}", rect.area());

    println!("The perimeter of the rectangle is: {}", rect.perimeter(2, 4));

    println!("Debugging the rectangle struct: {}", Rect::debug());


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
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self, num1: u32, num2: u32) -> u32 {
        2 * (num1 + num2)
    }

    // This is a static method, it can be called without an instance of the struct
    fn debug() -> u32{
        return 1;
    }
}

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