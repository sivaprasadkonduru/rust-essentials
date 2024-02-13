// program to print n fibonacci numbers

pub fn fib_num() {
    let mut a = 0;
    let mut b = 1;

    let mut input_text = String::new();
    println!("Enter any number: ");
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line.");
    let n= input_text.trim().parse().expect("The entered number is: ");

    for _ in 0..=n {
        println!("{}", a);
        let c = a + b;
        //println!("{}", c);

        a = b;
        b = c;
    }
}