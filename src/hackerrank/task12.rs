pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&candle| candle == *max_height).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_cake_candles() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }
}