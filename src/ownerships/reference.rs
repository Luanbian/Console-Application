pub fn imutable_ref() {
    let text = String::from(
        "Rust é uma linguagem de programação multiparadigma compilada desenvolvida pela Mozilla."
    );

    let count_of_words = calc_number_of_words(&text);
    println!("O texto {} tem {} palavras", text, count_of_words); 
}

fn calc_number_of_words(text: &String) -> usize {
    text.split_whitespace().count()
}

pub fn mutable_ref() {
    let mut name = String::from("Luan");
    add_last_name(&mut name);
    println!("{}", name);
}

fn add_last_name(name: &mut String) {
    name.push_str(" Almeida");
}