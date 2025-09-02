fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function!");
    println!("The measurement is: {value}{unit_label}");
}