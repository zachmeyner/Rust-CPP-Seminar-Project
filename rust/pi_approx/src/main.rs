use rug::Integer;
use rug::Rational;
use rug::{float::Special, Float};

fn main() {
    calc_precise_to(50000);
    // println!("{}", calc_next_sum(0));
}

fn calc_precise_to(out_to: u32) {
    // let front_const = Float::with_val(10_000, 53360 * Float::with_val(10_000, 640320).sqrt());
    let front_const = Float::with_val(1_000_000, 53360) * Float::with_val(10_000, 640320).sqrt();
    let mut bad_pi = Rational::from(0);
    let mut pi_approx_store = Float::with_val(1_000_000, Special::Zero);

    let mut sum_num: u32 = 0;

    let mut accuracy = 0;

    loop {
        let cpy = bad_pi.clone();
        bad_pi = cpy + calc_next_sum(sum_num);
        let good_pi: Float = front_const.clone() * Rational::from((bad_pi.denom(), bad_pi.numer()));

        // let pi_one = Integer::from(good_pi.numer() * Integer::from(Integer::u_pow_u(10, 500)))
        //     / good_pi.denom();
        // let pi_two =
        //     Integer::from(pi_approx_store.numer() * Integer::from(Integer::u_pow_u(10, 500)))
        //         / pi_approx_store.denom();

        accuracy = compare_pi(&good_pi, &pi_approx_store, accuracy);

        pi_approx_store = good_pi;

        // println!("{}", accuracy);

        //  println!("{}", pi_approx_store);

        if accuracy >= out_to {
            break;
        }
        sum_num += 1;
    }

    let pi_str = pi_approx_store.to_string();

    let finstr: String = pi_str.chars().take(accuracy as usize + 2).collect();

    // let fin =
    //     Integer::from(pi_approx_store.numer() * Integer::from(Integer::u_pow_u(10, accuracy)))
    //         / pi_approx_store.denom();
    println!("{}\n{}", finstr, accuracy);
}

fn calc_next_sum(n: u32) -> Rational {
    let sign = if n % 2 == 0 { 1 } else { -1 };

    let frac_one = Rational::from((
        Integer::factorial(6 * n),
        Integer::from(Integer::factorial(n))
            * Integer::from(Integer::factorial(n))
            * Integer::from(Integer::factorial(n))
            * Integer::from(Integer::factorial(3 * n)),
    ));

    let frac_two = Rational::from((
        13591409 + Integer::from(545140134) * Integer::from(n),
        Integer::u_pow_u(640320, 3 * n),
    ));
    // let numer_one = Float::with_val(10_000, Float::factorial(6 * n));
    //
    // let denom_one = Float::with_val(10_000, Float::factorial(n))
    //     * Float::with_val(10_000, Float::factorial(n))
    //     * Float::with_val(10_000, Float::factorial(n))
    //     * Float::with_val(10_000, Float::factorial(3 * n));
    // let numer_two = Float::with_val(10_000, 13591409)
    //     + Float::with_val(10_000, 545140134) * Float::with_val(10_000, n);
    // let denom_two = Float::with_val(10_000, Float::u_pow_u(640320, 3 * n));

    sign * frac_one * frac_two
}

fn compare_pi(pi1: &Float, pi2: &Float, start: u32) -> u32 {
    let pi_str1 = pi1.to_string();
    let pi_str2 = pi2.to_string();

    for i in start..pi_str1.len() as u32 {
        if pi_str1.chars().nth(i as usize) != pi_str2.chars().nth(i as usize) {
            return i;
        }
    }
    return start;
}

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
