use std::io;
use std::process;
fn main(){
    loop{
        let mut separate = String::new();
        for i in 0..100{
            separate.push_str("-");
        }  
        println!("{}\n
                1-減法\n
                2-加法\n
                3-結束\n{}",separate,separate);  
        let mut value = String::new();
        let mut num1 = String::new();
        let mut num2 = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        if value.trim() == "3"{
            process::exit(0x0100);
        }
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        match value.trim(){
            "1" => {
                println!("答案: {}",num1.trim().parse::<i32>().unwrap() + num2.trim().parse::<i32>().unwrap());
            },
            "2" =>{
                println!("答案: {}",num1.trim().parse::<i32>().unwrap() - num2.trim().parse::<i32>().unwrap());
            },
            _ => {
                println!("錯誤");
            }
        }
    }
}