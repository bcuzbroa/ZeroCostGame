# 🎮 Guide du Joueur – ZeroCostGame

Bienvenue dans **ZeroCostGame** !  
Ce document explique **ce que vous devez installer**, **comment préparer l’oracle** et **comment vérifier tes challenges**.

---

## 🛠️ Prérequis

### Installer Rust (rustc)

Le jeu nécessite le compilateur Rust.

Installation recommandée via **rustup** :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Vérifier ensuite que Rust est bien installé :


```bash
rustc --version
cargo --version
```

--- 

## 🔐 Préparer l’oracle

L’oracle est le programme qui valide tes solutions et révèle les flags.

Rendre l’oracle exécutable

Depuis la racine du projet :

```bash
chmod +x oracle
```

Pour vérifier qu'il est bien exécutable : 

```bash
ls -l oracle
```

---

## 🧠 Résoudre un Challenge

Chaque challenge correspond à une ou plusieurs fonctions à implémenter. Pour cela il faut:

1. Aller dans le dossier des challenges :
2. Ouvrir le fichier correspondant au challenge (exemple : `challenge0.rs`)
3. Implémenter la fonction demandée en respectant strictement l’énoncé.


---

## 🧪 Tester le code localement

Vous pouvez faire vos propres tests directement dans la fonction main du fichier de challenge: 

Exemple de main pour le `challenge0.rs`:
``` rust
fn main() {
    println!("{}", hello());
}

``` 

⚠️ Le contenu du main n’a aucun impact sur la validation par l’oracle. Il sert uniquement à t’aider à tester votre code et est "ignoré" lorsque l'oracle vérifie le challenge.

---

## Vérifier un challenge avec l'oracle

Une fois le code prêt, vous pouvez utiliser l'oracle pour vérifier votre solution:


```bash
./oracle -- -p /path/to/challenge
```

Par exemple :
```bash
 ./oracle -- -p /chemin/vers/challenges/src/challenge0.rs
```

Le résultat attendu est le suivant:
```bash
✨ Sanity Check ✨ verified: FLAG{XXXX-XXXX-XXXX-XXX}
```

---

🚀 Bon courage ! Lisez attentivement les consignes, testez votre code dans main. Les challenges sont de plus en plus difficiles et valent de plus en plus de points.