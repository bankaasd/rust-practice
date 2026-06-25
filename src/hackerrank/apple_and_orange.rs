pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apples_count = 0;
    let mut oranges_count = 0;

    for &apple in apples {
        let pos = a + apple;
        if pos >= s && pos <= t {
            apples_count += 1;
        }
    }

    for &orange in oranges {
        let pos = b + orange;
        if pos >= s && pos <= t {
            oranges_count += 1;
        }
    }

    println!("{}", apples_count);
    println!("{}", oranges_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
    }
}