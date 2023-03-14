use anyhow::Result;
use rayon::prelude::*;
use std::io::{BufWriter, stdin, stdout, Write};
use token_read::TokenReader;

fn calculate_primes(n: u64) -> Vec<u64> {
    let mut primes: Vec<bool> = Vec::with_capacity((n + 1) as usize);

    for _ in 2..=n {
        primes.push(true);
    }

    let mut count = 2;

    {
        let mut p: usize = 2;

        while p * p <= n as usize {
            if primes[p - 2] {
                count += 1;

                let mut i = p * p;
                while i <= n as usize {
                    primes[i - 2] = false;
                    i += p;
                }
            }

            p += 1;
        }
    }

    let mut res = Vec::with_capacity(count);

    for (p, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            res.push((p + 2) as u64);
        }
    }

    res
}

fn disassembly_num(n: u64, primes: &[u64]) -> Vec<(u64, u64)> {
    let mut n = n;

    let mut result = Vec::new();

    for p in primes {
        let mut c = 0;

        if n == 0 || n == 1 {
            break;
        }

        while n % p == 0 {
            c += 1;
            n /= p;
        }

        if 0 < c {
            result.push((*p, c));
        }
    }
    result
}

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());
    let mut output = BufWriter::new(stdout().lock());
    let (n,): (usize,) = input.line()?;

    let (primes, input_numbers) = {
        let mut input_numbers = Vec::with_capacity(n);
        let mut max_number = 0;
        for _ in 0..n {
            let (x, ): (u64, ) = input.line()?;
            if max_number < x {
                max_number = x;
            }
            input_numbers.push(x);
        }
        (calculate_primes(max_number), input_numbers)
    };

    let disassembled = input_numbers.par_iter()
        .map(|x| disassembly_num(*x, &primes))
        .collect::<Vec<_>>();

    for primes in disassembled {
        let mut primes = primes.iter();
        let Some((p, c)) = primes.next() else { todo!() };
        if *c == 1 {
            write!(output, "{}", p)?;
        } else {
            write!(output, "{}^{}", p, c)?;
        }
        for (p, c) in primes {
            if *c == 1 {
                write!(output, "*{}", p)?;
            } else {
                write!(output, "*{}^{}", p, c)?;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}
