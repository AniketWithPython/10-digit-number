# 10-digit-number
## Problem
The Problem statement goes as follows:

<i>Find a 10 digit number such that

The first digit gives the number of zeros in the number

The second digit gives the number of ones in the number

The third digit gives the number of twos in the number

The fourth digit gives the number of threes in the number

The fifth digit gives the number of fours in the number

The sixth digit gives the number of fives in the number

The seventh digit gives the number of sixes in the number

The eighth digit gives the number of sevens in the number

The ninth digit gives the number of eights in the number

The tenth digit gives the number of nines in the number</i>

## Rust solution
This solution uses a brute-force approach and parallelism to achieve the result<sup>*</sup>. Parallelism is achieved using [rayon](https://docs.rs/rayon/latest/rayon/).

## Answer
```
6210001000
```

> ©Aniket Maity 2024

> \* Performance may vary depending on system.
