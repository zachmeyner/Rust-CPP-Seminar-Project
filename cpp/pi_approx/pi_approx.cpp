#include <gmp.h>
#include <mpf2mpfr.h>
#include <mpfr.h>
#include <stdio.h>
#include <string>

void calcPreciseTo(unsigned long int outTo);
void calcNextSum(mpq_t sum, unsigned long int n);
unsigned long comparePi(mpfr_t pi_1, mpfr_t pi_2);

mpz_t linear;
mpz_t exponential;
mpq_t multinomial;
mpz_t kth;

mpz_t LINEARCONST;
mpz_t EXPONENTIALCONST;

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
    mpfr_init_set_ui(piStore, 0, MPFR_RNDD);

    unsigned long int accuracy = 0;
    unsigned long int sumNum = 0;

    while (accuracy < outTo) {
        calcNextSum(badPi, sumNum);
    }
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

unsigned long comparePi(mpfr_t pi_1, mpfr_t pi2) {}