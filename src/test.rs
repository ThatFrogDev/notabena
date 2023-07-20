fn main() {
    divide(10, 2, |result| {
        if result.is_err() { println!("Error: {}", error); }
        println!("Result: {}", result);

        divide(20, 0, |nested_result| {
            if nested_result.is_err() { println!("Nested Error: {}", nested_error); }
            println!("Nested Result: {}", nested_result);
        });
    });
}
