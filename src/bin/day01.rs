use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn input () -> std::io::Result<String>{
    let mut f = File::open("input/day01.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)


}
fn part1() -> std::io::Result<()>{

    let input = input().unwrap();
    let freq: i32 = input.split("\n").map(|n| n.parse::<i32>().unwrap()).sum();
    
    println!("{}", freq);
    
    
    Ok(())
}


fn main() -> std::io::Result<()> {
    part1()?;
    

    Ok(())
}
