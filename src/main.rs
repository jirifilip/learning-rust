use learning_rust::add;
use learning_rust::structs_and_traits::HelloWorldRecord;


fn main() {
    let record = HelloWorldRecord{ width: 15, height: 30 };

    println!("Hello, world!");
    println!("Add result: {}", add(2, 3));
    println!("{}", record);
}
