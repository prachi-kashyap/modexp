# Modular Exponentiation Tool

## Author

Prachi Kashyap

## Project Name

Command-line Modular Exponentiation Tool

## Description

This project implements a command-line tool for calculating the modular exponentiation of x raised to y modulo m using Rust. The goal is to efficiently compute the result even for large numbers without causing an overflow.

## Implementation

### modexp Function

The main function modexp(x: u64, y: u64, m: u64) -> u64 calculates the modular exponentiation of x raised to y modulo m. We use the following steps to calculate the result:

1. Check if m is 0 would cause an overflow
2. If m is 1, the result is always 0
3. Initialize variables z, x, and y as u128 to avoid overflow during calculations
4. If y is odd, multiply z by x and take the result modulo m.
5. Divide y by 2.
6. Square x and take the result modulo m.
7. Convert the final result back to u64 and return it.

### Helper Functions

- `error()`: Prints a usage error message and exits the program with exit status 1.
- `parsenum(s: &str) -> u64`: Parses a string as a `u64` value. Calls the `error()` function on error.

### Main Function

The main function collects command-line arguments, checks for the correct number of arguments, and parses them as `u64` values using the `parsenum()` function. It then calls `modexp()` with the parsed values and prints the result to the standard output.

## Challenges and Outcomes

The main challenge faced during the development was handling large numbers and avoiding overflow. The use of u128 type for intermediate calculations helped overcome this challenge.

Another challenge was handling edge cases and ensuring the correctness of the algorithm. This was addressed by writing unit tests for the `modexp()` function with a variety of inputs, including some large numbers.

## Testing

The program is tested using the following test cases:

Large prime numbers and large exponents.
Edge cases involving the largest prime less than 2\*\*64.
Examples from modular-exponentiation-for-large-numbers.
The tests have been carefully crafted to ensure the correctness and efficiency of the implemented algorithm.

## Usage

To use the command-line tool, compile the Rust code and run it with the following syntax:

cargo run 2 20 17

## References

1. https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
2. https://canvas.pdx.edu/courses/68012/assignments/682966
3. https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
4. https://practice.geeksforgeeks.org/problems/modular-exponentiation-for-large-numbers5537/1
5. https://chat.openai.com/
