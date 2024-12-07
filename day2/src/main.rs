use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input);

    let mut num_good_reports = 0;
    for line in input.lines(){
        let report: Vec<i32> = line.split_whitespace().map(|x:&str| x.parse::<i32>().unwrap()).collect();
        let res = report_is_valid(report.clone(), None).unwrap();
        // println!("Report: {:?} Result:{}", report, res);

        if res == true{
            num_good_reports += 1;
        }
    }

    println!("Stable reports: {}", num_good_reports);
    Ok(())
}

fn report_is_valid(mut report: Vec<i32>, on_sublist:Option<bool>) -> Result<bool,Box<dyn std::error::Error>>{
    // Assumes report is at least 2 numbers long
    let ascention_order_mult = if report[0] < report[1] {1} else {-1};
    
    // For part 2. If looking for part 1 solution, just set this to true
    let mut allow_one_problem = if on_sublist.is_some_and(|x| x == true) {false} else {true};

    for i in 1..report.len(){
        let first_number = report[i -1];
        let second_number = report[i];
        
        // Calculate valid ranges, adjusting descending to be negative to and ascending to be positive for easier comparison
        let range: [i32; 2] = [ascention_order_mult * (first_number + (1 * ascention_order_mult)), 
                               ascention_order_mult * (first_number + (3 * ascention_order_mult))];

        let adjusted_num = second_number * ascention_order_mult;

        match (range[0] <= adjusted_num && adjusted_num <= range[1]) {
            true => continue,
            false => if !allow_one_problem {return Ok(false)} else{
                // Determine if we can pop first or second number
                let mut report_attempt_one = report.clone();
                let mut report_attempt_two = report.clone();
                report_attempt_one.remove(i);
                report_attempt_two.remove(i-1);

                let res1 = report_is_valid(report_attempt_one, Some((true))).unwrap();
                let res2 = report_is_valid(report_attempt_two, Some((true))).unwrap();
                return Ok(res1 || res2)
            }
            
        }
    }
    Ok(true)
}
