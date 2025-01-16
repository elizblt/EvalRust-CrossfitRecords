# Crossfit Records 🏋️‍♂️

Un projet Rust pour gérer un cache **LRU (Least Recently Used)** afin d'enregistrer et de suivre vos records de mouvements CrossFit. 

---

## 🗋 Fonctionnalités

- **Ajout de records** : Enregistrez vos performances en poids pour des mouvements comme le Snatch, Clean and Jerk, Deadlift, etc.
- **Cache LRU** : Gère automatiquement un cache avec une capacité limitée en éjectant les entrées les moins utilisées.
- **Sauvegarde des records** : Stockez vos records dans un fichier pour les réutiliser plus tard.
- **Rechargement des records** : Récupérez vos records sauvegardés pour continuer à les suivre et les améliorer.
- **Tests unitaires** : Vérifiez la fiabilité du cache grâce aux tests intégrés.

---

## 🚀 Comment démarrer

### Prérequis
- [Rust](https://www.rust-lang.org/) installé (version stable recommandée).
- Un terminal (comme PowerShell, Bash ou VS Code).

### Installation et exécution

1. **Clonez le dépôt Git** :
   ```bash
   git clone https://github.com/elizblt/EvalRust-CrossfitRecords.git
   cd EvalRust-CrossfitRecords
   ```

2. **Construisez le projet** :
   ```bash
   cargo build
   ```

3. **Exécutez le projet** :
   ```bash
   cargo run
   ```

4. **Lancez les tests** (facultatif mais recommandé) :
   ```bash
   cargo test
   ```

---

## 🔂 Structure du projet

- **`src/main.rs`** : Le fichier principal contenant la logique utilisateur (ajout, sauvegarde, rechargement des records).
- **`src/lru_cache.rs`** : Implémentation du cache LRU utilisé pour stocker les records.
- **`tests/lru_cache_tests.rs`** : Tests unitaires pour vérifier la fiabilité et la robustesse de l'implémentation.

---

## 🚨 Exemple d'utilisation

1. **Ajoutez vos records** :
   ```bash
   Snatch : 65 kg
   Clean and Jerk : 75 kg
   Deadlift : 140 kg
   ```

2. **Sauvegardez-les dans un fichier** :
   - Les records seront enregistrés dans `records.txt`.

3. **Rechargez vos records** :
   - Récupérez vos données à tout moment pour continuer à suivre vos progrès.

---

## 📜 Licence

Ce projet est sous licence MIT. Vous êtes libre de l'utiliser et de le modifier à des fins personnelles ou professionnelles.

---

