mod functions;
mod lambda;

use crate::utils::terminal::{ clean_terminal, show_menu, wait_enter };

pub fn execute() {
    loop {
        let itens = [
            "Básicos",
            "Map",
            "Filter",
            "Voltar"
        ];

        let selected = show_menu("Functions", &itens, false);

        clean_terminal();
        
        match selected {
            1 => functions::example(),
            2 => lambda::map(),
            3 => lambda::filter(),
            _ => break
        }

        wait_enter();
    }
}