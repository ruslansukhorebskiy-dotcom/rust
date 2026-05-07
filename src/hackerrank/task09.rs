pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut result = 1;

    for bird in 2..=5 {
        if counts[bird] > counts[result] {
            result = bird;
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let birds = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&birds), 4);
    }
}
