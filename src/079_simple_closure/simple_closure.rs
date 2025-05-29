fn main() {
    // Define a closure that takes two i32 numbers and returns their sum.
    // The types of the parameters (a and b) and the return type are inferred by the compiler.
    let add = |a, b| a + b;

    // Assign two numbers to variables.
    let num1 = 10;
    let num2 = 5;

    // Call the closure with the two numbers.
    let sum = add(num1, num2);

    // Print the result.
    println!("The sum of {} and {} is {}", num1, num2, sum);

    // You can also explicitly type-annotate the closure.
    let add_explicit: fn(i32, i32) -> i32 = |a, b| a + b;
    let sum2 = add_explicit(20, 30);
    println!("Another sum: {}", sum2);
}