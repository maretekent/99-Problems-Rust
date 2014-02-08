fn is_prime(n: int) -> bool {
    if n <= 2 {
        return n == 2;
    }
    let mut primes = ~[2];
    let mut i = 1;
    while std::num::pow(i, 2) < n {
        let &p = primes.last().unwrap();
        if n % p == 0 {
            return false;
        }
        i += 2;
        if primes.iter().all(|&x| i%x != 0) {
            primes.push(i);
        }
    }
    true
}

fn main() {
    for n in range(0, 100) {
        if is_prime(n) {
            println!("{}", n);
        }
    }
}


