// https://www.hackerrank.com/challenges/staircase/problem
pub fn staircase(n: usize) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        result.push_str(&(spaces + &hashes));

        if i != n {
            result.push('\n');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let expected = "   #\n  ##\n ###\n####";
        assert_eq!(staircase(4), expected);
    }
}