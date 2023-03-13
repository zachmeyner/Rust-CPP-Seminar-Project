#include <iostream>
#include <limits.h>
#include <math.h>
#include <vector>

typedef unsigned long long ull;

void sieve(std::vector<ull> &prime, ull limit);
void ExtendedSieve(ull limit);

int main() {
    ExtendedSieve(100000000);
    return 0;
}

void sieve(std::vector<ull> &prime, ull limit) {
    std::vector<bool> mark(limit + 1, true);

    for (ull i = 4; i <= limit; i += 2) {
        mark[i] = false;
    }

    for (ull i = 3; i <= (ull)sqrt(limit); i += 2) {
        if (mark[i]) {
            for (ull j = i * i; j <= limit; j += 2 * i) {
                mark[j] = false;
            }
        }
    }

    for (ull i = 2; i <= limit; i++) {
        if (mark[i]) {
            prime.push_back(i);
            std::cout << i << " ";
        }
    }
}

void ExtendedSieve(ull limit) {
    ull size = (ull)sqrt(limit) + 1;
    std::vector<ull> prime;

    sieve(prime, size);

    ull low = size + 1;
    ull high = 2 * size;

    while (low < limit) {
        if (high > limit) {
            high = limit;
        }

        std::vector<bool> visited(size + 1, true);

        for (ull x : prime) {
            ull lowLim = (ull)(low / x) * x;

            if (lowLim < low) {
                lowLim += x;
            }

            for (ull j = lowLim; j <= high; j += x) {
                visited[j - low] = false;
            }
        }

        for (ull i = low; i <= high; i++) {
            if (visited[i - low]) {
                std::cout << i << " ";
            }
        }
        low += size;
        high += size;
    }
}