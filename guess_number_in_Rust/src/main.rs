use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {

    let x=rand::rng().random_range(1..=10);
    println!("Guess the number");



    loop  {
            let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("error");
  
        let  guess:i32=match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };


match x.cmp(&guess){
    Ordering::Less=>println!("go dowm"),
    Ordering::Greater=>println!("go up"),
    Ordering::Equal=>{println!("win!!!");break;}
}

    }


}
