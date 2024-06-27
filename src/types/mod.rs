mod basics;
mod sequences;
mod customs;

use crate::utils::terminal::{clean_terminal, show_menu, wait_enter};

pub fn execute() {
    loop {
        let itens = [
            "Básicos",
            "Sequências",
            "Custom - Structs",
            "Custom - Enums",
        ];

        let selected = show_menu("Tipos", &itens, false);

        clean_terminal();


        match selected {
            1 => basics::example(),
            2 => sequences::example(),
            3 => customs::structs(),
            4 => customs::enums(),
            _ => break,
        }

        wait_enter();
    }
}