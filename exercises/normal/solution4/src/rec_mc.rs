pub fn dp_rec_mc(amount: u32) -> u32 {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let mut amount = amount;
    let mut sum = 0;
    for index in 0..coins.len() {
        let count = amount / coins[index];
        amount -= count* coins[index];
        sum += count;
    }
    sum
}
