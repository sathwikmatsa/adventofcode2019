fn matches_password_criteria(i: u32) -> (bool, bool) {
    let num: String = i.to_string();
    let mut same_adj_digits_strict = false;
    let mut same_adj_digits = false;
    let mut never_decrease;
    if num.len() == 6 {
        let mut digit_iter = num.chars();
        let mut prev_prev = digit_iter.next().unwrap();
        let mut prev = digit_iter.next().unwrap();
        let mut same_group_count = if prev == prev_prev {
            same_adj_digits = true;
            2
        } else {
            1
        };
        never_decrease = prev >= prev_prev;

        for d in digit_iter {
            if d < prev {
                never_decrease = false;
            }
            if d == prev {
                same_group_count += 1;
                same_adj_digits = true;
            } else {
                if prev == prev_prev && same_group_count < 3 {
                    same_adj_digits_strict = true;
                }
                same_group_count = 1;
            }

            prev_prev = prev;
            prev = d;
        }
        return (
            same_adj_digits && never_decrease,
            (same_adj_digits_strict || same_group_count == 2) && never_decrease,
        );
    }
    (false, false)
}

fn main() {
    let start = 193_651;
    let end = 649_729;
    let mut p1_viable_passwords = 0;
    let mut p2_viable_passwords = 0;

    for i in start..=end {
        let (c1, c2) = matches_password_criteria(i);
        if c1 {
            p1_viable_passwords += 1;
        }
        if c2 {
            p2_viable_passwords += 1;
        }
    }

    println!("{}", p1_viable_passwords);
    println!("{}", p2_viable_passwords);
}
