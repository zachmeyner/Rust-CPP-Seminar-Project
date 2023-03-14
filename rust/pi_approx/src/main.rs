fn main() {
    calc_precise_to(500);
    // println!("{}", calc_next_sum(0));
}

fn calc_precise_to(out_to: u64) {}

/*
fn calc_precise_to(out_to: u64) {
    let front_const: BigDecimal =
        BigDecimal::from(53360) * BigDecimal::sqrt(&BigDecimal::from(640320)).unwrap();

    let mut bad_pi: BigDecimal = BigDecimal::from(0);
    let mut pi_approx_store: BigDecimal = BigDecimal::from(0);

    let mut sum_num: u64 = 0;

    let mut accuracy = 0;

    loop {
        bad_pi += calc_next_sum(sum_num);
        let good_pi = &front_const * bad_pi.inverse();

        sum_num += 1;

        accuracy = compare_pi(&good_pi, &pi_approx_store, accuracy);

        pi_approx_store = good_pi.clone();

        if accuracy >= out_to {
            break;
        }
    }
    println!("{}", pi_approx_store);
}

fn calc_next_sum(n: u64) -> BigDecimal {
    let sign: BigDecimal = if n % 2 == 0 {
        BigDecimal::from(1)
    } else {
        BigDecimal::from(-1)
    };

    let numer_one = factorial(6 * n);
    let denom_one = (factorial(n) * factorial(n) * factorial(n)) * factorial(3 * n);

    let numer_two = BigDecimal::from_u64(13591409).unwrap()
        + BigDecimal::from_u64(545140134).unwrap() * BigDecimal::from(n);
    let denom_two = spec_power(n);

    // println!(
    //     "{}\n{}\n{}\n{}\n",
    //     numer_one, denom_one, numer_two, denom_two
    // );

    sign * (numer_one * numer_two) / (denom_one * denom_two)
}

fn factorial(num: u64) -> BigDecimal {
    if num == 0 || num == 1 {
        return BigDecimal::from(1);
    }

    let mut ret = BigDecimal::from(1);

    for j in 2..num + 1 {
        ret *= BigDecimal::from(j);
    }

    ret
}

fn spec_power(mut num: u64) -> BigDecimal {
    if num == 0 {
        return BigDecimal::from(1);
    }
    if num == 1 {
        return BigDecimal::from(8)
            * BigDecimal::from_u64(100100025).unwrap()
            * BigDecimal::from_u64(327843840).unwrap();
    }

    let mut ret = BigDecimal::from(8)
        * BigDecimal::from_u64(100100025).unwrap()
        * BigDecimal::from_u64(327843840).unwrap();

    while num > 1 {
        ret *= BigDecimal::from(8)
            * BigDecimal::from_u64(100100025).unwrap()
            * BigDecimal::from_u64(327843840).unwrap();
        num -= 1;
    }

    ret
}

fn compare_pi(pi1: &BigDecimal, pi2: &BigDecimal, start: u64) -> u64 {
    let pistr1 = pi1.to_string();
    let pistr2 = pi2.to_string();

    for idx in start as usize..pistr1.len() {
        if pistr1.chars().nth(idx) != pistr2.chars().nth(idx) {
            return idx as u64;
        }
    }
    start
}
// 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117065836261950984074393921085308210472766668564561791580936405626912908179578838718360526249888389145953252480
*/
