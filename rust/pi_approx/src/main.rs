use rug::Integer;
use rug::Rational;

fn main() {
    calc_precise_to(100_000);
}

fn calc_precise_to(out_to: u32) {
    let front_const = Integer::from(53360)
        * Integer::from(640320 * Integer::from(Integer::u_pow_u(10, 300_000))).sqrt();
    let mut bad_pi = Rational::from(0);
    let mut pi_approx_store = Integer::from(0);

    let mut sum_num: u32 = 0;

    let mut accuracy = 0;

    loop {
        let cpy = bad_pi.clone();
        bad_pi = cpy + calc_next_sum(sum_num);

        let pi_ratio: Rational = &front_const * Rational::from((bad_pi.denom(), bad_pi.numer()));

        let good_pi: Integer = Integer::from(pi_ratio.numer() / pi_ratio.denom());

        accuracy = compare_pi(&good_pi, &pi_approx_store, accuracy);

        pi_approx_store = good_pi;

        if accuracy >= out_to {
            break;
        }
        sum_num += 1;
    }

    let pi_str = pi_approx_store.to_string();

    let finstr: String = pi_str.chars().take(accuracy as usize + 2).collect();

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

    sign * frac_one * frac_two
}

fn compare_pi(pi1: &Integer, pi2: &Integer, start: u32) -> u32 {
    let pi_str1 = pi1.to_string();
    let pi_str2 = pi2.to_string();

    for i in start..pi_str1.len() as u32 {
        if pi_str1.chars().nth(i as usize) != pi_str2.chars().nth(i as usize) {
            return i;
        }
    }
    return start;
}
