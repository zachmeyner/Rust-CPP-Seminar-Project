use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

fn main() {
    // calc_precise_to(1);
    println!("{}", calc_next_sum(0));
}

fn calc_precise_to(out_to: usize) {
    let front_const: BigDecimal = BigDecimal::from(
        BigDecimal::from(53360) * BigDecimal::sqrt(&BigDecimal::from(640320)).unwrap(),
    );

    let mut bad_pi: BigDecimal = BigDecimal::from(0);

    let mut sum_num: u64 = 0;

    loop {
        bad_pi += calc_next_sum(sum_num);
        if sum_num < 10 {
            break;
        }

        sum_num += 1;
    }

    let good_pi = front_const * bad_pi.inverse();

    println!("{}", good_pi);
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

    println!(
        "{}\n{}\n{}\n{}\n",
        numer_one, denom_one, numer_two, denom_two
    );

    return sign * (numer_one * numer_two) / (denom_one * denom_two);
}

fn factorial(num: u64) -> BigDecimal {
    if num == 0 || num == 1 {
        return BigDecimal::from(1);
    }

    let mut ret = BigDecimal::from(1);

    for j in 2..num + 1 {
        ret *= BigDecimal::from(j);
    }

    return ret;
}

fn spec_power(mut num: u64) -> BigDecimal {
    if num == 0 {
        return BigDecimal::from(1);
    }
    if num == 1 {
        return (BigDecimal::from(8)
            * BigDecimal::from_u64(100100025).unwrap()
            * BigDecimal::from_u64(327843840).unwrap());
    }

    let mut ret = (BigDecimal::from(8)
        * BigDecimal::from_u64(100100025).unwrap()
        * BigDecimal::from_u64(327843840).unwrap());

    while num > 1 {
        ret *= (BigDecimal::from(8)
            * BigDecimal::from_u64(100100025).unwrap()
            * BigDecimal::from_u64(327843840).unwrap());
        num -= 1;
    }

    return ret;
}
