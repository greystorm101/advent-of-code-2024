use std::fs::File;
use std::io::{prelude, Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input);
    
    let pairs = input.split("\n");
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for item in pairs{
        let mut nums = item.split_ascii_whitespace();
        left_list.push(nums.next().unwrap().parse().unwrap());
        right_list.push(nums.next().unwrap().parse().unwrap());
    }

    // Part 1
    left_list.sort();
    right_list.sort();

    let mut diffs: Vec<i32> = vec![];
    let mut sum: i32 = 0;

    for n in 0..left_list.len(){
        let diff = (left_list[n]-right_list[n]).abs();
        diffs.push(diff);
        sum += diff;
    }
    // println!("{:?}", diffs);
    println!("Compared Sum: {}", sum);

    // Part 2
    let mut similarity_score = 0;
    for num in left_list{
        let mut dup_count = 0;
        for item in &right_list{
            if item == &num{
                dup_count += 1;
            }
        }
        // println!("Dupe: {}", dup_count);
        similarity_score += dup_count * num;
    } 
    println!("Similarity: {}", similarity_score);

    Ok(())

}
