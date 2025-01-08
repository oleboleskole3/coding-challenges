use std::{collections::HashMap, fs};

// const datapath: str = "data.txt";

fn main() {
    let data = read_data();

    // let val  = 2;

    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];

    let mut right_vals: HashMap<i32, i32> = HashMap::new();

    right_vals.insert(1, 5);


    for line in data.split('\n').into_iter() {
        println!("line: {line}");
        let mut vals = line.split("   ").into_iter();

        let val1 = vals.next().unwrap().trim();
        let val1 = val1.parse().unwrap();
        nums1.push(val1);
        
        let val2 = vals.next().unwrap().trim();
        let val2 = val2.parse().unwrap();
        nums2.push(val2);

        // right_vals.get()

        // right_vals.get(val2)

        if let Some(count) = right_vals.get(&val2) {
            right_vals.insert(val2, count + 1);
        } else {
            right_vals.insert(val2, 1);
        }
    }

    // println!("Count:\n{right_vals}");

    // println!("Nums1:\n{nums1}");

    nums1.sort();
    nums2.sort();

    let mut sum = 0;
    let mut sum2 = 0;

    for i in 0..nums1.len() {
        sum += (nums1[i] - nums2[i]).abs();

        sum2 += nums1[i] * right_vals.get(&nums1[i]).unwrap_or(&0);
    }

    println!("Sum of difference of sorted vals: {sum}");
    println!("Sum2: {sum2}");
}

fn read_data() -> String {
    fs::read_to_string("data.txt").expect("File doesn't exist")
}
