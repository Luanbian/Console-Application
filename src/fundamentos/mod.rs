mod first_example;
mod variables;
mod operators;

use crate::utils::terminal::{clean_terminal, show_menu, wait_enter};


pub fn execute () {
    loop {
        let itens = [
            "Primeiro exemplo",
            "Variáveis - Imutáveis", 
            "Variáveis - Mutáveis",
            "Variáveis - Constantes",
            "Variáveis - shadowing",
            "Operadores - Aritméticos",
            "Operadores - Relacionais",
            "Operadores - Lógicos",
        ];

        let selected = show_menu("Fundamentos", &itens, false);

        clean_terminal();

        match selected {
            1 => first_example::example(),
            2 => variables::imutables(),
            3 => variables::mutables(),
            4 => variables::constants(),
            5 => variables::shadowing(),
            6 => operators::aritmetics(),
            7 => operators::relations(),
            8 => operators::logics(),
            _ => break,
        }

        wait_enter();
    }
}