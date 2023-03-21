#include <gmp.h>
#include <mpf2mpfr.h>
#include <mpfr.h>
#include <stdio.h>
#include <string>

void calcPreciseTo(unsigned long int outTo);
void calcNextSum(mpz_t n);
void compare_pi(mpf_t pi_1, mpf_t pi_2);

mpz_t linear;
mpz_t exponential;
mpq_t multinomial;
mpz_t kth;

int main(int argc, char* argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Input as pi_approx [DIGITS]\n");
        return 1;
    }

    std::string precision = argv[1];

    for (char const& ch : precision) {
        if (!std::isdigit(ch)) {
            fprintf(stderr, "Input is not a number\n");
            return 1;
        }
    }
}

void calcPreciseTo(unsigned long int outTo) {
    unsigned long int floatAccuracy = outTo * 20 + 32;
    mpfr_t frontConst;
    mpq_t badPi;
    mpfr_t piStore;

    mpf_set_default_prec(floatAccuracy);

    mpfr_init_set_ui(frontConst, 10005, MPFR_RNDD);

    // Set the front constant that is always multiplied by the sum
    mpfr_sqrt(frontConst, frontConst, MPFR_RNDD);
    mpfr_mul_ui(frontConst, frontConst, 426880, MPFR_RNDD);

    // Set the linear, exponential, and multinomial iterative values
    mpz_init_set_ui(linear, 545140134);
    mpz_init_set_ui(exponential, 1);
    mpq_init(multinomial);
    mpq_set_ui(multinomial, 1, 1);
    mpz_init_set_si(kth, -6);

    //

    unsigned long int accuracy = 0;
    unsigned long int sumNum = 0;

    while (accuracy > outTo) {
    }
}