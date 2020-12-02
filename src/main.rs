use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("data/01/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split("\n");

    let nums = lines
        .into_iter()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let matches = find_sums(&nums, 2, 2020);
    println!("{:?}", matches);
    for m in matches {
        println!("prouct: {}", m.into_iter().product::<u32>());
    }

    Ok(())
}

fn find_sums(nums: &Vec<u32>, count: u32, total: u32) -> Vec<Vec<u32>> {
    fn sums(check_nums: &Vec<u32>, count: u32, total: u32, current: &Vec<u32>) -> Vec<Vec<u32>> {
        let next_count = count - 1;
        let mut found = Vec::new();
        for check_num in check_nums {
            let mut next_current = current.clone();
            next_current.push(*check_num);
            next_current.sort();

            let next_sum = next_current.iter().sum::<u32>();

            if next_count > 0 {
                let mut check_found = sums(check_nums, next_count, total, &next_current);

                found.append(&mut check_found);
            } else if next_sum == total {
                found.push(next_current)
            }
        }
        found.dedup();
        found
    };

    sums(nums, count, total, &Vec::new())
}
