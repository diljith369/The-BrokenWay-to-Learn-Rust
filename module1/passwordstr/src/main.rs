fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage {} passwordtotest", &args[0]);
        std::process::exit(1);
    }

    let passwd = &args[1];
    let strgn = check_strength(passwd);
    println!("Password {} is {}", passwd, strgn);
}

fn check_strength(passwd: &String) -> String {
    let leng = passwd.len();
    let has_upper = passwd.chars().any(|c| c.is_uppercase());
    let has_lower = passwd.chars().any(|c| c.is_lowercase());
    let has_digit = passwd.chars().any(|c| c.is_digit(10));
    let spl_char = passwd.chars().any(|c| "!@#$%^&*()".contains(c));
    if leng >= 12 && has_upper && has_lower && has_digit && spl_char {
        return "Strong".to_string();
    } else {
        return "Weak".to_string();
    }
}
