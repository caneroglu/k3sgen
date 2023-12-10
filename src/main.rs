

/*


1. pull olarak cihazlara gönderecek.
2. ssh kullanacak
3. duruma göre en güncel sürümü indirecek veya belirttiğin imajı gönderecek.
4. cihazlarda server - worker olarak *temel* konfigürasyonları gönderecek.
5. herhangi bir server'den kubeconfig dosyasını o anki pc'ye indirecek.
6. en önemlisi hata durumları.

ekstra konfigleri tanımlamaya gerek yok. Nitekim, *--custom-conf-* olararak K3S sitesine baksın, belirtsin.
7. kişisel.

*/

use clap::Parser;
use colored::Colorize;
use k3sgen::parse::model as CliModel;

fn main() {
    let cli = CliModel::Cli::parse();

    println!(r"
    _     _____                          
   | | __|___ /  ___   __ _   ___  _ __  
   | |/ /  |_ \ / __| / _` | / _ \| '_ \ 
   |   <  ___) |\__ \| (_| ||  __/| | | |
   |_|\_\|____/ |___/ \__, | \___||_| |_|
                      |___/              
  ");
   
    if let Some(a) = cli.name {
        println!("{}",a.red().italic());
    }



}
    