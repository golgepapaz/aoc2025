
use helpers;
fn main() {
   let tt = helpers::read_file("input", 1);
   let mut vec1: Vec<u32> = Vec::new();
   let mut vec2: Vec<u32> = Vec::new();  
   tt.lines().for_each(|line|{
        let mut split = line.split("   ");
        {
                
                vec1.push(split.next().and_then(|s| s.parse().ok()).unwrap());
                vec2.push(split.next().and_then(|s| s.parse().ok()).unwrap());
        }}
   );
   vec1.sort();
   vec2.sort();
   let diff: u32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a.abs_diff(*b)).sum();

   let mut similar: u32 = 0;
   for i in vec1.iter() {
       
        similar += i *  vec2.iter().filter(|&x| *x == *i).count() as u32;
   }
   println!("{:?}", similar);
}
