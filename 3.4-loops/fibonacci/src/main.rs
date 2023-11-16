// Ex. Generate the nth Fibonacci number

fn main() {
    let mut left_res : u32 = 0;
    let mut right_res: u32 = 1;
    let mut pointer: u32;

    println!("The Fibonacci Sequence");

    while(right_res < 2000) {
        pointer = left_res;
        left_res = right_res;
        right_res = pointer + right_res;
        println!("{pointer}");
    }
}