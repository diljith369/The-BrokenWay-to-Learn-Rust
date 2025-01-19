use std::io;

fn main() {
    let mut kinput =  String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut kinput).expect("read error");
    let num: u32 = kinput.trim().parse().expect("parsing failed");
    println!("Hex  of {} is {:x}", num, num);

}