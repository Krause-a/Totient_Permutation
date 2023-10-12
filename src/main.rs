fn main() {
    let args : Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Expected exactly one argument.");
        return;
    }

    let Ok(value) = args[1].parse::<usize>() else {
        eprintln!("Could not parse argument as unsigned integer.");
        return;
    };

    let mut smallest_ratio = f64::MAX;
    let mut smallest_n = usize::MAX;

    for i in 2..=value {
        let tot = totient(i);

        if check_permutation(i, tot) {
            let ratio = i as f64 / tot as f64;
            if ratio < smallest_ratio {
                smallest_ratio = ratio;
                smallest_n = i;
                //println!("phi({}) = {} -> {}", i, tot, smallest_ratio);
            }
        }
    }

    println!("phi({}) = {} -> {}", smallest_n, totient(smallest_n), smallest_ratio);
    println!("n is {}", smallest_n);
}

//Euler's totient function. Also known as Euler's phi function.
fn totient(mut n:usize) -> usize {
    let mut count = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            count -= count / i;
        }
        i += 1;
    }

    if n > 1 {
        count -= count / n;
    }

    count
}

fn check_permutation(a:usize, b:usize) -> bool {
    count_digits(a) == count_digits(b)
}

fn count_digits(n:usize) -> [usize; 10] {
    let mut counts = [0; 10];
    let mut num = n;

    while num > 0 {
        counts[(num % 10)] += 1;
        num /= 10;
    }

    counts
}
