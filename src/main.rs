fn main() {
    println!("Hello, world!");
    // println!("{}", is_even(4));
    println!("{}", fib(4))
}

//i32 is a signed 32-bit integer
// fn is_even(num : i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }

fn fib(num : u32) -> u32 {
    let mut first = 0; // mut is a keyword that allows us to change the value of a variable
    let mut second = 1;

    if num == 0 {
        return first;
    } else if num == 1 {
        return second;
    } else {
        for _ in 0..num - 1 {
            let temp = second;
            second = second + first;
            first = temp;
        }
        return second;
    }
}

