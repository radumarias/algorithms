use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let _vata: Vec<i32> = vec![22, 15, 18];
    let mut statistics: HashMap<i32, (i32, i32)> = HashMap::new();
    for &val in &_vata {
        println!(
            "----------------------------{}----------------------------",
            val
        );
        let mut index = 0;
        let mut found = false;
        for (&key, value) in statistics.iter_mut() {
            println!(
                "cheking: {}, for key {}: ({}, {})",
                val, key, value.0, value.1
            );
            if &val == &key {
                found = true;
                value.1 += 1;
                println!("number found {} updating number of occurence ", val)
            } else if &key < &val {
                value.0 += 1;
                println!(
                    "key {} <  number {} updated number of biggers for {} to {}",
                    key, val, key, value.0
                );
            } else {
                index += 1;
            }
        }

        if !found {
            println!("inserting with 0");
            statistics.insert(val, (index, 1));
        }
    }
    let vlen: i32 = (_vata.len() / 2).try_into().unwrap();
    let mut middle: i32 = 0;

    for (&key, value) in &statistics {
        println!(
            "Checking {}. vlen {}. has {} greater than ",
            key, vlen, value.0
        );
        if vlen <= value.0.try_into().unwrap() {
            middle = max(middle, key);
            println!("middle updated to {} ---", middle);
        } else if vlen < value.1.try_into().unwrap() {
            middle = max(middle, key);
            println!("middle updated to {} on else", middle);
        }
    }
    for (&key, value) in &statistics {
        print!(
            "number {} can be found {} times in the vector. It has {} items greater then him ",
            key, value.1, value.0
        );
        if key == middle {
            print!("This number can be found at the middle of the array.");
        }
        println!("");
    }
}
