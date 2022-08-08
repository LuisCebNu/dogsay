extern crate structopt;
extern crate colored;
use  colored::*;
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self,Read};

#[derive(StructOpt)]
struct  Options{
    #[structopt(default_value = "Woof!")]
    ///What does the dog say?
    message: String, // [1]

    #[structopt(short = "d", long="dead")]
    ///Make the dog appear dead
    dead: bool,

    #[structopt(short = "f", long="file", parse(from_os_str))]
    ///Load the dog picture from the specified file
    dogfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long="stdin")]
    ///Read the message from STDIN instead of the argument
    stdin: bool,
}

//We return exitFailure instead of failure::Error
fn main() -> Result<(),ExitFailure> {
    //this function returns an iterator to the argument. 
    //nth(0) is the names of the binary itself (main), and nth(1) is the argument 
    // let message = std::env::args().nth(1)
    //     .expect("Missing the message. Usage: dogsay <message>");
    let options = Options::from_args(); //[2]
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    }else{
        message = options.message;
    }
    let eye = if options.dead {"X"} else {"O"};
    println!("{}",message.bright_yellow().underline().on_purple()); //<- we print the argument here
    match &options.dogfile {
        Some(path) =>{
            let cat_template = std::fs::read_to_string(path).
                with_context(|_| format!("Could not read file {:?}",path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}",&cat_picture);
        },
        None => {

    
            println!(" \\");
            println!("  \\");
            println!("    /^\\");
            println!("  o({eye} {eye}\\",eye = eye.blue().bold());
            println!("    (   \\/");
    
            if message.to_lowercase() == "meow"{
                eprintln!("A dog shouldn't be saying meow! That's what cat say.")
            }
        }
    }
    Ok(())
}
