fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage {} param1", &args[0]);
    } else {
        let firscmd = &args[1];
        println!(" first args {}", firscmd);
    }
}
