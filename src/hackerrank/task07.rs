fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    x
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().copied().reduce(lcm).unwrap_or(1);
    let gcd_b = b.iter().copied().reduce(gcd).unwrap_or(1);

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}
