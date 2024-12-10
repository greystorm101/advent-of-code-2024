use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};

struct PrintOrder{
    before_numbers : HashMap<i32, Vec<i32>>,
    after_numbers : HashMap<i32, Vec<i32>>
}

impl PrintOrder {
    fn new(order_pairs : Vec<Vec<i32>>) -> Self{

        let mut before_numbers : HashMap<i32, Vec<i32>> = HashMap::new();
        let mut after_numbers : HashMap<i32, Vec<i32>> = HashMap::new();

        for ordered_pair in order_pairs{
            let before_num = ordered_pair[0];
            let after_num = ordered_pair[1];
            
            before_numbers.entry(after_num).or_insert(vec![]).push(before_num);
            after_numbers.entry(before_num).or_insert(vec![]).push(after_num);
        }

        Self{before_numbers , after_numbers}
    }

    fn is_valid_order(&self, input: &[i32]) -> bool{
        for i in 0..input.len(){
            let num = input[i];
            let before_prints = &input[..i];
            let after_prints = &input[i+1..];
            
            if before_prints.iter().any(|x| self.after_numbers.get(&num).unwrap_or(&vec![]).contains(x)){
                return false;
            }

            if after_prints.iter().any(|x| self.before_numbers.get(&num).unwrap_or(&vec![]).contains(x)){
                return false;
            }
        }
        true
    }

    fn fix_order(&self, input: &[i32]) -> Vec<i32>{
        let mut new_order: Vec<i32> = input.to_vec();
        while !self.is_valid_order(&new_order){

            for i in 0..new_order.len(){
                let num = new_order[i];
                let before_prints = &new_order[..i];
                let after_prints = &new_order[i+1..];
                
                let before_map: Vec<bool> = before_prints.iter().map(|x| self.after_numbers.get(&num).unwrap_or(&vec![]).contains(x)).collect();
                let after_map: Vec<bool> = after_prints.iter().map(|x| self.before_numbers.get(&num).unwrap_or(&vec![]).contains(x)).collect();
                
                if before_map.contains(&true){
                    let bad_before_pos = before_map.iter().position(|&x| x == true).unwrap();
                    new_order.swap(bad_before_pos, i);
                    break;
                } else if after_map.contains(&true) {
                    let bad_after_pos = after_map.iter().position(|&x| x == true).unwrap();
                    new_order.swap(bad_after_pos + i + 1, i);
                    break;
                }
            }
        }
        
        new_order
    }
}


fn main() -> std::io::Result<()>{
    // Parse input files
    let mut file = File::open("input-1.txt")?;
    let mut pairs_input = String::new();
    file.read_to_string(&mut pairs_input);

    let mut file = File::open("input-2.txt")?;
    let mut printing_input = String::new();
    file.read_to_string(&mut printing_input);

    let order_pairs: Vec<Vec<i32>> = pairs_input.split('\n').map(
        |x| x.split('|').map(|y|y.parse().unwrap()).collect())
        .collect();

    // Map each int to numbers that must come before it and afer it
    let printing_orders = PrintOrder::new(order_pairs);
    
    let printing_queues:  Vec<Vec<i32>> = printing_input.split('\n').map(
        |x| x.split(',').map(|y|y.parse().unwrap()).collect())
        .collect();
    
    let mut sum_of_correct_middles = 0;
    let mut sum_of_corrected_middles = 0;

    for print_order in printing_queues{
        if printing_orders.is_valid_order(&print_order){
            sum_of_correct_middles += find_middle_numer(&print_order);
        } else {
            let new_print_order = printing_orders.fix_order(&print_order);
            sum_of_corrected_middles += find_middle_numer(&new_print_order);

        }
    }

    println!("Sum of correct middles: {}", sum_of_correct_middles);
    println!("Sum of corrected middles: {}", sum_of_corrected_middles);

    Ok(())
}


fn find_middle_numer(input: &[i32]) -> i32{
    input[input.len()/2]
}