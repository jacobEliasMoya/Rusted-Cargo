
fn main() {
    println!("This is a function, some other function is below");
    some_other_function();
    function_with_parameters(79);
    function_with_two_parameters(55, 'J');

    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };  

    println!("The value of y is: {y}");

    let x: u32 = return_an_item();

    println!("The returned value is: {x}");

    let z:u32 = function_variable_retuned(1230);

    println!("The returned value is: {z}");

}

// testing and adding in another function within main
fn some_other_function() {
    println!("This is that other function") // simple print to be used in main 
}

fn function_with_parameters(number: u32) {
    println!("The number in this function is: {number}") // returning the param
}

fn function_with_two_parameters(number: u32, unit_label: char) {
    println!("The number in this function is: {number}:{unit_label}") // returning the 2 params
}

fn return_an_item() -> u32 {
    5 // this is also returning something
}

fn function_variable_retuned(x:u32) -> u32 {
    x + 1 // this is returning something
}

// comments, simple enough just a quick read and that should be good lol