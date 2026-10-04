## Lets have a walkthrough what was the inital code and what was wrong
```rust 
use rand::RngExt;
use std::io;

fn main() {
    //to generate a random num
    let mut rng = rand::rng();
    let random_num: u32 = rng.random();
    //very big large random num
    println!("Random number is {}", random_num);
    //random num from 1 to 10
    let new_num: u32 = random_num%10;
    print!("{}", new_num);

    //an empty string
    let mut gnum = String::new();
    //earlier we had this line typecasting an empty "" into an integer which was wrong 
    io::stdin().read_line(&mut gnum).unwrap();

    
    while true {

        //then we didn't had this clear state why is it even needed ?
        gnum.clear();

        io::stdin().read_line(&mut gnum).unwrap();
        //then we didn't had .trim() here which basically counted 5\n "\n" was counted an the program crashed.
        let mut num = gnum.().parse::<u32>().unwrap();

        if  num == new_num {
            break;
        }
    }
}
```