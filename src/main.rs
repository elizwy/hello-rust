use ferris_says::say;
use std::io::{stdout,BufWriter};

fn main() {

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();


    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6;
    four.call();
    
}


enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

impl IpAddrKind{
    fn call(&self){
        println!("test")
    }
}

struct  IpAddr{
    kind: IpAddrKind,
    address: String,
}  
