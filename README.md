# Vigenère – Chiffrement, Démo, Analyse Kasiski & Mode d’emploi

## Introduction

Ce projet propose une implémentation moderne et robuste du chiffre de Vigenère en Rust, avec prise en charge complète de l’ASCII imprimable, gestion des caractères non imprimables, et une analyse Kasiski pour l’estimation de la taille de la clé. Il permet de chiffrer et déchiffrer des messages, en mode manuel ou via fichiers, tout en respectant la casse, la ponctuation et les caractères spéciaux.

---

## Sommaire
- [Présentation](#présentation)
- [Historique et principe du chiffre de Vigenère](#historique-et-principe-du-chiffre-de-vigenère)
- [Sécurité et limites](#sécurité-et-limites)
- [Installation](#installation)
- [Organisation du code](#organisation-du-code)
- [Modes d’utilisation](#modes-dutilisation)
- [Jeux d’essais](#jeux-dessais)
- [Tests](#tests)

---

## Présentation

- **Langage** : Rust
- **Fonctionnalités** :
  - Chiffrement et déchiffrement Vigenère sur tout l’ASCII
  - Gestion des caractères non imprimables (ne sont pas chiffrés, n’avancent pas la clé)
  - Analyse Kasiski pour retrouver la taille de la clé
  - Mode manuel et mode fichiers
  - Respect de la casse, ponctuation, chiffres, espaces
  - Tests unitaires et d’intégration

---

## Historique et principe du chiffre de Vigenère

Le chiffre de Vigenère est un algorithme de chiffrement polyalphabétique inventé au XVIe siècle. Il consiste à utiliser une clé pour décaler chaque caractère du message selon une table de correspondance. Sa simplicité et sa robustesse apparente en ont fait un standard pendant plusieurs siècles, jusqu’à ce que des méthodes d’analyse comme celle de Kasiski permettent de le casser facilement si la clé est courte ou le texte répétitif.

- **Principe** :
  - Chaque caractère du texte est chiffré en le décalant selon le caractère correspondant de la clé (qui boucle si elle est plus courte).
  - Seuls les caractères ASCII imprimables (codes 32 à 126) sont chiffrés.
  - Les caractères non imprimables (sauts de ligne, tabulations, etc.) sont laissés inchangés et n’avancent pas la clé.

---

## Sécurité et limites

- **Sécurité** :
  - Le chiffre de Vigenère est vulnérable à l’analyse de fréquences et à l’attaque de Kasiski si la clé est courte ou le texte répétitif.
  - Pour une sécurité optimale, utilisez une clé aussi longue et aléatoire que le texte (chiffre de Vernam).
- **Limites** :
  - Ne protège pas contre les attaques modernes (brute force, cryptanalyse avancée).
  - Ne convient pas pour des usages sensibles ou la protection de données confidentielles.

---

## Installation

1. **Prérequis** :
   - [Rust](https://www.rust-lang.org/fr/tools/install) installé sur votre machine.

2. **Compilation** :
   ```bash
   cargo build --release
   ```
   Le binaire sera généré dans `target/release/vigenere`.

---

## Organisation du code

- **src/main.rs** : Point d’entrée, gestion du menu, lecture des fichiers ou saisie manuelle, affichage des résultats.
- **src/vigenere.rs** : Logique du chiffrement/déchiffrement, gestion de la clé, prise en charge ASCII.
- **src/kasiski.rs** : Analyse Kasiski, calcul des diviseurs, PGCD, détection des motifs répétés.
- **src/tests.rs** : Tests unitaires et d’intégration (chiffrement, déchiffrement, analyse Kasiski, gestion des clés).
- **data/text.txt** : Fichier d’entrée pour le texte à chiffrer (mode fichiers).
- **data/key.txt** : Fichier d’entrée pour la clé (mode fichiers).

---

## Modes d’utilisation

### Mode fichiers

- Placez le texte à chiffrer dans `data/text.txt`.
- Placez la clé dans `data/key.txt`.
- Lancez le programme :
  ```bash
  cargo run --release
  ```
- Choisissez l’option **1** dans le menu.

### Mode manuel

- Lancez le programme :
  ```bash
  cargo run --release
  ```
- Choisissez l’option **2** dans le menu.
- Saisissez le message et la clé directement dans le terminal.

---

## Jeux d’essais

### Jeu d’essais 1 — Basique (lettres majuscules)

- **Plaintext** : `HELLOWORLD`
- **Clé** : `KEY`
- **Chiffré attendu** : `tk'xu2{x'p`
- **Déchiffré attendu** : `HELLOWORLD`
- **Kasiski (attendu)** :
  ```
  Echantillon trop petit pour Kasiski
  ```

> **But** : Vérifier le cycle de clé et le mappage ASCII imprimable sur un cas simple.

### Jeu d’essais 2 — Casse, ponctuation, chiffres

- **Plaintext** : `Hello, World! 123`
- **Clé** : `Vig3nere`
- **Chiffré attendu** : `PU!_rs>G]Uxpf%xj`
- **Déchiffré attendu** : `Hello, World! 123`
- **Kasiski (attendu)** :
  ```
  Echantillon trop petit pour Kasiski
  ```

> **But** : Montrer que tout l’ASCII imprimable est chiffré (espaces, virgule, point d’exclamation, chiffres), et que la casse est préservée par la logique. Les non-imprimables ne sont pas concernés ici.

### Jeu d’essais 3 — Caractères non imprimables (n’avancent pas la clé)

- **Plaintext** :
  ```
  é
  ```
- **Clé** : `K`
- **Chiffré attendu** : é
- **Déchiffré attendu** : é
- **Comportement attendu** : Le caractère é qui n'est pas dans la table ascii n'est pas chiffré.
- **Kasiski (attendu)** :
  ```
  Echantillon trop petit pour Kasiski
  ```

> **But** : Vérifier que la logique saute les caractères non imprimables et que la synchronisation de la clé est correcte.

### Jeu d’essais 4 — Texte long et motifs répétés (clé retrouvée)

- **Plaintext** : `fichier data/text.txt`
- **Clé** : `fichier data/key.txt`
- **Kasiski (attendu)** :
  ```
  Tailles de clé possibles : [3]
  ```

> **But** : Tester l’analyse Kasiski sur un texte propice à la détection de la taille de clé.


### Jeu d’essais 5 — Texte aléatoire ou trop court (Kasiski échoue)

- **Plaintext** : `QWERTYUIOPASDFGHJKLZXCVBNM`
- **Clé** : `KEY`
- **Chiffré attendu** : }} ~z4"o*|g.pl"tp&x!3o||zs
- **Déchiffré attendu** : QWERTYUIOPASDFGHJKLZXCVBNM
- **Kasiski (attendu)** :
  ```
  Echantillon trop petit pour Kasiski

  ```

> **But** : Vérifier que Kasiski ne propose rien si le texte est trop court ou sans répétitions.

### Jeu d’essais 6 — Clé plus longue que le texte

- **Plaintext** : `a`
- **Clé** : `aa`
- **Chiffré attendu** : D
- **Déchiffré attendu** : a
- **Kasiski (attendu)** :
  ```
  Echantillon trop petit pour Kasiski
  ```

> **But** : Tester le comportement quand la clé dépasse la longueur du texte.


### Exécution des tests

```bash
cargo test
```

---

## Tests

Des tests sont fournis pour garantir la robustesse du chiffrement, du déchiffrement, de la gestion des clés et de l’analyse Kasiski. Ils couvrent :
- Chiffrement/déchiffrement sur différents types de textes
- Gestion des caractères non imprimables
- Analyse Kasiski sur textes répétitifs
- Calcul du PGCD et des diviseurs

