fn main() {
    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
