fn main() {
    //control flow now
    let number: u8 = 87;

    if number != 4 {
        println!("Number is not equal 4");
    } else {
        println!("Number is equal 4");
    }

    let number_one: u32 = 456;

    if number_one > 2 {
        println!("Three is less than two");
    } else {
        println!("Three is not less than two");
    }

    let number_three: u8 = 4;

    if number_three == 1 {
        println!("Number is 1");
    } else if number_three == 2 {
        println!("Number is 2");
    } else if number_three == 3 {
        println!("Number is 3");
    } else if number_three == 4 {
        println!("Number is 4");
    }

    let condition: bool = false;
    let conditional_if: i8 = if condition { 4 } else { 5 };
    println!("This conditional number is {conditional_if}");

    // loop {
    //     println!("This is an infitinte loop");
    // }

    let mut counter: u64 = 0;

    let result: u64 = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("This is a counter, the res is {result}");

    let mut count: u32 = 0;

    'counting_up: loop {
        println!("Count = {count}");

        let mut remaining: u32 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut while_number: u32 = 0;

    while while_number != 4 {
        println!("While Number is {while_number}");
        while_number += 1;
    }

    println!("While number has been reached!");

    let array: [u32; 5] = [10, 20, 30, 40, 50];
    let mut arr_index: usize = 0;

    while arr_index < 5 {
        println!("The value is {}", array[arr_index]);
        arr_index += 1;
    }

    for element in array {
        println!("This is my element {element}")
    }

    for number in 1..=84 {
        println!("This is the number: {number}")
    }

    println!("Liftoff brother!")
}
