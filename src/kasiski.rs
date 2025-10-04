use std::collections::HashMap;

/// Calcule le plus grand commun diviseur (PGCD) de deux entiers.
///
/// # Arguments
/// a - Premier entier.
/// b - Deuxième entier.
///
/// # Retour
/// Le PGCD de `a` et `b`.
pub const fn compute_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Retourne la liste des diviseurs d'un entier supérieur ou égal à 2.
///
/// # Arguments
/// n - L'entier dont on veut les diviseurs.
///
/// # Retour
/// Un vecteur contenant les diviseurs de `n`.
fn divisors(n: usize) -> Vec<usize> {
    let mut out = Vec::new();

    if n >= 2 {
        let mut d = 2usize;
        while d * d <= n {
            if n % d == 0 {
                out.push(d);
                let q = n / d;
                if q != d {
                    out.push(q);
                }
            }
            d += 1;
        }
        out.push(n);
        out.sort_unstable();
        out.dedup();
    }
    out
}

/// Construit la table des répétitions de fragments dans le message.
///
/// # Arguments
/// message - Le texte à analyser.
///
/// # Retour
/// Un vecteur de tuples contenant le fragment répété et la distance entre deux occurrences.
pub fn build_repet_table(message: &str) -> Vec<(String, usize)> {
    let bytes = message.as_bytes();
    let l = bytes.len();
    let mut map: HashMap<&[u8], Vec<usize>> = HashMap::new();

    for length in 3..=l.saturating_div(2).max(3) {
        if length > l {
            break;
        }
        for start in 0..=l - length {
            let slice = &bytes[start..start + length];
            map.entry(slice).or_default().push(start);
        }
    }

    map.retain(|_, v| v.len() >= 2);

    let mut repet: Vec<(String, usize)> = Vec::new();

    for (frag, positions) in map {
        for w in positions.windows(2) {
            let d = w[1] - w[0];
            repet.push((String::from_utf8(frag.to_vec()).unwrap(), d));
        }
    }

    repet
}

/// Effectue l'analyse Kasiski sur un message chiffré pour estimer la longueur de la clé.
/// Affiche les tailles de clé possibles ou un message d'erreur si l'échantillon est trop petit.
///
/// # Arguments
/// message - Le texte chiffré à analyser.
pub fn kasiski(message: &str) {
    // Filtrer le message pour ne garder que les caractères ASCII imprimables
    let filtered: String = message
        .chars()
        .filter(|&c| (' '..='~').contains(&c))
        .collect();
    let repet = build_repet_table(&filtered);

    if repet.is_empty() {
        println!("Echantillon trop petit pour Kasiski");
        return;
    }

    let first_dist = repet[0].1;
    let mut candidats = divisors(first_dist);

    for &(_, dist) in repet.iter().skip(1) {
        let mut temp: Vec<usize> = Vec::new();
        for &cand in &candidats {
            let g = compute_gcd(cand, dist);
            if g > 1 {
                temp.push(g);
            }
        }

        temp.sort_unstable();
        temp.dedup();

        if !temp.is_empty() {
            candidats = temp;
        }
    }

    candidats.sort_unstable();
    candidats.dedup();

    if candidats.is_empty() {
        println!("?");
    } else {
        println!("Tailles de clé possibles : {candidats:?}");
    }
}
