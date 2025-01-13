use std::fs;
use regex::Regex;

const DATAPATH: &str = "data.txt";

fn main() {
    let re = Regex::new(r"(?<instr>(mul\((?<arg1>\d*),(?<arg2>\d*)\))|(do\(\))|(don't\(\)))").unwrap();

    let data = read_data();

    let mut sum_all = 0;
    let mut sum_enabled = 0;
    let mut enabled = true;

    for caps in re.captures_iter(data.as_str()) {
        match &caps["instr"] {
            "do()" => {
                enabled = true;
                continue;
            }
            "don't()" => {
                enabled = false;
                continue;
            }
            _ => {}
        }
        // println!("Arg 1 {}, Arg 2 {}", &caps["arg1"], &caps["arg2"]);

        let arg1 = caps["arg1"].parse::<i32>().unwrap();
        let arg2 = caps["arg2"].parse::<i32>().unwrap();

        sum_all += arg1 * arg2;

        if enabled {
            sum_enabled += arg1 * arg2;
        }
    }

    println!("Sum of multiplications: {sum_all}");
    println!("Sum of enabled multiplications: {sum_enabled}");
}

fn read_data() -> String {
    fs::read_to_string(DATAPATH).expect("File doesn't exist")
}