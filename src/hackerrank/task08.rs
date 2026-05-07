pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut highest_count = 0;
    let mut lowest_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            highest_count += 1;
        } else if score < lowest {
            lowest = score;
            lowest_count += 1;
        }
    }

    vec![highest_count, lowest_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }
}
