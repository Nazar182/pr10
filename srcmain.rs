fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = ((n % len) + len) % len;
    let split = shift as usize;

    format!("{}{}", &s[len as usize - split..], &s[..len as usize - split])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }
}
