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

    
    while true {
        //why do we even need this ....cause first time you give a val in gnum 
        //it stores gnum = "5" a string ofc ...wo to tum lateron usko typecast kar ke
        //storing the val in num ....and the second time gnum has already "5" so 
        //you give "8" it concatinates and becomes "58" and hence num =58 
        //which is a kinda fucked up situation
        gnum.clear();

        io::stdin().read_line(&mut gnum).unwrap();
        let mut num = gnum.trim().parse::<u32>().unwrap();

        if  num == new_num {
            break;
        }
    }
}
