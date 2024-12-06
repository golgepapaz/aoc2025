use helpers;
fn part1() {
    let tt = helpers::read_file("input", 2);
    tt.lines().for_each(|line|{
        let t: Vec<_> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        t.as_slice().windows(2).filter(|x| x[0] < x[1]);
    }
)
}
fn predicate(values: &[i32])-> bool {
    let mut inc =true;
    let mut dec = true;
    let valid =true;

    values.windows(2).filter(|&x| {
       if x[0].abs_diff(x[1]) > 3 {
        return false;
       }
       inc &= x[0] < x[1];
       dec &= x[0] > x[1];
       if !(inc ^ dec) {
           return false;
       }
    }
    )

}
    