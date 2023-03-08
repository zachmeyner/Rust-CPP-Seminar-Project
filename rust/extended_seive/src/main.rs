fn main() {
    extended_sieve(u32::MAX as usize);
    // extended_sieve(100);
    println!();
}

fn sieve(prime: &mut Vec<usize>, limit: usize) {
    let mut mark = vec![true; limit + 1];

    mark.iter_mut().skip(4).step_by(2).for_each(|p| *p = false);

    for i in (3..f64::sqrt(limit as f64) as usize + 1).step_by(2) {
        if mark[i] {
            for j in (i * i..limit).step_by(2 * i) {
                mark[j] = false
            }
        }
    }

    for (idx, val) in mark.into_iter().enumerate().skip(2) {
        if val {
            prime.push(idx);
            // print!("{} ", idx)
        }
    }
}

fn extended_sieve(limit: usize) {
    let size: usize = f64::sqrt(limit as f64) as usize + 1;
    let mut prime: Vec<usize> = vec![];
    sieve(&mut prime, size);
    // println!("{:?}", prime);

    let mut low = size + 1;
    let mut high = 2 * size;

    while low < limit {
        if high > limit {
            high = limit;
        }

        let mut visited = vec![true; size + 1];

        for val in &prime {
            let mut low_lim = (low / val) as usize * val;

            if low_lim < low {
                low_lim += val;
            }

            for j in (low_lim..high).step_by(*val) {
                visited[j - low] = false;
            }
        }
        for i in low..high {
            if visited[i - low] {
                // print!("{} ", i);
            }
        }
        low += size;
        high += size;
    }
}
