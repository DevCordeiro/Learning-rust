fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function!");
    println!("The measurement is: {value}{unit_label}");
}