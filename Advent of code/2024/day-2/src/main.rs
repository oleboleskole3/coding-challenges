use std::fs;

enum ReportDir {
    Unknown,
    Rising,
    Falling
}

const DATAPATH: &str = "data.txt";


fn main() {
    let data = read_data();
    let reports = data.split('\n');

    let mut safe_reports = 0;
    for report in reports {
        let mut dir = ReportDir::Unknown;
        let mut prev = None;

        let mut fail = false;
        report.trim().split(' ').for_each(|val| {
            let val: i32 = val.parse().expect("{val} not number");

            if let Some(prev) = prev {
                let diff: i32 = val - prev;
                if (diff).abs() == 0 || diff.abs() > 3 {
                    fail = true;
                    print!("fail, ")
                } 
                match dir {
                    ReportDir::Rising => {
                        if diff < 0 {
                            fail = true;
                        }
                    }
                    ReportDir::Falling => {
                        if diff > 0 {
                            fail = true;
                        }
                    }
                    ReportDir::Unknown => {
                        if diff > 0 {
                            dir = ReportDir::Rising;
                        } else {
                            dir = ReportDir::Falling;
                        }
                    }
                }
                print!("{diff}, {fail}, ")
            }
            println!();

            prev = Some(val);
        });
        if !fail {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
}

fn read_data() -> String {
    fs::read_to_string(DATAPATH).expect("File doesn't exist")
}
