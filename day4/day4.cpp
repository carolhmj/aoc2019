#include <iostream>
#include <string>
#include <deque>

typedef std::deque<int> digits;
typedef int digit;

digits getDigits(int n) {
    digits digits = {};

    while (n > 0) {
        int digit = n % 10;
        digits.emplace_front(digit);
        n /= 10;
    }

    return digits;
}

bool hasAdjSame(digits digits) {
    for (int i = 0; i < digits.size()-1; i++) {
        if (digits[i] == digits[i+1]) {
            return true;
        }
    }
    return false;
}

bool hasAnyAdjSameNonMatching(digits digits) {
    int numRepeatingDigits = 1;
    digit currDigit = digits[0];

    for (int i = 1; i < digits.size(); i++) {
        // std::cout << "digit " << digits[i] << "\n";
        if (digits[i] == currDigit) {
            // std::cout << "digit " << digits[i] << " is equal to curr\n";
            numRepeatingDigits++;
        } else {
            if (numRepeatingDigits == 2) {
                return true;
            }
            numRepeatingDigits = 1;
            currDigit = digits[i];
        }
    }
    // std::cout << "numrepeatingdigits " << numRepeatingDigits << "\n"; 
    return numRepeatingDigits == 2;
}

bool isMonotonicallyIncreasing(digits digits) {
    for (int i = 0; i < digits.size()-1; i++) {
        if (digits[i] > digits[i+1] ) {
           return false; 
        }
    }
    return true;
}

int main(int argc, char const *argv[]) {
    int start = 264360;
    // int start = 123444;
    int end = 746325;
    // int end = 123445;

    int amountInRangeRule1 = 0;
    int amountInRangeRule2 = 0;

    for (int i = start; i < end; i++) {
        // std::cout << "i is " << i << std::endl;
        auto digits = getDigits(i);
        if (isMonotonicallyIncreasing(digits)) {
            if (hasAnyAdjSameNonMatching(digits)) {
                amountInRangeRule1++;
                amountInRangeRule2++;
            } else if (hasAdjSame(digits)) {
                amountInRangeRule1++;
            }
        }
    }

    std::cout << "Amount of numbers in range (rule 1): " << amountInRangeRule1 << std::endl;
    std::cout << "Amount of numbers in range (rule 2): " << amountInRangeRule2 << std::endl;
    return 0;
}
