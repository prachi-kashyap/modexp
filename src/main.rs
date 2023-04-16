//! Command-line modular exponentation tool
//! Prachi Kashyap 2023
//! Calculate the modular exponentiation of x raised to y modulo m.
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 0 {
        error();
    }
    if m == 1 {
        return 0;
    }

    let mut z: u128 = 1;
    let mut x = u128::from(x);
    let mut y = u128::from(y);
    let m = u128::from(m);

    while y > 0 {
        if y % 2 == 1 {
            z = (z * x) % m;
        }
        y /= 2;
        x = (x * x) % m;
    }

    u64::try_from(z).unwrap()
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

/// Parse the given string as a `u64`.
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

/// The main function handles command-line arguments, parsing them as `u64` values using the `parsenum()` function.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        error();
    }
    let x = parsenum(&args[1]);
    let y = parsenum(&args[2]);
    let m = parsenum(&args[3]);

    let result = modexp(x, y, m);
    println!("{}", result);
}

/// The `tests` module contains unit tests for the `modexp()` function.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modexp() {
        // Largest prime less than 2**64.
        // https://primes.utm.edu/lists/2small/0bit.html
        let bigm = u64::max_value() - 58;
        assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
        assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
        assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
        //    modular-exponentiation-for-large-numbers/0
        assert_eq!(4, modexp(10, 9, 6));
        assert_eq!(34, modexp(450, 768, 517));
    }
}
