use std::fs;

// const datapath: str = "data.txt";

fn main() {
    let data = read_data();

    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];

    for line in data.split('\n').into_iter() {
        println!("line: {line}");
        let mut vals = line.split("   ").into_iter();

        let val = vals.next().unwrap().trim();
        println!("val: {val}");
        nums1.push(val.parse().unwrap());
        let val = vals.next().unwrap().trim();
        println!("val: {val}");
        nums2.push(val.parse().unwrap());
    }

    // println!("Nums1:\n{nums1}");

    nums1.sort();
    nums2.sort();

    let mut sum = 0;

    for i in 0..nums1.len() {
        sum += (nums1[i] - nums2[i]).abs();
    }

    println!("Sum of difference of sorted vals: {sum}")
}

fn read_data() -> String {
    fs::read_to_string("data.txt").expect("File doesn't exist")
}
