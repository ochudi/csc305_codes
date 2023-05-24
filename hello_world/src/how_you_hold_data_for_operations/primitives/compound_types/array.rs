pub fn multiplier(num_arr: &[f64]) -> f64 {
    let mut multiple: f64 = 1.0;
    for i in num_arr {
        multiple *= i;
    }
    println!("{}", multiple);
    multiple
}
