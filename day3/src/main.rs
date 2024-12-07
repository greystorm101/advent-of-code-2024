use std::fs::File;
use std::io::{Read};
use regex::Regex;
fn main() -> std::io::Result<()>{
    // Parse input file
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input);

    // Swap to false for part 1
    let solve_part_2 = true;

    let mut sum = 0;

    if solve_part_2 == true{

        let stop_pattern = "don't()";
        let start_pattern = "do()";

        // If we split the input on the stop pattern, we know the all the strongs will be in the "off state"
        // until we see the start pattern
        let mut stop_strings: Vec<&str> = input.split(stop_pattern).collect();

        // Since the sequence starts enabled, we do a little hack to set it to do() right away
        let modified_start = &format!("{},{}", start_pattern ,stop_strings[0]);
        stop_strings[0] = modified_start;

        for instruction_string in stop_strings{
            // Split the string only on the first "do()" and do anything after that
            if let Some(do_instructions) = instruction_string.split_once(start_pattern){
                sum += get_sum_of_mul_functions(do_instructions.1);
            }
        }

    } else {
        sum = get_sum_of_mul_functions(&input);
    }
    
    println!("Sum: {}", sum); 
    Ok(())
}

fn get_sum_of_mul_functions(input_str: &str) -> i32{
    let number_list: Vec<i32> = _parse_numbers_in_muls(&input_str);
    let mut sum = 0;
    number_list.iter().for_each(|i| sum = i + sum );
    sum
}

fn _parse_numbers_in_muls(input_str: &str) -> Vec<i32>{
    let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    // Parse out al `mul(x,x)` occurence
    let mults: Vec<&str> = mul_regex.find_iter(input_str).map(|m| m.as_str()).collect();
    // List of lists for mult number pairs
    let parsed_numbers: Vec<Vec<&str>> = mults.iter().map(|x: &&str| 
        x.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",").collect()).collect();
    
    parsed_numbers.iter().map(|x: &Vec<&str>| x[0].parse::<i32>().unwrap() * x[1].parse::<i32>().unwrap() ).collect()
}
