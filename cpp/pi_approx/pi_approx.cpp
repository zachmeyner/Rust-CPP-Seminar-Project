#include <gmp.h>
#include <mpfr.h>
#include <stdio.h>
#include <string>

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
