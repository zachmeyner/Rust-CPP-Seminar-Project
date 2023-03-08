fn main() {
    sieve(/*prime,*/ 100);
    println!();
}

fn sieve(/*prime: &mut Vec<usize>,*/ limit: usize) {
    let mut mark = vec![true; limit + 1];

    mark.iter_mut().skip(4).step_by(2).for_each(|p| *p = false);

    for i in 2..f64::sqrt(limit as f64) as usize + 1 {
        if mark[i] {
            for j in (i * i..limit).step_by(i) {
                mark[j] = false
            }
        }
    }

    for (n, b) in mark.into_iter().enumerate().skip(2) {
        if b {
            print!("{} ", n)
        }
    }
}
