use rand::random;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let difficult = &args[1];

    let mut mask: i32 = -2147483648i32; // 0b1000_0000_0000_0000_0000_0000_0000_0000
    mask = mask >> (difficult.parse::<u32>().unwrap() - 1);

    loop {
        let r = random::<i32>();
        println!("{:032b}", r);
        if (r & mask) == 0 {
            break;
        }
    }
}
