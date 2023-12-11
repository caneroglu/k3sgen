use clap::{arg, Command, crate_description, crate_name, Subcommand};
use colored::Colorize;

use clap::Parser;

#[derive(Debug,Parser)]
#[command(author, version, about, long_about = None,arg_required_else_help(true),
before_help(r"

_     _____                          
| | __|___ /  ___   __ _   ___  _ __  
| |/ /  |_ \ / __| / _` | / _ \| '_ \ 
|   <  ___) |\__ \| (_| ||  __/| | | |
|_|\_\|____/ |___/ \__, | \___||_| |_|
                  |___/              
"))]
pub struct CliApp {


    #[arg(long("output"),short('o'),global(true),help("Save STDOUT to an specified file."))]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Altkomutlar,
}
#[derive(Debug,Subcommand)]
pub enum Altkomutlar{

    #[command(about("Initialize configuration file"),long_about("
    Initialize configuration file witttttttttttth long
    "))]
    Init {
        #[arg(long("default"), short('d'),help("generate with default settings."))] 
        default: bool, // Make it FLAG. It doesn't need to be wrapped around with 'Option'. Default = false.
        
        #[arg(long("no-save"), short('n'),help("Print only to STDOUT."))]
        only_stdout: bool,
    }

}
 