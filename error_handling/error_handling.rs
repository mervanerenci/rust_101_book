// Simple Error Handling with `match`:
// Simple error handling in Rust can be done using the `Result` type and the `match` expression.
// The `Result` type is an enum that represents either success (`Ok`) or failure (`Err`).
// The `match` expression is used to handle the `Result` and perform different actions based on the success or failure case.


// The `divide` function demonstrates simple error handling using the `Result` type.
// If the divisor `b` is 0, an error is returned with a custom error message.
// Otherwise, the division operation is performed, and the result is returned.
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// In the `main` function, the `divide` function is called with two different arguments.
// The `match` expression is used to handle the `Result` returned by the `divide` function.
// If the `Result` is `Ok`, the result is printed. If the `Result` is `Err`, the error message is printed.
fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

// Handling Errors with `?` Operator
// The `?` operator can be used to propagate errors in Rust.
// It is a shorthand for the `match` expression and is commonly used with functions that return a `Result`.

// The `parse_and_double` function demonstrates the use of the `?` operator to propagate errors.
// It tries to parse the input string into an integer, and if successful, it doubles the value.
// If the parsing operation fails, the error is propagated up the call stack.
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = s.parse()?;
    Ok(num * 2)
}

// In the `main` function, the `parse_and_double` function is called with two different arguments.
// The `match` expression is used to handle the `Result` returned by the `parse_and_double` function.
// If the `Result` is `Ok`, the result is printed. If the `Result` is `Err`, the error message is printed.
fn main() {
    let result = parse_and_double("42");
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let result = parse_and_double("not_a_number");
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

// Defining Custom Errors
// Custom errors can be defined using enums in Rust.
// This allows you to create specific error types for different error cases and handle them accordingly.

// The `MyError` enum is defined to represent two types of errors: `DivisionByZero` and `ParseError`.
// The `From` trait is implemented to convert `std::num::ParseIntError` into `MyError::ParseError`.
#[derive(Debug)]
enum MyError {
    DivisionByZero,
    ParseError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::ParseError(err)
    }
}



// The `parse_and_double` function is updated to use the custom `MyError` enum.
// It checks for the case where the parsed number is 0, and returns a `DivisionByZero` error.
// Otherwise, it doubles the value and returns the result.
fn parse_and_double_custom_err(s: &str) -> Result<i32, MyError> {
    let num: i32 = s.parse()?;
    if num == 0 {
        Err(MyError::DivisionByZero)
    } else {
        Ok(num * 2)
    }
}



// In the `main` function, the `parse_and_double` function is called with two different arguments.
// The `match` expression is used to handle the `Result` returned by the `parse_and_double` function.
// The different error cases are handled separately, with specific actions for `DivisionByZero` and `ParseError`.
fn main() {
    match parse_and_double("42") {
        Ok(result) => println!("Result: {}", result),
        Err(MyError::DivisionByZero) => println!("Error: Division by zero"),
        Err(MyError::ParseError(error)) => println!("Error: {}", error),
    }

    match parse_and_double("not_a_number") {
        Ok(result) => println!("Result: {}", result),
        Err(MyError::DivisionByZero) => println!("Error: Division by zero"),
        Err(MyError::ParseError(error)) => println!("Error: {}", error),
    }
}



// Using `unwrap` and `expect`
// The `unwrap` and `expect` methods can be used to handle `Result` values in a more concise way.
// They provide a convenient way to handle the success case and panic with a custom error message in case of an error.
// 
// In the `main` function, the `parse_and_double` function is called with two different arguments.
// The `unwrap` method is used to handle the `Result` returned by the `parse_and_double` function.
// If the `Result` is `Err`, the program will panic with a default error message.
// The `expect` method is also used to handle the `Result`, and it allows you to provide a custom error message in case of a panic.
fn main() {
    let result = parse_and_double("42").unwrap();
    println!("Result: {}", result);

    let result = parse_and_double("not_a_number").expect("Failed to parse number");
    println!("Result: {}", result);
}