fn sum_multiples_to_range(multiplied: i32,max_number: i32) -> i32 {
    let mut summed: i32 = 0;
    let mut multiplier: i32 = 1;
    while multiplied * multiplier <= max_number {
        summed += multiplier * multiplier;
        multiplier += 1;
    }
    summed
}
fn main() {
    let sumss: i32 = sum_multiples_to_range(3,1000) + sum_multiples_to_range(5,1000);
    println!("{}",sumss);
}
