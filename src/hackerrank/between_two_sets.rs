pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let max_a = *a.iter().max().unwrap_or(&1);
    let min_b = *b.iter().min().unwrap_or(&100);
    let mut count = 0;

    for x in max_a..=min_b {
        let is_factor_for_x = a.iter().all(|&val| x % val == 0);
        let is_factor_of_b = b.iter().all(|&val| val % x == 0);

        if is_factor_for_x && is_factor_of_b {
            count += 1;
        }
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
        assert_eq!(get_total_x(&a, &b), 3); // Числа 4, 8 и 16 подходят
    }
}