use core::num;
use std::fs::File;
use std::io::{Read};

use regex::Regex;

fn main() -> std::io::Result<()>{
    // Parse input file
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input);

    // Approach is to create strings representing each possible reading
    // direction and check for the string in each
    let mut num_xmas_words = 0;
    let lines : Vec<&str> = input.split("\n").collect();
    let horizontal_line_len = lines.len();
    let vertical_line_len = lines[0].len();

    // Start with Horizontal and the reverse
    for line in &lines{
        num_xmas_words += count_xmas_occurences(line);
        num_xmas_words += count_xmas_occurences(&reverse_string(line));
    }


    // Vertical and reverse (assumes all rows are same length)
    let vertical_lines = lines.clone();
    for i in 0..vertical_line_len{
        let vertical_row : String = vertical_lines.iter().map(|x| x.chars().nth(i).unwrap_or(' ')).collect();
        num_xmas_words += count_xmas_occurences(&vertical_row);
        num_xmas_words += count_xmas_occurences(&reverse_string(&vertical_row));
    }

    // Diagonals and reverse. These suck :(
    //      3 4 5 6 7 8 9
    //      2 3 4 5 6 7 8
    //      1 2 3 4 5 6 7
    //      0 1 2 3 4 5 6
    // We can walk upwards from the las row to the first and
    // push each diagonal char into a string. This will be reversed
    // but fortunetly we need them both directions anyways
    let num_diagonals = vertical_line_len + horizontal_line_len - 1;
    let mut current_diagonal_start_num = 0;
    let mut l_r_diagonals : Vec<String> = vec!["".to_string(); num_diagonals];
    let mut r_l_diagonals : Vec<String> = vec!["".to_string(); num_diagonals];


    // Left-to-right
    for i in 0..vertical_line_len{
        let row = lines[vertical_line_len - 1 - i]; // Reverse walk index
        for j in 0..horizontal_line_len{
            let target_char = row.chars().nth(j).unwrap().to_string();
            l_r_diagonals[current_diagonal_start_num + j] += &target_char;
        }
        current_diagonal_start_num += 1;
    }

    for diagonal in l_r_diagonals{
        num_xmas_words += count_xmas_occurences(&diagonal);
        num_xmas_words += count_xmas_occurences(&reverse_string(&diagonal));
    }

    // Right-to-left. We need to flip our horizontal indexing
    //      9 8 7 6 5 4 3
    //      8 7 6 5 4 3 2
    //      7 6 5 4 3 2 1
    //      6 5 4 3 2 1 0
    current_diagonal_start_num = 0;

    for i in 0..vertical_line_len{
        let row = lines[vertical_line_len - 1 - i]; // Reverse walk index
        for j in 0..horizontal_line_len{
            let target_char = row.chars().nth(horizontal_line_len -1 - j).unwrap().to_string();
            r_l_diagonals[current_diagonal_start_num + j] += &target_char;
        }
        current_diagonal_start_num += 1;
    }

    for diagonal in r_l_diagonals{
        num_xmas_words += count_xmas_occurences(&diagonal);
        num_xmas_words += count_xmas_occurences(&reverse_string(&diagonal));
    }

    // Our toils are over
    println!("Num XMAS occurences: {}", num_xmas_words);
    Ok(())
}

fn reverse_string(input: &str) -> String{
    format!("{}", input.chars().rev().collect::<String>())
}

fn count_xmas_occurences(input: &str) -> i32{
    let xmas_regex = Regex::new(r"XMAS").unwrap();
    let num_matches = xmas_regex.find_iter(input).count();
    // println!("Matches: {}\n-----", num_matches);
    return num_matches.try_into().unwrap()
    

}