use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RustyWords", version, about = "CLI-словарь")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Добавить слово и перевод
    Add {
        /// Оригинальное слово
        word: String,
        /// Перевод слова
        translation: String,
    },
    /// Найти перевод слова
    Get {
        /// Слово для поиска
        word: String,
    },
    /// Показать все слова
    List,
    /// Поиск слов по префиксу
    Search {
        /// Префикс для поиска (например, 't' найдет все слова, начинающиеся на 't')
        prefix: String,
    },

}

fn main() {
    let cli = Cli::parse();

    let db: sled::Db = sled::open("my_db").unwrap();

    match &cli.command {
        Commands::Add { word, translation } => {
            db.insert(word.as_bytes(), translation.as_bytes()).unwrap();
            
            println!("Добавлено: {} => {}", word, translation);
        }
        Commands::Get { word } => {
            match db.get(word.as_bytes()) {
                Ok(Some(translation)) => {
                    match String::from_utf8(translation.to_vec()) {
                        Ok(translation_str) => println!("{} => {}", word, translation_str),
                        Err(e) => println!("Ошибка при чтении перевода: {}", e),
                    }
                },
                Ok(None) => println!("Слово '{}' не найдено", word),
                Err(e) => println!("Ошибка при обращении к БД: {}", e),
            }

        }
        Commands::List => {
            println!("Словарь:");
            for result in db.iter() {
                match result {
                    Ok((key, value)) => {
                        let word = String::from_utf8_lossy(&key);
                        let translation = String::from_utf8_lossy(&value);
                        println!("{} => {}", word, translation);
                    }
                    Err(e) => println!("Ошибка при чтении записи: {}", e),
                }
            }
        }
        Commands::Search { prefix } => {
            println!("Поиск слов на '{}':", prefix);
            let prefix_lower = prefix.to_lowercase();

            let mut count = 0;
            for result in db.scan_prefix(prefix_lower.as_bytes()) {
                match result {
                    Ok((key, value)) => {
                        let word = String::from_utf8_lossy(&key);
                        let translation = String::from_utf8_lossy(&value);
                        println!("{} => {}", word, translation);
                        count += 1;
                    }
                    Err(e) => println!("Ошибка при чтении записи: {}", e),
                }
            }

            if count == 0 {
                println!("Слов, начинающихся на '{}', не найдено", prefix);
            } else {
                println!("\nНайдено слов: {}", count);
            }
        }
    }
}
