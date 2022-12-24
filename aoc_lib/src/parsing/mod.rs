use std::str::FromStr;

const DIGIT_CHARS: [char; 10] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Extracts all the integers separated by any character(s) in an arbitrary
/// string and parses them into T. Make sure T implements a FromStr that can
/// parse any string containing a number. Also matches negative numbers so make
/// sure the FromStr implementation of T handles negative numbers or your input
/// is devoid of them.
pub fn get_numbers<T: FromStr>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    let mut digit_indices = s.match_indices(DIGIT_CHARS).map(|(i, _)| i);

    let mut nums: Vec<T> = Vec::new();
    let mut num_start_index = digit_indices.next().unwrap();
    let mut last_index = num_start_index;

    loop {
        if let Some(i) = digit_indices.next() {
            if i > last_index + 1 {
                nums.push(s[num_start_index..=last_index].parse()?);
                num_start_index = if let Some("-") = s.get(i - 1..=i - 1) {
                    i - 1
                } else {
                    i
                };
            }
            last_index = i;
        } else {
            nums.push(s[num_start_index..=last_index].parse()?);
            break;
        }
    }
    Ok(nums)
}
