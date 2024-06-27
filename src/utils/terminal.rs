use rpassword::prompt_password;
use std::io::Write;

pub fn show_menu(title: &str, itens: &[&str], exit: bool) -> u32 {
    clean_terminal();

    let description = String::from("Studies :: ") + title;
    println!("{}", description);
    println!("{}", String::from("=").repeat(description.len()));

    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }

    println!("{}", if exit { "* - Sair" } else { "" });
    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response).unwrap();

    let selected_option: Result<u32, _> = user_response.trim().parse();

    match selected_option {
        Ok(option) => option, 
        _ => 0,
    }
}


pub fn wait_enter() {  
    prompt_password("Pressione Enter para continuar...").unwrap();
}

pub fn clean_terminal() {
    print!("{}c", 27 as char);
}