pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> String {
    let total: i32 = bill.iter().sum();
    let anna_share = (total - bill[k]) / 2;

    if b == anna_share {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_refund() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 12), "5");
    }

    #[test]
    fn test_bon_appetit_correct() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }
}