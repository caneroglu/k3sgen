use clap::{arg, Command, crate_description, crate_name, Subcommand};
use colored::Colorize;

use clap::Parser;

#[derive(Parser)]
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


    #[command(subcommand)]
    command: Altkomutlar,

}
#[derive(Subcommand)]
enum Altkomutlar{
    Init 

}
 