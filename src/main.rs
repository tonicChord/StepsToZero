use std::{io};


fn main() {
    println!("Bir sayı giriniz.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to recieve the input");

    let mut trimmed = input.trim();

    
    match trimmed.parse::<i32>(){
        Ok(i) => println!("Input = {trimmed}"),
        Err(..) => print!("this is not an integer {trimmed}"),
    };
    
    let mut rakam:i32 =trimmed.parse::<i32>().unwrap();
    let mut sayac:u16 = 1;

    
    while rakam > 0{
        print!("Step {sayac} ");
        
        if rakam %2 == 0{

            print!(" {rakam} is even; divided by 2 and obtain");
            rakam = rakam /2;
            println!(" {rakam}\n");
            
        }else if rakam %2 != 0 {
            print!(" {rakam} is odd; substract by 1 and obtained");
            rakam -= 1 ;
            println!(" {rakam}");
        }
        else if rakam == 0{
            println!("Sayı = {rakam}");
            break;
        }
        sayac = sayac +1;
    }
    
    
}
