fn main() {
    extended_sieve(100);
    println!();
}

fn sieve(prime: &mut Vec<usize>, limit: usize) {
    let mut mark = vec![true; limit + 1];

    mark.iter_mut().skip(4).step_by(2).for_each(|p| *p = false);

    for i in 2..f64::sqrt(limit as f64) as usize + 1 {
        if mark[i] {
            for j in (i * i..limit).step_by(i) {
                mark[j] = false
            }
        }
    }

    for (idx, val) in mark.into_iter().enumerate().skip(2) {
        if val {
            prime.push(idx);
            print!("{} ", idx)
        }
    }
}

fn extended_sieve(limit: usize) {
    let size: usize = f64::sqrt(limit as f64) as usize + 1;
    let mut prime: Vec<usize> = vec![];
    sieve(&mut prime, size);
    println!("{:?}", prime);

    let mut low = size;
    let mut high = 2 * size;

    while low < limit {
        if high >= limit {
            high = limit;
        }

        let mut visited = vec![true; size + 1];

        for val in 0..prime.len() {
            let mut low_lim = (low / prime[val]) as usize * prime[val];

            if low_lim < low {
                low_lim += prime[val];
            }

            for j in (low_lim..high).step_by(prime[val]) {
                visited[j - low] = false;
            }
        }
        for i in low..high {
            if visited[i - low] {
                // print!("{} ", i);
            }
        }
        low = low + limit;
        high = high + limit;
    }
}
