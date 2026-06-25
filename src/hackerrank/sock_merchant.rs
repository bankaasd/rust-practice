use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut sock_counts = HashMap::new();
    
    for &sock in ar {
        *sock_counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in sock_counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }
}