mod utils;
mod fundamentos;
mod types;
mod controls;
mod functions;
mod ownerships;

use std::process::exit;
use utils::terminal::show_menu;

fn main() {
    loop {
        let itens = ["Fundamentos","Tipos","Controle","Funções", "Ownerships"];
        let selected = show_menu("Rust", &itens, false);
      
        match selected {
            1 => fundamentos::execute(),
            2 => types::execute(),
            3 => controls::execute(),
            4 => functions::execute(),
            5 => ownerships::execute(),
            _ => exit(0),
        }
    }
}