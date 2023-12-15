pub mod advent2023;

#[cfg(test)]
mod test {
    use std::fs;

    /// Helps loading in test data. This can be changed in the future to pull the data directly
    /// from the server if it is not already in the file system.
    pub fn get_day_input(day_number: u8) -> String {
        fs::read_to_string(format!("./test/day{day_number}_input.txt")).expect("data")
    }
}
