use rug::float::Special;
use rug::Float;
use rug::Integer;
use rug::Rational;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // calc_precise_to(100);
    // return;
    if args.len() < 2 {
        eprintln!("No input");
        std::process::exit(1)
    }

    let precise = args[1].parse::<u64>();

    match precise {
        Ok(_) => {
            if precise.as_ref().unwrap() > &2_000_000 {
                eprintln!("u32 must be less than 2_000_000");
                std::process::exit(1)
            }
            calc_precise_to(precise.unwrap());
        }
        Err(_) => {
            eprintln!("Non u32 type entered. Enter a u32 < 2_000_000.");
            std::process::exit(1)
        }
    }

    // calc_precise_to(precise);
}

fn calc_precise_to(out_to: u64) {
    let float_accuracy = out_to as u32 * 20 + 32;

    let front_const: Float = Float::with_val(
        float_accuracy,
        426880 * Float::with_val(float_accuracy, 10005).sqrt(),
    );

    let mut bad_pi = Rational::from(0);
    let mut pi_approx_store = Float::with_val(float_accuracy, Special::Zero);

    let mut sum_num: u64 = 0;

    let mut accuracy: u64 = 0;

    let consts: Vec<Integer> = vec![
        Integer::from(545140134),
        Integer::from(13591409),
        Integer::from_str_radix("-262537412640768000", 10).unwrap(),
    ];

    // nth_vals[0] = Linear
    // nth_vals[1] = Exponential
    // nth_vals[2] = Multinomial
    // nth_vals[3] = k_n
    let mut nth_vals: Vec<Rational> = vec![
        consts[1].clone().into(),
        Rational::from(1),
        Rational::from(1),
        Rational::from(-6),
    ];

    loop {
        bad_pi = &bad_pi + calc_next_sum(sum_num, &consts, &mut nth_vals);

        let good_pi = Float::with_val(float_accuracy, &front_const * &bad_pi.clone().recip());

        accuracy = compare_pi(&good_pi, &pi_approx_store, accuracy);

        pi_approx_store = good_pi;

        // println!("{}", accuracy);

        if accuracy >= out_to {
            break;
        }
        sum_num += 1;
    }

    let pi_str = pi_approx_store.to_string();

    let finstr: String = pi_str.chars().take(accuracy as usize + 2).collect();

    println!("{}\n{}", finstr, accuracy);
}

fn calc_next_sum(n: u64, consts: &Vec<Integer>, nth_val: &mut Vec<Rational>) -> Rational {
    let ret: Rational = Rational::from(&nth_val[2] * &nth_val[0]) / &nth_val[1];

    // This finds the next multinomial :)
    // ! DO NOT TOUCH OR SO HELP ME
    nth_val[3] += 12;
    let multnom_numer = (&nth_val[3] * Rational::from(&nth_val[3] * &nth_val[3]) - Rational::from(16 * &nth_val[3]));
    let multnom_denom = Rational::from((n + 1) * (n + 1) * (n + 1));
    nth_val[2] *= multnom_numer / multnom_denom;

    // Way easier to find the next linear
    nth_val[0] += &consts[0];

    // Finds the next exponential value
    nth_val[1] *= &consts[2];

    // ret = Rational::from(&nth_val[2] * &nth_val[0]) / &nth_val[1];

    ret
}

fn compare_pi(pi1: &Float, pi2: &Float, start: u64) -> u64 {
    let binding = pi1.to_string();
    let pi_str1 = binding.as_bytes();
    let binding_2 = pi2.to_string();
    let pi_str2 = binding_2.as_bytes();

    for (idx, _val) in pi_str1.iter().enumerate() {
        if pi_str1[idx] != pi_str2[idx] {
            return idx as u64;
        }
    }
    start
}

// for floating point accuracy 8 * 20 + 32
