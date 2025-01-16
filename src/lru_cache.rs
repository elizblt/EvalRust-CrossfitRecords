use std::collections::HashMap;
use std::fs::{File};
use std::io::{self, BufRead, Write};

// Structure représentant un cache LRU (Least Recently Used).
// Permet de stocker des paires clé-valeur avec une capacité limitée.
// Les éléments les moins récemment utilisés sont supprimés en premier.
#[derive(Debug)]
pub struct LruCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub capacity: usize,        // Taille maximale du cache
    pub map: HashMap<K, V>,     // Stocke les paires clé-valeur
    pub order: Vec<K>,          // Liste des clés dans l'ordre d'utilisation
}

impl<K: Clone + Eq + std::hash::Hash + std::fmt::Display, V: Clone + std::fmt::Display> LruCache<K, V> {
    // Crée un nouveau cache LRU avec une capacité donnée
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "La capacité doit être supérieure à 0.");
        Self {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    // Ajoute ou met à jour une paire clé-valeur dans le cache
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            if let Some(oldest) = self.order.first() {
                self.map.remove(oldest);
            }
            self.order.remove(0);
        }
        self.map.insert(key.clone(), value);
        self.order.push(key);
    }

    // Récupère la valeur associée à une clé dans le cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.map.get(key) {
            self.order.retain(|k| k != key);
            self.order.push(key.clone());
            Some(value)
        } else {
            None
        }
    }

    // Sauvegarde le contenu du cache dans un fichier texte
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        for key in &self.order {
            if let Some(value) = self.map.get(key) {
                writeln!(file, "{}:{}", key, value)?;
            }
        }
        Ok(())
    }

    // Recharge un cache depuis un fichier texte
    pub fn load_from_file(path: &str, capacity: usize) -> io::Result<Self>
    where
        K: std::str::FromStr,
        V: std::str::FromStr,
        K::Err: std::fmt::Debug,
        V::Err: std::fmt::Debug,
    {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut cache = LruCache::new(capacity);

        for line in reader.lines() {
            let line = line?;
            if let Some((key, value)) = line.split_once(':') {
                let key: K = key.parse().unwrap();
                let value: V = value.parse().unwrap();
                cache.put(key, value);
            }
        }

        Ok(cache)
    }
}

