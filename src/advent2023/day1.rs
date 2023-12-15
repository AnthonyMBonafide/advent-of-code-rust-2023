/// Representation of each single digit spelled out along with the number it represents.
const NUMBERS: [(&str, u8); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
];
/// Representation of each single digit spelled out in reverse along with the number it represents.
const REV_NUMBERS: [(&str, u8); 10] = [
    ("eno", 1),
    ("owt", 2),
    ("eerht", 3),
    ("ruof", 4),
    ("evif", 5),
    ("xis", 6),
    ("neves", 7),
    ("thgie", 8),
    ("enin", 9),
    ("orez", 0),
];

/// Solution to Day 1 part 1. See test for the expected results
pub fn trebuchet_part_1(input: String) -> i32 {
    input
        .lines()
        .map(|l| get_numeric_chars_from_line(l.to_string()))
        .fold(0, |r, l| r + l.expect("a number to be present"))
}

/// Obtains the first and last numeric characters in the provided String, combines them and returns
/// the result. If a number only contains 1 number it will be treated as the first and last number.
/// If no number is encountered then will return an error
fn get_numeric_chars_from_line(line: String) -> Result<i32, String> {
    let mut first_digit_char: Option<char> = None;
    let mut last_digit_char: Option<char> = None;
    let mut front_index: usize = 0;
    let mut back_index: usize = line.len() - 1;

    for (index, current_char) in line.chars().enumerate() {
        // Traverse over the characters in the line from both front and back at the same time. Once we encounter a
        // number we stop processing from that side. If the indices meet we have traversed the whole line and can
        // proceed with the information we have already.
        if first_digit_char.is_none() {
            // only increment if we need to process more
            front_index = index;
        }

        if front_index >= back_index {
            // the indexes have passed each other so one of the situations have happened:
            // 1. The front index did not encounter a number, but the back did
            // 2  The front index did encounter a number and the back did not
            // 3. Neither encountered a number

            // Situation 1
            if first_digit_char.is_none() && last_digit_char.is_some() {
                first_digit_char = last_digit_char;
            }
            // Situation 2
            else if first_digit_char.is_some() && last_digit_char.is_none() {
                last_digit_char = first_digit_char;
            }
            // Situation 3
            else if first_digit_char.is_none() && last_digit_char.is_none() {
                return Err("no number found".to_string());
            }

            break;
        }

        let chars: Vec<char> = line.chars().collect();
        if first_digit_char.is_none() && current_char.is_ascii_digit() {
            first_digit_char = Some(current_char);
        }

        if last_digit_char.is_none() {
            // only decrement if we need to process more
            back_index = (line.len() - 1) - index;
        }
        let char_from_back = *chars.get(back_index).expect("something");

        if front_index >= back_index {
            // the indexes have passed each other so one of the situations have happened:
            // 1. The front index did not encounter a number, but the back did
            // 2  The front index did encounter a number and the back did not
            // 3. Neither encountered a number

            // Situation 1
            if first_digit_char.is_none() && last_digit_char.is_some() {
                first_digit_char = last_digit_char;
            }
            // Situation 2
            else if first_digit_char.is_some() && last_digit_char.is_none() {
                last_digit_char = first_digit_char;
            }
            // Situation 3
            else if first_digit_char.is_none() && last_digit_char.is_none() {
                return Err("No number found".to_string());
            }

            break;
        }

        if last_digit_char.is_none() && current_char.is_ascii_digit() {
            last_digit_char = Some(char_from_back);
        }

        if first_digit_char.is_some() && last_digit_char.is_some() {
            break;
        }
    }

    // Combine the two chars and make a number
    let mut num_string = String::from(first_digit_char.expect("a value or 0"));
    num_string.push(last_digit_char.expect("a value or 0"));

    let final_result: i32 = num_string.parse().expect("expected a number");

    Ok(final_result)
}

/// Solution to Day 1 part 2. See test for the expected results
pub fn trebuchet_part_2(input: String) -> i32 {
    input
        .lines()
        .map(|l| get_nums_from_line_part2(l.to_string()))
        .sum::<i32>()
}

/// Obtains the first and last numeric characters or single digit number spelled out in the provided
/// String, combines them and returns the result. If a number only contains 1 number it will be
/// treated as the first and last number. If no number is encountered then will return an error
fn get_nums_from_line_part2(line: String) -> i32 {
    let first_digit: i32 = get_first_number_as_u8(line.clone()).expect("a number") as i32;
    let last_digit: i32 = get_last_number_as_u8(line.clone()).expect("a number") as i32;

    (first_digit * 10) + last_digit
}

/// Gets the first number either in the String. The number can be either a numeric character or a
/// single digit spelled out(i.e. "one", "five", "seven", "zero", etc).
fn get_first_number_as_u8(string: String) -> Result<u8, String> {
    get_first_numeric_char_or_number_spelled_lr(string, NUMBERS)
}

/// Gets the last number either in the String. The number can be either a numeric character or a
/// single digit spelled out(i.e. "one", "five", "seven", "zero", etc).
fn get_last_number_as_u8(string: String) -> Result<u8, String> {
    // flip the string and use the number words reversed to reuse the same logic for going from the front
    get_first_numeric_char_or_number_spelled_lr(string.chars().rev().collect(), REV_NUMBERS)
}

