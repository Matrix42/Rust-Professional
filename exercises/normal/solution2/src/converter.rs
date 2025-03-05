pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num, from_base) = parse(num_str);
    let decimal = to_decimal(&num, from_base);
    from_decimal(decimal, to_base)
}

fn parse(num_str: &str) -> (String, u32) {
    let parts = num_str.split('(').collect::<Vec<&str>>();
    let num_part = String::from(parts[0]);
    let base_part = parts[1].trim_end_matches(')').parse::<u32>().unwrap();
    (num_part, base_part)
}

fn to_decimal(num_str: &str, from_base: u32) -> u32 {
    let mut num = 0;
    for (i, c) in num_str.chars().rev().enumerate() {
        let digit = c.to_digit(from_base).unwrap();
        num += digit * from_base.pow(i as u32);
    }
    num
}

fn from_decimal(mut decimal: u32, to_base: u32) -> String {
    if decimal == 0 {
        return "0".to_string();
    }
    let mut num = String::new();
    while decimal > 0 {
        let digit = decimal % to_base;
        let c = char::from_digit(digit, to_base).unwrap();
        num.push(c);
        decimal /= to_base;
    }
    num.chars().rev().collect()
}
