mod conditionals;
mod loops;
use crate::utils::terminal::{ clean_terminal, show_menu, wait_enter };

pub fn execute() {
    loop {
        let itens = [
            "Conditionals",
            "For - Range",
            "For - Array",
            "While",
            "Loop",
            "* - Voltar"
        ];

        let selected = show_menu("Controls", &itens, true);

        clean_terminal();

        match selected {
            1 => conditionals::example(),
            2 => loops::for_range(),
            3 => loops::for_array(),
            4 => loops::example_while(),
            5 => loops::example_loop(),
            _ => break
        }

        wait_enter();
    }
}