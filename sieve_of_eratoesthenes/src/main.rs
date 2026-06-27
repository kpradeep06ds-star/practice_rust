fn gen_prime(n: usize) -> Result<Vec<bool>, String> {
    // Start by assuming every number from 0..=n is prime.
    // The sieve will disprove composites by crossing out multiples.
    if n <= 0{
        return Err(String::from("Please enter a positive value")) ;
    }
    // the above step is an overkill as usize never be negative and 0 is already handled
    // why I did still do it: To understand how Result works
    let mut v: Vec<bool> = vec![true; n + 1];

    // 0 and 1 are not prime.
    // For a general-purpose function, handle n < 2 safely
    // by returning early or using Result.
    v[0] = false;
    v[1] = false;

    // Start from the first prime candidate.
    let mut i = 2;

    // Only base numbers up to sqrt(n) need to be processed.
    // If a composite <= n has a factor greater than sqrt(n),
    // it also has a smaller factor that was already processed.
    while i * i <= n {
        // If i is still true, it is prime.
        // Only primes are used to cross out multiples.
        if v[i] {
            // Core sieve logic:
            // Start from i*i because smaller multiples of i -> Stepanov book says this (I am glad that book I read)
            // were already crossed out by smaller primes.
            for idx in ((i * i)..v.len()).step_by(i) {
                // idx is a multiple of i, so it is composite.
                v[idx] = false;
            }
        }

        i += 1;
    }

    // All indices still marked true are prime numbers.
    Ok(v)
}

fn main() {
    // println!("Hello, world!");
    let n = 100;
    println!("{:?}", gen_prime(n).unwrap());
}
