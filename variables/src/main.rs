fn main() {
    let mut x = 5;

    x = x + 1;

    {
        x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    
    let tup = (500, 6.4, 1);
    
    let (_x, y, _z) = tup;
    
    println!("The value of y is: {y}");

}
