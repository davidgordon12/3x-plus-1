fn get_input() -> String 
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() 
{
    println!("Enter a number to start");

    //Get input integer
    let n = get_input().trim().parse::<u64>()
    .expect("Number must be positive");

    three_x(n);
}

fn three_x(mut x: u64) 
{
    //Keep running until x = 1, at which point the program gets stuck in a loop (4, 2 ,1)
    while x != 1 
    {
        //If the number is even, divide by 2
        if x % 2 == 0
        {
            x = x / 2;
            println!("{}", x)
        }
        //Otherwise, multiply by 3, and add 1
        else 
        {
            x = x * 3 + 1;
            println!("{}", x)
        }
    }
}
