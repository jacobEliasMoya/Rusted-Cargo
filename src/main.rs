fn main() {
    println!("This is a function, some other function is below");
    some_other_function();
    function_with_parameters(79);
    function_with_two_parameters(55,'J');
}

// testing and adding in another function within main
fn some_other_function() {
    println!("This is that other function")
}

fn function_with_parameters(number: u32) {
    println!("The number in this function is: {number}")
}

fn function_with_two_parameters(number: u32, unit_label: char) {
    println!("The number in this function is: {number}:{unit_label}")
}
