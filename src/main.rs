

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

use std::fmt::Alignment;

use clap::{Parser, Command};
use colored::Colorize;
use k3sgen::parse::model::{CliApp as CliApp, Altkomutlar};

fn main() {

    let cliApp = CliApp::parse();

    
   match cliApp.command {
       Altkomutlar::Init { default:true, only_stdout} => println!("geldi"),
       _ => println!("debug")
   }






}
    