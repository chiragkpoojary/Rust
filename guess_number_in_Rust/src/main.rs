use rand::Rng;

use std::io;
fn main() {

    let x=rand::rng().random_range(1..=10);
    println!("Guess the number");



    loop  {
            let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("error");
  
        let mut guess:i32=guess.trim().parse().expect("jjj");
if x==guess{
println!("you win number is {x}");
break;
}else if(x<guess){
println!("go down");
}

else{
println!("go up");
}

    }


}
