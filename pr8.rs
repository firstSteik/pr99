use std::io::{self, Write};

fn toggle_case(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_uppercase() { c.to_lowercase() } else { c.to_uppercase() })
        .flatten()
        .collect()
}

fn main() {
    let texts = [
        "Привіт, Світ!",
        "Програмування на Rust",
        "Зміна РЕГІСТРУ",
    ];

    println!("Оберіть текст для зміни регістру:");
    for (індекс, текст) in texts.iter().enumerate() {
        println!("{}: {}", індекс + 1, текст);
    }

    print!("Введіть номер тексту, який бажаєте змінити: ");
    io::stdout().flush().unwrap();

    let mut вибір = String::new();
    io::stdin().read_line(&mut вибір).expect("Не вдалося зчитати рядок");

    if let Ok(індекс) = вибір.trim().parse::<usize>() {
        if let Some(обраний_текст) = texts.get(індекс - 1) {
            let змінений_текст = toggle_case(обраний_текст);
            println!("Оригінал: {}", обраний_текст);
            println!("Змінений: {}", змінений_текст);
        } else {
            println!("Невірний вибір.");
        }
    } else {
        println!("Помилка вводу. Будь ласка, введіть число.");
    }
}