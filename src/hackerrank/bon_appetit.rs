pub fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let total_cost: i32 = bill.iter().sum();
    let anna_actual = (total_cost - bill[k as usize]) / 2;
    
    if b == anna_actual {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_actual);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit() {
        let bill = vec![3, 10, 2, 9];
        bon_appetit(&bill, 1, 12); 
    }
}