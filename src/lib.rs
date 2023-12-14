use std::fs;

pub mod advent2023;


fn get_day_input(day_number: u8) -> String {
    fs::read_to_string(format!("./test/day{day_number}_input.txt")).expect("data")
}

#[cfg(test)]
mod test {
    use crate::get_day_input;

    #[test]
    fn test_day_1(){
        let input = get_day_input(1);
        assert_eq!(input.is_empty(), false)
    }
}