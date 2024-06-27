mod utils;
mod fundamentos;

use std::process::exit;
use utils::terminal::show_menu;

fn main() {
    loop {
        let itens = ["Fundamentos","Tipos","Controle","Funções"];
        let selected = show_menu("Rust", &itens, true);
      
        match selected {
            1 => fundamentos::execute(),
            2 => println!("2"),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}