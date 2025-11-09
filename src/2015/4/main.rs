fn main() {
    let input = include_str!("input");
    for i in 0.. {
        if &format!("{:x}", md5::compute(format!("{}{}", input, i)))[0..5] == "00000" {
            println!("{}", i);
            break;
        };
    }
    for i in 0.. {
        if &format!("{:x}", md5::compute(format!("{}{}", input, i)))[0..6] == "000000" {
            println!("{}", i);
            break;
        };
    }
}
