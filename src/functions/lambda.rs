pub fn map() {
    let base_2: Vec<i32> = (1..10).map(|x| i32::pow(2, x)).collect();
    println!("{:?}", base_2);
}

pub fn filter() {
    let is_prime = |n: i32| -> bool {
        if n < 2 {
            return false;
        }

        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    };

    let primos: Vec<i32> = (1..=100)
        .filter(|&x| is_prime(x))
        .collect();
    println!("{:?}", primos);
}
