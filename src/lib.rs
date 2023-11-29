use anyhow::Result;
pub use aoc_macro::main;

pub fn get_input(day: usize, year: usize) -> Result<String> {
    elv::get_input(day, year, None)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
