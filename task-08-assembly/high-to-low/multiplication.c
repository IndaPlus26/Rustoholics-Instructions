/***
 * multiplication.c
 * - Reference file for subassignment "Lower level => Higher level"
 * 
 * Author: Tobias Hansson <tohanss@kth.se>, Viola Söderlund <violaso@kth.se>
 * 
 * Last updated: 2024-11-02
 * Dmitry Chirin <dmitryc@kth.se> added a main function
 */

#include <stdio.h>

/**
 * Multiply given factors using addition.
 * @return The product of the multiplication of the factors a and b.
 */
int multiply(int a, int b) {
    int i, sum = 0;
    for (i = 0; i < a; i++)
        sum += b;
    return sum;
}

/**
 * Calculate faculty using addition.
 * @return The facorial of n.
 */
int faculty(int n) {
    int i, fac = 1;
    for (i = n; i > 1; i--)
        fac = multiply(fac, i);
    return fac;
}

/**
 * Main function to test multiply and facutly functions
 */
int main() {
    // a * b = 221
    printf("13 * 17 = %d\n", multiply(13, 17));
    // 7! = 5040
    printf("7! = %d\n", faculty(7));
};