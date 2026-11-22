use rhx::*;
use std::env;

fn main() {
    let cfg = match arg_parse(env::args()) {
        Ok(val) => val,
        Err(msg) => {
            help_menu();
            println!("\n{msg}");
            return;
        }
    };
    
    match make_rhx(cfg) {
        Ok(opt) =>{
            println!("File with random hex generated");
            if let Some(val) = opt {
                println!("Word start in 0x{:X} cell ", val)
            }
        }
        Err(msg) => {
            println!("Error: {msg}");
        }
    };

}
