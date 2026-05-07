pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = [0; 101];

    for &sock in ar {
        counts[sock as usize] += 1;
    }

    counts.iter().map(|count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&socks), 3);
    }
}
