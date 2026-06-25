pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut high_count = 0;
    let mut low_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            high_count += 1;
        } else if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    vec![high_count, low_count]
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