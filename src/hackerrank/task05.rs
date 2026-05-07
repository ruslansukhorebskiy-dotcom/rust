// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apples_count = apples
        .iter()
        .filter(|&&distance| {
            let position = a + distance;
            position >= s && position <= t
        })
        .count() as i32;

    let oranges_count = oranges
        .iter()
        .filter(|&&distance| {
            let position = b + distance;
            position >= s && position <= t
        })
        .count() as i32;

    (apples_count, oranges_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        let expected = (1, 1);

        assert_eq!(count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges), expected);
    }
}