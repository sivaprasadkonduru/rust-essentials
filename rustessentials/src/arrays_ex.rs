// This file contains the details about array

pub fn arrays() {

    // Array with data type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array of numbers = {:?}", numbers);

    println!("First element of the array is {}", numbers[0]);
    println!("Second element of the array is {}", numbers[0]);
    println!("Last element of the array is {}", numbers[numbers.len() - 1]);

    // Array without datatype
    let arr = [10, 20, 30, 100];
    println!("Array of numbers = {:?}", arr);

    println!("First element of the array is {}", arr[0]);
    println!("Second element of the array is {}", arr[0]);
    println!("Last element of the array is {}", arr[arr.len() - 1]);

    // Arrays with default values
    let arr1: [i32; 5] = [3; 5];
    println!("Array of numbers = {:?}", arr1);

}



