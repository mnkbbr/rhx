
use std::{env::Args};
pub mod files;
pub use files::make_rhx;

pub fn help_menu(){
    println!("Usage: ");
    println!("rhx <file_name> -<flag> <number>");
    println!("rhx <file_name> -<flag> <number> <\"Sentence\"> (hide word in hex)");
    println!("flags:");
    println!("-b <number> (gen file in Bytes)");
    println!("-kb <number> (gen file in Kbytes)");
    println!("-mb <number> (gen file in Mbytes)");
    println!("-gb <number> (gen file in Gbytes)");
}
pub struct Config{
    name : String,
    size : usize,
    word : Option<String>,
}

pub fn arg_parse( mut args : Args) -> Result<Config, &'static str>{
    
    if args.len() < 4 {
        return Err("Not enough arguments");
    }

    args.next(); // skip prog name

    let file_name = args.next().unwrap();
    let flag = args.next().unwrap();
    let value = args.next().unwrap();  
    let word= args.next();

    let size= size_parse(&flag, &value)?;

    if let Some(val) = &word {
        if val.len() > size{
            return Err("Word length more than file size");
        } 
    } 

    Ok(Config { name: (file_name), size, word })
}

pub fn size_parse(flag: &str, value: &str ) -> Result<usize, &'static str> {

    let x : usize = match value.trim().parse() {
        Ok(val) => val,
        Err(_) => return Err("Incorect_number"),   
    };
    let mult:usize = match flag {
        "-b" => 1,
        "-kb" => 1024,
        "-mb" => 1024*1024,
        "-gb" => 1024*1024*1024,
        _ => return Err("Incorect flag") 
    };
    x.checked_mul(mult).ok_or("Overflow size is too big")
}

