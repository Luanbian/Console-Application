mod utils;
mod fundamentos;
mod types;

use std::process::exit;
use utils::terminal::show_menu;

fn main() {
    loop {
        let itens = ["Fundamentos","Tipos","Controle","Funções"];
        let selected = show_menu("Rust", &itens, true);
      
        match selected {
            1 => fundamentos::execute(),
            2 => types::execute(),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}