use std::fs;

pub fn read_file(folder:& str, day: u8) -> String {
    let cwd= std::env::current_dir().unwrap();
    let path = cwd.join(folder).join(format!("{:02}.txt", day));
    let f = fs::read_to_string(path);
    f.expect("could not read file")

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let content = read_file("input", 1);
        assert_eq!(content, "Hello World!");

    }
}
