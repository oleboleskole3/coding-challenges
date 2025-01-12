use std::fs;
use regex::Regex;

const DATAPATH: &str = "data.txt";

fn main() {
    let re = Regex::new(r"mul\((?<arg1>\d*),(?<arg2>\d*)\)").unwrap();

    let data = read_data();

    let mut sum_all = 0;

    for caps in re.captures_iter(data.as_str()) {
        println!("Arg 1 {}, Arg 2 {}", &caps["arg1"], &caps["arg2"]);

        sum_all += caps["arg1"].parse::<i32>().unwrap() * caps["arg2"].parse::<i32>().unwrap();
    }

    println!("Sum of multiplications: {sum_all}");
}

fn read_data() -> String {
    fs::read_to_string(DATAPATH).expect("File doesn't exist")
}