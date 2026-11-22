use rand;
use std::{env::Args, fs::File, io::{self, Seek, Write}};

pub fn help_menu(){
    println!("Usage: ");
    println!("rhx {{file_name}} -{{flag}} {{number}}");
    println!("rhx {{file_name}} -{{flag}} {{number}} {{\"Sentence\"}} (hide word in hex)");
    println!("flags:");
    println!("-b {{number}} (gen file in Bytes)");
    println!("-kb {{number}} (gen file in Kbytes)");
    println!("-mb {{number}} (gen file in Mbytes)");
    println!("-gb {{number}} (gen file in Gbytes)");
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

pub fn make_rhx(cfg : Config) -> Result<Option<usize>, String> {
    let mut file = make_file(&cfg.name)?;
    let mut res : Option<usize> = Option::None; 

    if let Err(_) = random_fill(&mut file, cfg.size){
        return Err(String::from("Can't write file ")) ;
    }
    if let Some(val) = &cfg.word{
        let value = place_word(cfg.size, val);
        file.seek(io::SeekFrom::Start(value as u64)).unwrap();
        file.write_all(val.as_bytes()).unwrap();
        res = Some(value);
    }
    Ok(res)
}

fn random_fill(file : &mut File, file_size: usize) -> io::Result<()> {
    let mut buf: Vec<u8> = vec![0u8; file_size];
    for el in buf.iter_mut()  {
        *el = rand::random_range(0..=255);
    }
    file.write_all(&buf)?;
    Ok(())
}

fn place_word(file_len: usize, word : &str) -> usize {
    
    let mut start_pos = rand::random_range(0..file_len);

    if start_pos + word.len() > file_len {
        let difference = start_pos + word.len() - file_len;
        start_pos = start_pos - difference;
    };
    start_pos
}
fn make_file( file_name : &str) -> Result<File, String> {
    match File::create_new(file_name)  {
        Ok(val) => Ok(val),
        Err(_) => {
            println!("File is already existence \nRewrite this(Y/N)?");
            let mut answer = String::new();
            if let Err(msg) = std::io::stdin().read_line(&mut answer){
                return Err(msg.to_string()) ;
            }
            
            if answer.trim().to_lowercase() == "y"  {
                match File::create(file_name) {
                    Ok(val) => Ok(val),
                    Err(a) => {return Err(a.kind().to_string())}
                }
            }else {
                return Err("Cannot rewrite file".to_string());
            }
        }
    }
}