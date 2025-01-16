mod lru_cache;
use lru_cache::LruCache;
use std::io::{self, Write};

/// Ajoute des couleurs aux messages via les codes ANSI
fn colorize(text: &str, color: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, text)
}

/// Programme principal pour gérer les records CrossFit avec un cache LRU.
fn main() {
    let mut cache = LruCache::new(3); // Cache avec une capacité de 3 records
    let file_path = "records.txt"; // Chemin du fichier de sauvegarde

    loop {
        // Affichage du menu
        println!("\n{}", colorize("--- Menu CrossFit Records ---", "1;34"));
        println!("1. {}", colorize("Ajouter ou mettre à jour un record", "1;32"));
        println!("2. {}", colorize("Voir un record", "1;33"));
        println!("3. {}", colorize("Afficher tous les records", "1;36"));
        println!("4. {}", colorize("Sauvegarder les records", "1;35"));
        println!("5. {}", colorize("Recharger les records", "1;31"));
        println!("6. {}", colorize("Quitter", "1;37"));

        print!("{}", colorize("Choisissez une option : ", "1;34"));
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // Ajouter ou mettre à jour un record
                print!("{}", colorize("Nom du mouvement : ", "1;32"));
                io::stdout().flush().unwrap();
                let mut movement = String::new();
                io::stdin().read_line(&mut movement).unwrap();

                loop {
                    print!("{}", colorize("Nouveau record (kg) : ", "1;33"));
                    io::stdout().flush().unwrap();
                    let mut record = String::new();
                    io::stdin().read_line(&mut record).unwrap();

                    match record.trim().parse::<f32>() {
                        Ok(value) => {
                            cache.put(movement.trim().to_string(), value);
                            println!(
                                "{} {} avec {} kg !",
                                colorize("✅ Record pour", "1;32"),
                                colorize(movement.trim(), "1;34"),
                                colorize(&value.to_string(), "1;33")
                            );
                            break;
                        }
                        Err(_) => {
                            println!("{}", colorize("❌ Entrée invalide. Veuillez entrer un nombre.", "1;31"));
                        }
                    }
                }
            }
            "2" => {
                // Voir un record spécifique
                print!("{}", colorize("Nom du mouvement : ", "1;33"));
                io::stdout().flush().unwrap();
                let mut movement = String::new();
                io::stdin().read_line(&mut movement).unwrap();

                if let Some(&record) = cache.get(&movement.trim().to_string()) {
                    println!(
                        "{} pour {} : {} kg",
                        colorize("💪 Record", "1;32"),
                        colorize(movement.trim(), "1;34"),
                        colorize(&record.to_string(), "1;33")
                    );
                } else {
                    println!("{} {}", colorize("❌ Aucun record trouvé pour", "1;31"), movement.trim());
                }
            }
            "3" => {
                // Afficher tous les records
                if cache.order.is_empty() {
                    println!("{}", colorize("⚠️ Aucun record dans le cache.", "1;31"));
                } else {
                    println!("{}", colorize("\n+-------------------+------------+", "1;34"));
                    println!("{}", colorize("| Mouvement         | Poids (kg) |", "1;34"));
                    println!("{}", colorize("+-------------------+------------+", "1;34"));
                    for key in &cache.order {
                        if let Some(value) = cache.map.get(key) {
                            println!(
                                "| {:<17} | {:>10.2} |",
                                key,
                                value
                            );
                        }
                    }
                    println!("{}", colorize("+-------------------+------------+\n", "1;34"));
                }
            }
            "4" => {
                // Sauvegarder les records
                if let Err(e) = cache.save_to_file(file_path) {
                    eprintln!(
                        "{} {}",
                        colorize("❌ Erreur lors de la sauvegarde :", "1;31"),
                        e
                    );
                } else {
                    println!(
                        "{} {}",
                        colorize("✅ Records sauvegardés dans", "1;32"),
                        colorize(file_path, "1;34")
                    );
                }
            }
            "5" => {
                // Recharger les records
                match LruCache::<String, f32>::load_from_file(file_path, 3) {
                    Ok(loaded_cache) => {
                        cache = loaded_cache;
                        println!(
                            "{} {}",
                            colorize("✅ Records rechargés depuis", "1;32"),
                            colorize(file_path, "1;34")
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "{} {} : {}",
                            colorize("❌ Erreur lors du rechargement depuis", "1;31"),
                            colorize(file_path, "1;34"),
                            e
                        );
                    }
                }
            }
            "6" => {
                // Quitter avec sauvegarde automatique
                if let Err(e) = cache.save_to_file(file_path) {
                    eprintln!(
                        "{} {}",
                        colorize("❌ Erreur lors de la sauvegarde avant de quitter :", "1;31"),
                        e
                    );
                } else {
                    println!(
                        "{}",
                        colorize("✅ Records sauvegardés automatiquement. Au revoir !", "1;32")
                    );
                }
                break;
            }
            _ => println!("{}", colorize("❌ Choix invalide. Veuillez réessayer.", "1;31")),
        }
    }
}
