/*
Program Description:

This program dynamically allocates an integer array of size n using malloc.
It stores a sequence of integers in the allocated array, computes their sum,
and calculates their average.

The program outputs all stored values, the total sum, and the average value.
Memory allocated on the heap is explicitly freed before program termination.

Basic validation is performed to ensure that the array size is valid and that
memory allocation succeeds.
*/

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    const char *USAGE_TEXT = "./program2 <input_file.txt>";
    const int MAX_N = 10000000;     // Max of 10^7 inputs

    int n, i;       // Ensure n maxes out at 10^7. int = 4 bytes, 10^7 * 4 bytes = 40MB allocated memory
    int *numbers;
    long long sum = 0;
    double average;

    // Error 1: Incorrect number of arguments passed.
    if (argc != 2) {
        printf("Error: Incorrect usage. Correct Usage: %s\n", USAGE_TEXT);
        return 1;
    }

    // Read file and number of integers for input
    FILE *file = fopen(argv[1], "r");

    // Error 2: No valid file provided
    if (file == NULL) {
        printf("Error: File could not be opened. Correct Usage: %s\n", USAGE_TEXT);
        return 1;
    }

    // Error 3: input file has invalid first argument for n.
    if (fscanf(file, "%d", &n) != 1) {
        printf("Error: First value in %s is not valid\n", argv[1]);
        printf("Ensure that input file is a .txt file and contains only integers separated by spaces, with\n");
        printf("the first integer representing the number of integers to read for input.\n");
        fclose(file);
        return 1;
    }

    // Error 4: n is out of range (1 <= n <= 10^7).
    if (n < 1 || n > MAX_N) {
        printf("Error: n is out of bounds. Ensure that n falls within the interval: [1, 10^7]\n");
        fclose(file);
        return 1;
    }

    // Allocate memory.
    numbers = malloc(n * sizeof(int));

    // Error 5: Malloc fails for some reason. Most likely not enough memory.
    if (numbers == NULL) {
        printf("Error: Memory allocation failed. Ensure you have enough memory and try again\n");
        fclose(file);
        return 1;
    }

    // Read n numbers from input file
    for (i = 0; i < n; i++) {
        // Error 6: Invalid values present in input file.
        if (fscanf(file, "%d", &numbers[i]) != 1) {
            printf("Error: Incorrect value read from %s. Ensure all values in %s are signed integers in range [-2^31, 2^31 - 1], separated by spaces\n", argv[1], argv[1]);
            fclose(file);
            free(numbers);
            return 1;
        }

        sum += numbers[i];      // Assume sum will not overflow. Too many error checks already
    }

    average = (double)sum / n;

    // Print Results
    printf("Sum = %lld\n", sum);
    printf("Average = %.2f\n", average);

    // close all files and stuff
    fclose(file);
    free(numbers);

    return 0;
}