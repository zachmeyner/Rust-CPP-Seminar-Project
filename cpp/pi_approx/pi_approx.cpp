#include <gmp.h>
#include <math.h>
#include <mpfr.h>
#include <stdio.h>
#include <string>

void calcPreciseTo(unsigned long int outTo);
void calcNextSum(mpq_t sum, unsigned long int n);

mpz_t linear;
mpz_t exponential;
mpq_t multinomial;
mpz_t kth;

mpz_t LINEARCONST;
mpz_t EXPONENTIALCONST;

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Input as pi_approx [DIGITS]\n");
        return 1;
    }

    std::string precision = argv[1];

    for (char const &ch : precision) {
        if (!std::isdigit(ch)) {
            fprintf(stderr, "Input is not a number\n");
            return 1;
        }
    }

    unsigned long int out = std::stoul(precision);

    calcPreciseTo(out);

    return 0;
}

void calcPreciseTo(unsigned long int outTo) {
    unsigned long int iterations = ceilf32(float(outTo) / 14.8);
    unsigned long int floatAccuracy = outTo * 20 + 32;
    mpfr_t frontConst;
    mpq_t badPi;
    mpfr_t goodPi;

    char *outStr = NULL;

    mpf_set_default_prec(floatAccuracy);

    mpfr_init_set_ui(frontConst, 10005, MPFR_RNDN);

    // Set the front constant that is always multiplied by the sum
    mpfr_sqrt(frontConst, frontConst, MPFR_RNDN);
    mpfr_mul_ui(frontConst, frontConst, 426880, MPFR_RNDN);

    // Set the linear, exponential, and multinomial iterative values
    mpz_init_set_ui(linear, 13591409);
    mpz_init_set_ui(exponential, 1);
    mpq_init(multinomial);
    mpq_set_ui(multinomial, 1, 1);
    mpz_init_set_si(kth, -6);

    // Set constant values for linear and exponential iterations
    mpz_init_set_ui(LINEARCONST, 545140134);
    mpz_init_set_str(EXPONENTIALCONST, "-262537412640768000", 10);

    // Init pi-storage vars
    mpq_init(badPi);
    mpq_set_ui(badPi, 0, 1);

    for (unsigned long int sumNum = 0; sumNum <= iterations; sumNum++) {
        calcNextSum(badPi, sumNum);
    }

    mpq_inv(badPi, badPi);

    mpfr_init_set_q(goodPi, badPi, MPFR_RNDN);
    mpfr_mul(goodPi, goodPi, frontConst, MPFR_RNDN);

    mpfr_exp_t e;

    outStr = mpfr_get_str(NULL, &e, 10, outTo, goodPi, MPFR_RNDN);

    printf("%s", outStr);

    // A lot of clears
    mpz_clears(linear, exponential, kth, LINEARCONST, EXPONENTIALCONST);
    mpfr_free_str(outStr);
    mpq_clears(multinomial, badPi);
    mpfr_clears(frontConst, goodPi, (mpfr_ptr)0);
}

void calcNextSum(mpq_t sum, unsigned long int n) {
    mpq_t nextAdd;

    mpz_t monNum;
    mpz_t monDen;
    mpq_t monAdd;

    mpq_init(nextAdd);
    mpq_set_num(nextAdd, linear);
    mpq_set_den(nextAdd, exponential);
    mpq_mul(nextAdd, nextAdd, multinomial);

    mpq_add(sum, sum, nextAdd);
    mpq_clear(nextAdd);

    // Iteratre the multinomial
    // Numerator
    mpz_add_ui(kth, kth, 12);
    // k^3
    mpz_init_set(monNum, kth);
    mpz_mul(monNum, monNum, kth);
    mpz_mul(monNum, monNum, kth);
    // -16k
    mpz_submul_ui(monNum, kth, 16);

    // Denominator
    mpz_init_set_ui(monDen, n + 1);
    mpz_mul(monDen, monDen, monDen);
    mpz_mul(monDen, monDen, monDen);

    // The final bit
    mpq_init(monAdd);
    mpq_set_num(monAdd, monNum);
    mpq_set_den(monAdd, monDen);
    mpq_mul(multinomial, multinomial, monAdd);

    mpq_clears(monAdd);
    mpz_clears(monNum, monDen);

    // Iterate the linear value
    mpz_add(linear, linear, LINEARCONST);
    mpz_mul(exponential, exponential, EXPONENTIALCONST);
}
