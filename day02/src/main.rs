use std::str::SplitAsciiWhitespace;

use helpers;
fn part1() {
    let tt = helpers::read_file("input", 2);
    tt.lines().for_each(|line|{
        line.split_ascii_whitespace().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>().filter(|x| predicate(x)).count();
    //     let t: Vec<_> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
    //    t.iter().filter(|x| x[0] < x[1]);
    }
)
}
fn predicate(values: &[i32])-> bool {
    let mut inc =true;
    let mut dec = true;


    for x in values.windows(2)
    {
       if x[0].abs_diff(x[1]) > 3 {
        return false
       }
       inc &= x[0] < x[1];
       dec &= x[0] > x[1];
       if !(inc ^ dec) {
           return false;
       }
    }
  
  true
}
    