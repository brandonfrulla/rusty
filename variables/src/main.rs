fn main() {
    let mut x = 5;

    x = x + 1;

    {
        x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