/// Obtains the first and last numeric characters or single number spelled out(i.e. "one", "seven", etc.)
/// in the provided String, combines them and returns the result. If a number only contains 1 number
/// it will be treated as the first and last number. If no number is encountered then will return an
/// error
fn get_first_numeric_char_or_number_spelled_lr(
    string: String,
    numbers_tuple: [(&str, u8); 10],
) -> Result<u8, String> {
    // based on size of the string, remove words that it cannot be due to being too small
    let mut possible_numbers: Vec<(&str, u8)> = Vec::new();
    for tuple_number in numbers_tuple {
        if tuple_number.0.len() <= string.len() {
            possible_numbers.push(tuple_number);
        }
    }

    let string_array: Vec<char> = string.chars().collect();
    for (index, current_char) in string_array.iter().enumerate() {
        if (&'0'..=&'9').contains(&current_char) {
            return Ok(current_char.to_digit(10).unwrap() as u8);
        }

        for string_number_association in possible_numbers.iter() {
            if string[index..].starts_with(string_number_association.0) {
                return Ok(string_number_association.1);
            }
        }
    }

    Err("did not find a number".to_string()) // Make a result and return error here
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent2023::day1::trebuchet_part_1;
    use crate::test as test_util;

    #[test]
    fn read_line_two_numbers_at_ends() {
        let test_data = "1aabbaa2".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(12, result.unwrap());
    }

    #[test]
    fn read_line_two_numbers_in_middle() {
        let test_data = "abc34def".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(34, result.unwrap())
    }

    #[test]
    fn read_line_two_numbers_in_middleish_off_center() {
        let test_data = "abc34defghijklmnop".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(34, result.unwrap())
    }

    #[test]
    fn read_line_two_numbers_at_start() {
        let test_data = "56abcdefghi".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(56, result.unwrap())
    }

    #[test]
    fn read_line_two_numbers_at_end() {
        let test_data = "abcdefghi78".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(78, result.unwrap())
    }

    #[test]
    fn read_line_one_numbers_at_start() {
        let test_data = "9abcdefg".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(99, result.unwrap())
    }

    #[test]
    fn read_line_one_numbers_towards_start() {
        let test_data = "abc0defghijklmnop".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(00, result.unwrap())
    }

    #[test]
    fn read_line_one_numbers_towards_end() {
        let test_data = "abcdefghijklmnop1qrst".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(11, result.unwrap())
    }

    #[test]
    fn read_line_one_numbers_at_end() {
        let test_data = "abcdefghijklmnop1".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(11, result.unwrap())
    }

    #[test]
    fn read_line_one_numbers_in_middle() {
        let test_data = "abcdef5fedcba".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(55, result.unwrap())
    }

    #[test]
    fn read_line_one_examples_1() {
        let test_data = "1abc2".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(12, result.unwrap())
    }

    #[test]
    fn read_line_one_examples_2() {
        let test_data = "pqr3stu8vwx".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(38, result.unwrap())
    }

    #[test]
    fn read_line_one_examples_3() {
        let test_data = "a1b2c3d4e5f".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(15, result.unwrap())
    }

    #[test]
    fn read_line_one_examples_4() {
        let test_data = "treb7uchet".to_string();
        let result = get_numeric_chars_from_line(test_data);
        assert_eq!(true, result.is_ok());
        assert_eq!(77, result.unwrap())
    }

    #[test]
    fn trebuchet_example() {
        let test_data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();

        let result = trebuchet_part_1(test_data);
        assert_eq!(142, result)
    }

    #[test]
    fn day1_part1_answer() {
        let result = trebuchet_part_1(test_util::get_day_input(1));
        assert_eq!(54632, result)
    }

    // Part 2

    #[test]
    fn determine_first_number_or_word_simple_word() {
        let test_data = "onezy".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(1, result.unwrap())
    }

    #[test]
    fn determine_first_number_or_word_simple_word2() {
        let test_data = "zzonezy".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(1, result.unwrap())
    }

    #[test]
    fn determine_first_number_or_word_simple_digit() {
        let test_data = "4zzonezy".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(4, result.unwrap())
    }

    #[test]
    fn determine_first_number_or_word_simple_digit_middle() {
        let test_data = "zz4onezy".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(4, result.unwrap())
    }

    #[test]
    fn determine_first_number_or_word_simple_digit_end() {
        let test_data = "zzonzy4".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(4, result.unwrap())
    }

    #[test]
    fn determine_last_number_or_word_simple_digit_end() {
        let test_data = "zzzzzzzzseven".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(7, result.unwrap())
    }

    #[test]
    fn determine_last_number_or_word_simple_digit_towards_end() {
        let test_data = "zzzzzzzzsevenzzzz".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(7, result.unwrap())
    }

    #[test]
    fn determine_last_number_or_word_simple_digit_towards_front() {
        let test_data = "zsevenzzzz".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(7, result.unwrap())
    }

    #[test]
    fn determine_last_number_or_word_simple_digit_front() {
        let test_data = "sevenzzzz".to_string();
        let result = get_first_number_as_u8(test_data);
        assert_eq!(7, result.unwrap())
    }

    #[test]
    fn day1part2_full_example() {
        let test_data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(281, result)
    }

    #[test]
    fn day1part2_example_1() {
        let test_data = "two1nine".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(29, result)
    }

    #[test]
    fn day1part2_example_2() {
        let test_data = "eightwothree".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(83, result)
    }

    #[test]
    fn day1part2_example_3() {
        let test_data = "abcone2threexyz".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(13, result)
    }

    #[test]
    fn day1part2_example_4() {
        let test_data = "xtwone3four".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(24, result)
    }

    #[test]
    fn day1part2_example_5() {
        let test_data = "4nineeightseven2".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(42, result)
    }

    #[test]
    fn day1part2_example_6() {
        let test_data = "zoneight234".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(14, result)
    }

    #[test]
    fn day1part2_example_7() {
        let test_data = "7pqrstsixteen".to_string();
        let result = trebuchet_part_2(test_data);
        assert_eq!(76, result)
    }

    #[test]
    fn day1_part2_answer() {
        let test_data = test_util::get_day_input(1);
        let result = trebuchet_part_2(test_data);
        assert_eq!(54019, result)
    }
}
