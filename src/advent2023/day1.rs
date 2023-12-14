use std::ops::Add;

fn trebuchet(input: String) -> i32 {
    let input = crate::get_day_input(1);

    input.lines().map(|l| {
        get_nums_from_line(l.to_string())
    }).fold(0, |r, l| {
        r + l
    })
}

fn get_nums_from_line(line: String) -> i32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    let mut front_index: usize = 0;
    let mut back_index: usize = 0;

    for (i, c) in line.chars().enumerate() {
        // Traverse over the characters in the line from both front and back at the same time. Once we encounter a
        // number we stop processing from that side. If the indices meet we have traversed the whole line and can
        // proceed with the information we have already.
        if first_digit == None {
            // only increment if we need to process more
            front_index = i;
        }

        if last_digit == None {
            // only decrement if we need to process more
            back_index = (line.len() - 1) - i;
        }


        if front_index >= back_index {
            // the indexes have passed each other so one of the situations have happened:
            // 1. The front index did not encounter a number, but the back did
            // 2  The front index did encounter a number and the back did not
            // 3. Neither encountered a number

            // Situation 1
            if first_digit == None && last_digit != None {
                first_digit = last_digit.clone();
            }

            // Situation 2
            else if first_digit != None && last_digit == None {
                last_digit = first_digit.clone()
            }

            // Situation 3
            else if first_digit == None && last_digit == None {
                return 0;
            }

            break;
        }

        let char_from_front = c;
        let lz: Vec<char> = line.chars().collect();
        let char_from_back = lz.get(back_index).expect("something").clone();
        if first_digit == None && c >= '0' && c <= '9' {
            first_digit = Some(c);
        }

        if last_digit == None && char_from_back >= '0' && char_from_back <= '9' {
            last_digit = Some(char_from_back);
        }

        if first_digit != None && last_digit != None {
            break;
        }
    }

    // Combine the two chars and make a number
    let mut num_string = String::from(first_digit.expect("a value or 0"));
    num_string.push(last_digit.expect("a value or 0"));

    let re: i32 = num_string.parse().expect("expected a number");
    return re;
}

#[cfg(test)]
mod test {
    #[test]
    fn read_line_two_numbers_at_ends() {
        let test_data = "1aabbaa2".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(12, result)
    }

    #[test]
    fn read_line_two_numbers_in_middle() {
        let test_data = "abc34def".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(34, result)
    }

    #[test]
    fn read_line_two_numbers_in_middleish_off_center() {
        let test_data = "abc34defghijklmnop".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(34, result)
    }

    #[test]
    fn read_line_two_numbers_at_start() {
        let test_data = "56abcdefghi".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(56, result)
    }

    #[test]
    fn read_line_two_numbers_at_end() {
        let test_data = "abcdefghi78".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(78, result)
    }

    #[test]
    fn read_line_one_numbers_at_start() {
        let test_data = "9abcdefg".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(99, result)
    }

    #[test]
    fn read_line_one_numbers_towards_start() {
        let test_data = "abc0defghijklmnop".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(00, result)
    }

    #[test]
    fn read_line_one_numbers_towards_end() {
        let test_data = "abcdefghijklmnop1qrst".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(11, result)
    }

    #[test]
    fn read_line_one_numbers_at_end() {
        let test_data = "abcdefghijklmnop1".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(11, result)
    }

    #[test]
    fn read_line_one_numbers_in_middle() {
        let test_data = "abcdef5fedcba".to_string();
        let result = super::get_nums_from_line(test_data);
        assert_eq!(55, result)
    }
}