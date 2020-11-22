struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut pos_typed: usize = 0;
        let mut last_char = ' ';
        for c in name.chars() {
            if last_char != c {
                // 当名字里面无重复的时候， 需要把上次的长摁走完
                loop {
                    match typed.chars().nth(pos_typed) {
                        None => return false,
                        Some(temp_c) => match temp_c == last_char {
                            true => pos_typed += 1,
                            false => break,
                        },
                    }
                }
            }
            // 检测这一次
            match typed.chars().nth(pos_typed) {
                None => return false,
                Some(temp_c) => match temp_c == c {
                    true => pos_typed += 1,
                    false => return false,
                },
            }
            last_char = c;
        }

        // typed最后需要是连续的重复
        while let Some(temp_c) = typed.chars().nth(pos_typed) {
            if last_char != temp_c {
                return false;
            }
            pos_typed += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
