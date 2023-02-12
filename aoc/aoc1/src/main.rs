use std::{
    fs::File,
    io::{self, BufReader, prelude::*}, cmp,
};

#[allow(dead_code)]
fn my_sol_a() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);

    let mut cur_running_sum: u32 = 0;
    let mut ans:u32 = 0;
    for line in reader.lines() {
        match line?.parse::<u32>() {
            Ok(cur_energy) => cur_running_sum += cur_energy,
            Err(_) => {
                ans = cmp::max(ans, cur_running_sum);
                cur_running_sum = 0;
            }
        }
    }
    println!("my_sol_a: {ans}");
    Ok(())
}

#[allow(dead_code)]
fn better_sol_a(){
    println!(
        "better_sol_a: {}",
        include_str!("../in.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap(),
        );
}

#[allow(dead_code)]
fn my_sol_b() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);

    let mut energies = Vec::new();
    let mut cur_running_sum: u32 = 0;
    for line in reader.lines() {
        match line?.parse::<u32>() {
            Ok(cur_energy) => cur_running_sum += cur_energy,
            Err(_) => {
                energies.push(cur_running_sum);
                cur_running_sum = 0;
            }
        }
    }
    energies.sort();
    println!("my_sol_b: {}",energies.iter().rev().take(3).sum::<u32>());
    Ok(())
}

#[allow(dead_code)]
fn better_sol_b() {
    let mut cals = include_str!("../in.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    cals.sort_unstable();
    println!("{}", cals.into_iter().rev().take(3).sum::<u32>());
}


fn main() -> io::Result<()>{
    my_sol_a()?;
    better_sol_a();
    my_sol_b()?;
    better_sol_b();
    Ok(())
}

/*
   Learnings:

    1. prelude::* brings everything inside the crate to the scope.
    For example, here we are not specifically importing std::io::BufRead

    2. io.Result(from rust docs)
    /// A specialized [`Result`] type for I/O operations.
    /// We use `io::Result` instead of shadowing the [prelude]'s import
    /// of [`std::result::Result`][`Result`]
    Here prelude is from std::result

    3. include_str!(from rust docs)
    /// This macro will yield an expression of type `&'static str` which is the
    /// contents of the file.

    4. reverse iterators are very helpful
    
    TODO: learn more about iterators and closures
*/
