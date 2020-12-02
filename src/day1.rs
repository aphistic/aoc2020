use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub fn run() -> std::io::Result<()> {
    let mut file = File::open("data/01/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split("\n");

    let nums = lines
        .into_iter()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let matches = find_sums(&nums, 3, 2020);
    println!("{:?}", matches);
    for m in matches {
        println!("product: {}", m.into_iter().product::<u32>());
    }

    Ok(())
}

fn find_sums(nums: &Vec<u32>, count: u32, total: u32) -> Vec<Vec<u32>> {
    fn sums(check_nums: &Vec<u32>, count: u32, total: u32, current: &Vec<u32>) -> Vec<Vec<u32>> {
        check_nums
            .iter()
            .map(|chk| {
                let next_count = count - 1;

                let mut next_current = current.clone();
                next_current.push(*chk);
                next_current.sort();

                let next_sum = next_current.iter().sum::<u32>();

                if next_count > 0 {
                    sums(check_nums, next_count, total, &next_current)
                } else if next_sum == total {
                    vec![next_current]
                } else {
                    vec![]
                }
            })
            .flatten()
            .collect()
    };

    let mut found = nums.into_par_iter()
        .map(|c| sums(nums, count-1, total, &vec!{*c}))
        .flatten()
        .collect::<Vec<Vec<u32>>>();
    found.dedup();
    found
}