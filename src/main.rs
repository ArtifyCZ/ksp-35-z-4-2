use anyhow::Result;
use std::io::{BufWriter, stdin, stdout, Write};
use token_read::TokenReader;

fn calculate_primes(n: u128) -> Vec<u128> {
    let mut primes: Vec<bool> = Vec::with_capacity(n);

    for i in 0..n {
        primes.push(true);
    }

    for i in 0..(n.sqrt() + 1) {
        if !primes[i] {
            continue;
        }

        let mut j = (i + 2) * 2;
        while j <= n {
            primes[j - 2] = false;
        }
    }

    let mut res = Vec::new();
    let mut primes = primes.iter().enumerate();

    while let Some((i, p)) = primes.next() {
        if *p {
            res.push((i + 2) as u128);
        }
    }
    res
}

fn disassembly_num(n: u128) -> Vec<(u128, u128)> {
    let mut x = n;

    let mut primes = Vec::new();

    for i in 2..(n + 1) {
        if x == 1 {
            break
        }

        if x % i != 0 {
            continue
        }

        let mut times = 0;

        while x % i == 0 {
            times += 1;
            x /= i;
        }

        assert_ne!(i, 0);

        primes.push((i, times));
    }

    assert!(!primes.is_empty(), "primes is empty; n = {}", n);

    primes
}

fn main() -> Result<()> {
    /*let mut input = TokenReader::new(stdin().lock());
    let mut output = BufWriter::new(stdout().lock());
    let (n,): (usize,) = input.line()?;

    for _ in 0..n {
        let (x,): (u128,) = input.line()?;
        let mut primes = disassembly_num(x);
        primes.sort();
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
        write!(output, "\n")?;
    }*/

    let n = 1000;
    dbg!(calculate_primes(n));

    Ok(())
}
