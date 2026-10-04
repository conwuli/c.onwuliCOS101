use std::io;
fn main() {
    println!("Welcome to Onwuli's Restaurant!");
    println!("Here's our Menu for today");
    let p = "Poundo Yam/Edinkaiko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Ewedu Soup";
    let e = "Eba & Egusi Soup";
    let w = "White rice & Stew";
    let price_p = 3_200.00;
    let price_f = 3_000.00;
    let price_a = 2_500.00;
    let price_e = 2_000.00;
    let price_w = 2_500.00;
    println!("{}---{} \n{}--{} \n{}---{} \n{}---{} \n{}---{}",p,price_p,f,price_f,a,price_a,e,price_e,w,price_w );
    println!("What would you like to order?");
    println!("Please input: \np for {}, \nf for {}, \na for {}, \ne for {}, \nw for {}",p,f,a,e,w );
     let mut input = String::new();
    loop {   
    io::stdin().read_line(&mut input).expect("Invalid input");
    if input.trim() == "p" {
        println!("You have chosen {}",p);
        break;
    } else if input.trim() == "f"{
        println!("You have chosen {}",f);
        break;
    } else if input.trim() == "a" {
        println!("You have chosen {}",a);
        break;
    } else if input.trim() == "e" {
        println!("You have chosen {}",e);
        break;
    } else if input.trim() == "w" {
        println!("You have chosen {}",w);
        break;
    } else {
        println!("Invalid input, try again");
    }
    
    }
    println!("How much of {} would you like to get",input.trim());
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
        let qty:f32 = input2.trim().parse().expect("Invalid input");
    let mut total = qty * 1.0;
    loop {
        if qty >= 0.0 && input.trim() == "p"{
            total = qty * price_p;
            println!("Your total is {}",total );
            break;
        } else if qty >= 0.0 && input.trim() == "f" {
            total = qty * price_f;
            println!("Your total is {}",total);
            break;
        } else if qty >= 0.0 && input.trim() == "a" {
            total = qty * price_a;
            println!("Your total is {}", total);
            break;
        } else if qty >= 0.0 && input.trim() == "e" {
            total = qty * price_e;
            println!("Your total is {}",total);
            break;
        } else if qty >= 0.0 && input.trim() == "w"{
            total = qty * price_w;
            println!("Your total is {}", total);
            break;
        } else {
            println!("Invalid, try again");
        }
    }
    if total > 10_000.00 {
        let discount = 0.05 * total;
        println!("Your discount is: {}",discount);
        println!("Your new total is {}",total - discount );
    }
}
 