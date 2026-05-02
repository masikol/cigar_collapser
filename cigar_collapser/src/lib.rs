
// >>> Lib functions >>>

pub fn collapse_cigar(str_arg: &String) -> Result<String, String> {
    let mut q_len:          u32 = 0;
    let mut left_clip_len:  u32 = 0;
    let mut right_clip_len: u32 = 0;

    let mut left_clip_char: char = '\0';
    let mut right_clip_char: char = '\0';

    let mut tmp_chars: Vec<char> = vec![];
    let mut first_operation: bool = true;

    for c in str_arg.chars() {
        if c >= '0' && c <= '9' {
            tmp_chars.push(c);
        } else if c == 'M' || c == 'X' || c == '=' || c == 'I' {
            q_len += calc_operation_len(&tmp_chars)?;
            tmp_chars.clear();
            first_operation = false;
        } else if c == 'D' {
            tmp_chars.clear();
            first_operation = false;
        } else if c == 'S' || c == 'H' {
            if first_operation {
                left_clip_len = calc_operation_len(&tmp_chars)?;
                left_clip_char = c;
            } else {
                right_clip_len = calc_operation_len(&tmp_chars)?;
                right_clip_char = c;
            }
            tmp_chars.clear();
            first_operation = false;
        } else {
            return Err(
                format!("Error: invalid char: `{}`", c)
            );
        }
    }

    Ok(
        format_collapsed_cigar(
            q_len,
            left_clip_len,
            right_clip_len,
            left_clip_char,
            right_clip_char
        )
    )
}

fn calc_operation_len(char_vec: &Vec<char>) -> Result<u32, String> {
    if char_vec.is_empty() {
        return Err(
            String::from("Error: invalid CIGAR operation order")
        );
    }
    let digit_str: String = char_vec.iter().collect();

    Ok(
        digit_str.parse().unwrap()
    )
}

// TODO: test
fn format_collapsed_cigar(q_len: u32,
                          left_clip_len: u32,
                          right_clip_len: u32,
                          left_clip_char: char,
                          right_clip_char: char) -> String {
    let q_len_str = String::from(
        format!("{}", format_number(q_len))
    );

    let mut left_clip_len_str = String::from(".");
    if left_clip_char != '\0' {
        left_clip_len_str = String::from(
            format!(
                "{}-{}",
                format_number(left_clip_len),
                left_clip_char
            )
        );
    }
    let mut right_clip_len_str = String::from(".");
    if right_clip_char != '\0' {
        right_clip_len_str = String::from(
            format!(
                "{}-{}",
                format_number(right_clip_len),
                right_clip_char
            )
        );
    }

    format!(
        "{:>15}|{:>15}|{:>15}",
        left_clip_len_str,
        q_len_str,
        right_clip_len_str
    )
}

fn format_number(n: u32) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = Vec::new();

    for (i, &c) in bytes.iter().rev().enumerate() {
        if i != 0 && (i+1)%3 == 1 {
            result.push(b',');
        }
        result.push(c);
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

// <<< Lib functions <<<


// >>> Tests >>>

#[cfg(test)]
mod test_calc_len {
    use super::{calc_operation_len};

    #[test]
    fn test_calc_len_ok() {
        let char_vec = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let left: u32 = calc_operation_len(&char_vec).unwrap();
        let right: u32 = 1234567890;
        assert_eq!(left, right);
    }

    #[test]
    fn test_calc_len_err_empty() -> Result<(), ()> {
        let char_vec = vec![];
        if let Err(_) = calc_operation_len(&char_vec) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    #[should_panic]
    fn test_calc_len_err_invalid_char() {
        let char_vec = vec!['1', 'X', '8'];
        let _: u32 = calc_operation_len(&char_vec).unwrap();
    }
}


#[cfg(test)]
mod test_collapse_cigar {
    use super::{collapse_cigar};

    #[test]
    fn test_cigar_ok_with_clips() {
        let cigar_str = String::from(
            "1234S10M10I5D20X30=333H"
        );
        let left: String = collapse_cigar(&cigar_str).unwrap();
        let right: String = String::from(
            "        1,234-S|             70|          333-H"
        );
        assert_eq!(left, right);
    }

    #[test]
    fn test_cigar_ok_left_clip() {
        let cigar_str = String::from(
            "1234S10M10I5D20X30="
        );
        let left: String = collapse_cigar(&cigar_str).unwrap();
        let right: String = String::from(
            "        1,234-S|             70|              ."
        );
        assert_eq!(left, right);
    }

    #[test]
    fn test_cigar_ok_right_clip() {
        let cigar_str = String::from(
            "10M10I5D20X30=333H"
        );
        let left: String = collapse_cigar(&cigar_str).unwrap();
        let right: String = String::from(
            "              .|             70|          333-H"
        );
        assert_eq!(left, right);
    }

    #[test]
    fn test_cigar_ok_no_clips() {
        let cigar_str = String::from(
            "10M10I5D20X30="
        );
        let left: String = collapse_cigar(&cigar_str).unwrap();
        let right: String = String::from(
            "              .|             70|              ."
        );
        assert_eq!(left, right);
    }

    #[test]
    fn test_cigar_err_empty_operation_len() -> Result<(), ()> {
        let cigar_str = String::from(
            "10M10I5DM20X30="
        );
        if let Err(err_str) = collapse_cigar(&cigar_str) {
            let right = String::from("Error: invalid CIGAR operation order");
            if err_str != right {
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    fn test_cigar_err_invalid_char() -> Result<(), ()> {
        let invalid_char = '*';
        let cigar_str = String::from(
            format!("10M10I5{}M20X30=", invalid_char)
        );
        if let Err(err_str) = collapse_cigar(&cigar_str) {
            let right = String::from(
                format!("Error: invalid char: `{}`", invalid_char)
            );
            if err_str != right {
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    fn test_cigar_err_invalid_unicode_char() -> Result<(), ()> {
        let invalid_char = "Ж";
        let cigar_str = String::from(
            format!("10M10I5{}M20X30=", invalid_char)
        );
        if let Err(err_str) = collapse_cigar(&cigar_str) {
            let right = String::from(
                format!("Error: invalid char: `{}`", invalid_char)
            );
            if err_str != right {
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test_format_number {
    use super::format_number;

    #[test]
    fn test_format_tens() {
        let num: u32 = 11;
        let left = format_number(num);
        let right = String::from("11");
        assert_eq!(left, right);
    }

    #[test]
    fn test_format_hundreds() {
        let num: u32 = 111;
        let left = format_number(num);
        let right = String::from("111");
        assert_eq!(left, right);
    }

    #[test]
    fn test_format_thousands() {
        let num: u32 = 2111;
        let left = format_number(num);
        let right = String::from("2,111");
        assert_eq!(left, right);
    }

    #[test]
    fn test_format_millions() {
        let num: u32 = 3222111;
        let left = format_number(num);
        let right = String::from("3,222,111");
        assert_eq!(left, right);
    }
}

// <<< Tests <<<
