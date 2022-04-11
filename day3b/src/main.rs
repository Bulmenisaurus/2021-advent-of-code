fn filter_numbers(numbers: Vec<&str>, nth_digit: i32, filter_more_popular_char: bool) -> &str {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let nth_digits: Vec<String> = numbers
        .iter()
        .map(|number| number.chars().nth(nth_digit as usize).unwrap().to_string())
        .collect();

    let nth_digits: Vec<&str> = nth_digits.iter().map(|digit| digit.as_str()).collect();

    let amount_of_ones = nth_digits
        .iter()
        .filter(|digit| digit == &&String::from("1"))
        .count();

    let amount_of_zeroes = numbers.len() - amount_of_ones;

    let filter_char = if filter_more_popular_char {
        if amount_of_ones >= amount_of_zeroes {
            '1'
        } else {
            '0'
        }
    } else {
        #[allow(clippy::else_if_without_else)]
        if amount_of_ones < amount_of_zeroes {
            '1'
        } else {
            '0'
        }
    };

    let filtered_numbers = numbers
        .iter()
        .filter(|number| number.chars().nth(nth_digit as usize).unwrap() == filter_char)
        // this is weird
        .map(|a| a as &str)
        .collect();

    filter_numbers(filtered_numbers, nth_digit + 1, filter_more_popular_char)
}

fn main() {
    let input = include_str!("../input.txt");
    let numbers: Vec<&str> = input.split('\n').collect();

    let oxygen_generator_setting = filter_numbers(numbers.to_vec(), 0, true);
    let co2_scrubber_setting = filter_numbers(numbers, 0, false);

    let oxygen_generator_setting = i32::from_str_radix(oxygen_generator_setting, 2).unwrap();
    let co2_scrubber_setting = i32::from_str_radix(co2_scrubber_setting, 2).unwrap();

    println!("{}", oxygen_generator_setting * co2_scrubber_setting);
}
