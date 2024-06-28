mod mechanism;
mod escopo;
mod reference;
mod slice;

use crate::utils::terminal::{ clean_terminal, show_menu, wait_enter };

pub fn execute() {
    loop {
        let itens = [
            "Básico",
            "Tempo de vida",
            "Mover",
            "Clone",
            "Cópia",
            "Movendo a posse #1",
            "Movendo a posse #2",
            "Referência Imutável",
            "Referência Mutável",
            "Slice",
        ];
        let selected = show_menu("Ownership", &itens, false);

        clean_terminal();

        match selected {
            1 => escopo::basic(),
            2 => escopo::life_cicle(),
            3 => escopo::transfer(),
            4 => escopo::clone(),
            5 => escopo::copy(),
            6 => mechanism::one(),
            7 => mechanism::two(),
            8 => reference::imutable_ref(),
            9 => reference::mutable_ref(),
            10 => slice::example(),
            _ => break
        }

        wait_enter();
    }
}