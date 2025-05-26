# 🦀 100 Rust Programs – From Basics to Advanced

This repository contains a collection of 100 Rust programs, organized by increasing complexity and covering all core Rust concepts such as ownership, borrowing, structs, enums, file handling, concurrency, and more.

| No. | Program                            | Concepts Covered                           |
|-----|------------------------------------|--------------------------------------------|
| 1   | Hello World                        | Print to console, basic syntax             |
| 2   | Add Two Numbers                    | Variables, input, arithmetic               |
| 3   | Swap Two Numbers                   | Mutability, temporary var                  |
| 4   | Odd or Even Checker                | If, modulo                                 |
| 5   | Larger of Two Numbers              | If-else                                    |
| 6   | Simple Calculator (+ - * /)        | Match on operator                          |
| 7   | Fahrenheit ⇄ Celsius               | Basic functions                            |
| 8   | Kilometers ⇄ Miles                 | Arithmetic, input                          |
| 9   | Simple Interest                    | Arithmetic, f32                            |
| 10  | Area of Circle                     | Constants, f64                             |
| 11  | Area of Rectangle                  | User input, multiplication                 |
| 12  | Perimeter of Square                | Simple math                                |
| 13  | Compound Interest                  | Power function                             |
| 14  | Grade Calculator                   | If-else chain                              |
| 15  | Leap Year Checker                  | Logical operators                          |
| 16  | Factorial (loop)                   | Loops, u64                                 |
| 17  | Factorial (recursion)             | Recursion, return values                   |
| 18  | Fibonacci (n terms)                | Loops, Vec                                 |
| 19  | Fibonacci (recursive)              | Recursion                                  |
| 20  | Sum of First N Numbers             | For loop, accumulator                      |
| 21  | Count Digits in Number             | While loop, division                       |
| 22  | Sum of Digits                      | Loop, modulo                               |
| 23  | Reverse an Integer                 | Loop, math                                 |
| 24  | Palindrome Number                  | Check equality                             |
| 25  | Prime Number Test                  | For loop, break                            |
| 26  | Primes in Range                    | Range iteration                            |
| 27  | GCD (Euclid)                       | While loop                                 |
| 28  | LCM (using GCD)                    | Math logic, functions                      |
| 29  | Power (a^b)                        | Loop multiplication                        |
| 30  | Multiplication Table               | For loop, printing                         |
| 31  | Print ASCII Value                  | Casting char → u8                          |
| 32  | Print Alphabet                     | A-Z loop                                   |
| 33  | Vowel or Consonant                 | Match statement                            |
| 34  | String Length                      | `len()` method                             |
| 35  | Reverse a String                   | `chars().rev()`                            |
| 36  | Palindrome String                  | String comparison                          |
| 37  | Count Vowels & Consonants          | Match on chars                             |
| 38  | Word Count                         | Simple splitting                           |
| 39  | Find Largest in Array              | Loops, arrays                              |
| 40  | Find Smallest in Array             | Loops                                      |
| 41  | Array Sum & Average                | Array iteration                            |
| 42  | Linear Search                      | Array scan                                 |
| 43  | Binary Search                      | While loop, indices                        |
| 44  | Bubble Sort                        | Nested loops                               |
| 45  | Selection Sort                     | Nested loops                               |
| 46  | Insertion Sort                     | While loop, shifting                       |
| 47  | Merge Two Arrays                   | Extend Vec                                 |
| 48  | Matrix Addition                    | 2D Vec, nested loops                       |
| 49  | Matrix Multiplication              | Nested loops                               |
| 50  | Transpose Matrix                   | Nested loops                               |
| 51  | Pattern – Right Triangle           | Nested loops, printing                     |
| 52  | Pattern – Pyramid                  | Spacing logic                              |
| 53  | Pattern – Inverted Pyramid         | Loops, println!                            |
| 54  | Star Diamond Pattern               | Nested loops                               |
| 55  | ASCII Art Initials                 | String print                               |
| 56  | Simple Stopwatch                   | `std::time::Instant`                       |
| 57  | Random Number Guessing             | `rand` crate, loop                         |
| 58  | Coin Toss Simulation               | Random bool                                |
| 59  | Dice Roll Simulation               | Random range                               |
| 60  | Basic Calculator (CLI args)        | `std::env::args`                           |
| 61  | Read Text File                     | `std::fs`, error handling                  |
| 62  | Write Text File                    | File write, append                         |
| 63  | Append Log Entry                   | Timestamps, write                          |
| 64  | File Copy Utility                  | `fs::copy`                                 |
| 65  | Read CSV Lines                     | Simple parsing                             |
| 66  | Simple JSON Encode                 | `serde_json`                               |
| 67  | Simple JSON Decode                 | Struct + derive                            |
| 68  | Struct for Point                   | Structs, methods                           |
| 69  | Struct with Trait Display          | `fmt::Display`                             |
| 70  | Enum for Traffic Light             | Enums, match                               |
| 71  | Calculator with Enum Ops           | Pattern match                              |
| 72  | Vector CRUD (push/pop)             | Vectors                                    |
| 73  | Stack Using Vec                    | Encapsulation                              |
| 74  | Queue Using VecDeque               | `collections`                              |
| 75  | HashMap Word Frequency             | HashMap insert                             |
| 76  | Ownership Demo                     | Move semantics                             |
| 77  | Borrowing Demo                     | References                                 |
| 78  | Lifetime Example                   | Function returns ref                       |
| 79  | Simple Closure                     | Add two nums                               |
| 80  | Sort with Closure                  | `sort_by`                                  |
| 81  | Custom Iterator                    | `impl Iterator`                            |
| 82  | Generic Swap Function              | Generics                                   |
| 83  | Result Error Handling              | Divide by zero                             |
| 84  | Option Handling                    | Safe unwrap                                |
| 85  | Thread “Hello”                     | Print from threads                         |
| 86  | Threaded Counter                   | `Arc<Mutex<…>>`                            |
| 87  | Channel Message Passing            | `mpsc`                                     |
| 88  | Simple Timer (sleep)               | `thread::sleep`                            |
| 89  | Command-line Progress Bar          | Loops, print!                              |
| 90  | Simple Logger to File              | File append                                |
| 91  | Read Environment Variable          | `std::env`                                 |
| 92  | Unit Test for Add                  | `#[test]`                                  |
| 93  | Benchmark with `instant`           | Time diff                                  |
| 94  | Feature Flag Demo                  | `cfg` attributes                           |
| 95  | Compile-time Constant              | `const`                                    |
| 96  | Mutable Static Counter             | `static mut` (unsafe)                      |
| 97  | Unsafe Pointer Deref               | Raw pointers                               |
| 98  | Simple Trait Object                | Dynamic dispatch                           |
| 99  | Basic Module Split                 | `mod`, `use`                               |
| 100 | Build & Run with Cargo             | Project layout                             |

---

✅ **License**: MIT  
📂 **Topics**: Rust Programming, Problem Solving, Systems Programming, CLI, Algorithms  
🎯 **Goal**: Help you master Rust by practicing 100 fundamental and advanced examples  
