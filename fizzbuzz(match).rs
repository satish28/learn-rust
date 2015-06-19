//fizzbuzz program using match

fn main(){
    for x in 1..101{
        match x{
            _ if x % 15 == 0 => println!("{} FizzBuzz", x),
            _ if x % 5 == 0 => println!("{} Buzz", x),
            _ if x % 3 == 0 =>  println!("{} Fizz", x),
            _ => println!("{}", x),
                
            }
        }
    }
        
