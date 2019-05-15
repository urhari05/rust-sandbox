pub fn run() {
    //Print to console
    println!("Hello World!!");
    //basic formatting
    println!("{} is from {}","harish","hubli");

    //positional arguements
    println!("{0} is from {1} and {0} likes to {2}", "Harish","Bangalore","Code");

    //Named Arguements
    println!("{name} likes to play {activity}", name="harish", activity="cricket");

    //placeholder traits
    println!("Binary : {:b} Hex: {:x} Octal: {:0}",10,10,10);

    //placeholder fordebug traits
    println!("{:?}",(12, true, "harish"));

    //basic math
    println!("10 + 10 is {}", 10+10);
}