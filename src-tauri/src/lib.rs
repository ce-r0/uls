use serde::Serialize;

#[derive(Serialize)]
pub struct Analysis {
    pub gramatical: String,
    pub gramatical_count: usize,
    pub adjustment: i32,
    pub poetica: String,
    pub count: usize,
    pub kind: String,
    pub ritmo: String,
    pub terminacion: String,
}

fn is_a(c: char) -> bool { "aeoáéóíú".contains(c) } 
fn is_i(c: char) -> bool { "iuü".contains(c) }      
fn is_l(c: char) -> bool { "rl".contains(c) }       
fn is_o(c: char) -> bool { "pbftdckg".contains(c) } 
fn is_c(c: char) -> bool { "bcdfghjklmnñpqrstvwxyz".contains(c) }

fn get_word_syllables(word: &str) -> Vec<String> {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    if chars.is_empty() { return vec![]; }

    let mut letras = Vec::new();
    let mut estructura = String::new();
    let mut j = 0;

    while j < chars.len() {
        if j == 0 && j + 1 < chars.len() {
            let next_c = chars[j+1];
            if chars[j] == 'p' && "snt".contains(next_c) {
                letras.push(format!("p{}", next_c)); estructura.push('C'); j += 2; continue;
            } else if chars[j] == 'g' && next_c == 'n' {
                letras.push("gn".to_string()); estructura.push('C'); j += 2; continue;
            }
        }
        if j < chars.len() - 1 {
            let next_c = chars[j+1];
            if (chars[j] == 'c' && next_c == 'h') || (chars[j] == 'l' && next_c == 'l') || (chars[j] == 'r' && next_c == 'r') {
                letras.push(format!("{}{}", chars[j], next_c)); estructura.push('C'); j += 2; continue;
            }
        }

        let c = chars[j];
        if is_a(c) { letras.push(c.to_string()); estructura.push('A'); }
        else if is_i(c) { letras.push(c.to_string()); estructura.push('I'); }
        else if is_l(c) { letras.push(c.to_string()); estructura.push('L'); }
        else if is_o(c) { letras.push(c.to_string()); estructura.push('O'); }
        else if is_c(c) { letras.push(c.to_string()); estructura.push('C'); }
        j += 1;
    }

    estructura.push('C');
    letras.push("".to_string());

    let est_chars: Vec<char> = estructura.chars().collect();
    let mut salida = Vec::new();
    let mut silaba = String::new();
    j = 0;

    while j < letras.len() {
        if letras[j].is_empty() { break; }
        silaba.push_str(&letras[j]);

        let e_j = est_chars[j];
        let e_j1 = *est_chars.get(j+1).unwrap_or(&' ');

        if e_j == 'A' {
            if e_j1 == 'A' {
                salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
            } else if e_j1 == 'I' { j += 1; continue;
            } else if e_j1 == 'O' {
                let e_j2 = *est_chars.get(j+2).unwrap_or(&' ');
                if "AIL".contains(e_j2) {
                    if letras[j+1] == "d" && letras.get(j+2).map(|s| s.as_str()) == Some("l") {
                        silaba.push_str(&letras[j+1]); salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                    }
                    salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
                } else {
                    let e_j3 = *est_chars.get(j+3).unwrap_or(&' ');
                    if letras.get(j+2).map(|s| s.as_str()) == Some("s") && "LCO".contains(e_j3) {
                        silaba.push_str(&letras[j+1]); silaba.push_str(&letras[j+2]);
                        salida.push(silaba.clone()); silaba.clear(); j += 3; continue;
                    }
                    silaba.push_str(&letras[j+1]); salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                }
            } else { 
                let e_j2 = *est_chars.get(j+2).unwrap_or(&' ');
                if "AI".contains(e_j2) { salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
                } else {
                    let e_j3 = *est_chars.get(j+3).unwrap_or(&' ');
                    if letras.get(j+2).map(|s| s.as_str()) == Some("s") && "LCO".contains(e_j3) {
                        silaba.push_str(&letras[j+1]); silaba.push_str(&letras[j+2]);
                        salida.push(silaba.clone()); silaba.clear(); j += 3; continue;
                    }
                    silaba.push_str(letras.get(j+1).unwrap_or(&"".to_string()));
                    salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                }
            }
        } else if e_j == 'I' {
             if "AI".contains(e_j1) { j += 1; continue;
            } else if e_j1 == 'O' {
                 let e_j2 = *est_chars.get(j+2).unwrap_or(&' ');
                if "AIL".contains(e_j2) {
                     if letras[j+1] == "d" && letras.get(j+2).map(|s| s.as_str()) == Some("l") {
                        silaba.push_str(&letras[j+1]); salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                    }
                    salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
                } else {
                     let e_j3 = *est_chars.get(j+3).unwrap_or(&' ');
                    if letras.get(j+2).map(|s| s.as_str()) == Some("s") && "LCO".contains(e_j3) {
                        silaba.push_str(&letras[j+1]); silaba.push_str(&letras[j+2]);
                        salida.push(silaba.clone()); silaba.clear(); j += 3; continue;
                    }
                    silaba.push_str(&letras[j+1]); salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                }
            } else { 
                let e_j2 = *est_chars.get(j+2).unwrap_or(&' ');
                if "AI".contains(e_j2) { salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
                } else {
                     let e_j3 = *est_chars.get(j+3).unwrap_or(&' ');
                    if letras.get(j+2).map(|s| s.as_str()) == Some("s") && "LCO".contains(e_j3) {
                        silaba.push_str(&letras[j+1]); silaba.push_str(&letras[j+2]);
                        salida.push(silaba.clone()); silaba.clear(); j += 3; continue;
                    }
                    silaba.push_str(letras.get(j+1).unwrap_or(&"".to_string()));
                    salida.push(silaba.clone()); silaba.clear(); j += 2; continue;
                }
            }
        } else if e_j == 'O' {
            if "AIL".contains(e_j1) { j += 1; continue;
            } else {
                if letras.get(j+1).map(|s| s.as_str()) == Some("") { salida.push(silaba.clone()); break; }
                salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
            }
        } else { 
             if "AI".contains(e_j1) { j += 1; continue;
            } else {
                 if letras.get(j+1).map(|s| s.as_str()) == Some("") { salida.push(silaba.clone()); break; }
                 else if letras.get(j+1).map(|s| s.as_str()) == Some("s") {
                     salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
                 }
                salida.push(silaba.clone()); silaba.clear(); j += 1; continue;
            }
        }
    }
    salida
}

// NUEVO: Motor de Acentuación Dual Core (Devuelve un Vec<usize>)
fn get_tonic_indices(word: &str, syls: &[String]) -> Vec<usize> {
    let w = word.to_lowercase();
    let unstressed_words = [
        "el", "la", "los", "las", "un", "una", "unos", "unas", "al", "del", // Artículos y contracciones
        "a", "ante", "bajo", "con", "contra", "de", "desde", "en", "entre", "hacia", "hasta", "para", "por", "según", "sin", "sobre", "tras", // Preposiciones
        "me", "te", "se", "nos", "os", "le", "les", "lo", "la", // Pronombres
        "mi", "tu", "su", "mis", "tus", "sus", // Posesivos
        "que", "si", "y", "e", "o", "u", "ni",
        "es", "son", "ha", "he", "han" // Copulativos y auxiliares (Parche poético)
    ];
    
    if syls.len() == 1 {
        return if unstressed_words.contains(&w.as_str()) { vec![] } else { vec![0] };
    }
    
    if w.ends_with("mente") && w.len() > 5 {
        let falsos = ["mente", "amente", "demente", "clemente", "inclemente", "vehemente", "simiente", "lamente", "comente", "alimente", "ornamente", "pavimente", "aumente", "fomente", "experimente", "fermente", "fragmente", "implemente", "documente", "parlamente", "pigmente"];
        if !falsos.contains(&w.as_str()) {
            let mut indices = Vec::new();
            let mut has_tilde = false;
            for (i, syl) in syls.iter().enumerate().take(syls.len().saturating_sub(2)) {
                if syl.chars().any(|c| "áéíóúÁÉÍÓÚ".contains(c)) { indices.push(i); has_tilde = true; break; }
            }
            if !has_tilde && syls.len() >= 3 {
                let base = w.strip_suffix("mente").unwrap_or("");
                let last = base.chars().last().unwrap_or(' ');
                indices.push(if "nsaeiou".contains(last) { syls.len().saturating_sub(4) } else { syls.len().saturating_sub(3) });
            }
            indices.push(syls.len().saturating_sub(2));
            return indices;
        }
    }
    
    for (i, syl) in syls.iter().enumerate() {
        if syl.chars().any(|c| "áéíóúÁÉÍÓÚ".contains(c)) { return vec![i]; }
    }
    
    let last = w.chars().last().unwrap_or(' ');
    if "nsaeiou".contains(last) {
        if syls.len() >= 2 { vec![syls.len() - 2] } else { vec![0] }
    } else {
        if !syls.is_empty() { vec![syls.len() - 1] } else { vec![] }
    }
}

// --- NUEVO SENSOR: EXTRACTOR DE BLOQUE DE RIMA ---
fn extract_rhyme_block(last_word: &str, syls: &[String]) -> String {
    let w = last_word.to_lowercase();
    if w.is_empty() || syls.is_empty() { return String::new(); }

    // 1. Buscamos qué sílaba tiene el acento (usando nuestro motor existente)
    let mut tonic_idx = 0; // Asumimos aguda por defecto si todo falla
    
    // Si la palabra tiene tilde, la encontramos rápido
    for (i, syl) in syls.iter().enumerate() {
        if syl.chars().any(|c| "áéíóúÁÉÍÓÚ".contains(c)) {
            tonic_idx = i;
            break;
        }
    }
    
    // Si no hay tilde, aplicamos la regla ortográfica general
    if tonic_idx == 0 && syls.len() > 1 && !syls.iter().any(|s| s.chars().any(|c| "áéíóúÁÉÍÓÚ".contains(c))) {
        let last_char = w.chars().last().unwrap_or(' ');
        if "nsaeiou".contains(last_char) {
            tonic_idx = syls.len() - 2; // Grave
        } else {
            tonic_idx = syls.len() - 1; // Aguda
        }
    }

    // 2. Extraemos desde la sílaba tónica hasta el final
    let mut rhyme_block = String::new();
    for i in tonic_idx..syls.len() {
        rhyme_block.push_str(&syls[i]);
    }

// 3. RECALIBRACIÓN POÉTICA: Extracción ignorando semivocales (El parche de los Diptongos)
    let mut start_idx = 0;
    let chars_vec: Vec<char> = rhyme_block.chars().collect();
    
    for (i, &c) in chars_vec.iter().enumerate() {
        let is_vowel = "aeiouáéíóúü".contains(c);
        if is_vowel {
            // Si es vocal débil (i, u) y la siguiente es fuerte (a, e, o), LA SALTAMOS.
            if "iuü".contains(c) && i + 1 < chars_vec.len() && "aeoáéó".contains(chars_vec[i+1]) {
                continue;
            }
            start_idx = i;
            break;
        }
    }

    let raw_rhyme: String = chars_vec[start_idx..].iter().collect();
    
    // Normalizamos quitando tildes para que el emparejamiento sea matemáticamente perfecto
    let clean_rhyme: String = raw_rhyme.chars().map(|c| match c {
        'á' => 'a', 'é' => 'e', 'í' => 'i', 'ó' => 'o', 'ú' | 'ü' => 'u',
        _ => c
    }).collect();

    clean_rhyme
}

#[tauri::command]
fn analyze_line(line: &str) -> Analysis {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut flat_syls = Vec::new();
    let mut is_word_start = Vec::new();
    let mut flat_rhythm = Vec::new(); // NUEVO: Matriz de fuerza rítmica

    // 1. Reconstrucción Sincronizada
    for w in &words {
        let clean: String = w.chars().filter(|c| c.is_alphabetic() || "áéíóúüÁÉÍÓÚÜ".contains(*c)).collect();
        if clean.is_empty() { continue; } // Protección de símbolos sueltos

        let s = get_word_syllables(&clean);
        let tonics = get_tonic_indices(&clean, &s); // Analizamos acentos
        
        let mut char_idx = 0;
        let original_chars: Vec<char> = w.chars().collect();

        for (i, syl) in s.iter().enumerate() {
            let mut syl_orig = String::new();
            for _ in 0..syl.chars().count() { 
                while char_idx < original_chars.len() && !original_chars[char_idx].is_alphabetic() && !"áéíóúüÁÉÍÓÚÜ".contains(original_chars[char_idx]) {
                    syl_orig.push(original_chars[char_idx]); 
                    char_idx += 1;
                }
                if char_idx < original_chars.len() {
                    syl_orig.push(original_chars[char_idx]);
                    char_idx += 1;
                }
            }
            if i == s.len() - 1 {
                while char_idx < original_chars.len() {
                    syl_orig.push(original_chars[char_idx]);
                    char_idx += 1;
                }
            }
            flat_syls.push(syl_orig);
            is_word_start.push(i == 0); 
            flat_rhythm.push(tonics.contains(&i)); // Guardamos si es fuerte o débil
        }
    }

    let gramatical_str = flat_syls.join("-");
    let gramatical_count = flat_syls.len();

    // 2. Sinalefa con Amalgama Rítmica
    let mut poetic_units = Vec::new();
    let mut poetic_rhythm_vec = Vec::new(); // Lista final de círculos
    let mut j = 0;

    while j < flat_syls.len() {
        let mut unit = flat_syls[j].clone();
        let mut unit_is_tonic = flat_rhythm[j]; // Hereda el ritmo inicial
        
        while j + 1 < flat_syls.len() && is_word_start[j+1] {
            let unit_lower = unit.to_lowercase();
            let last_char = unit_lower.chars().filter(|c| c.is_alphabetic()).last().unwrap_or(' ');
            
            let next_raw = flat_syls[j+1].to_lowercase();
            let next_alpha: String = next_raw.chars().filter(|c| c.is_alphabetic()).collect();
            let mut next_letters = next_alpha.chars();
            
            let first_v = next_letters.next().unwrap_or(' ');
            let second_v = next_letters.next().unwrap_or(' ');
            
            let mut is_next_vocal = "aeiouáéíóúü".contains(first_v) || (first_v == 'h' && "aeiouáéíóúü".contains(second_v));
            if next_alpha == "y" { is_next_vocal = true; }

            if "aeiouáéíóúüy".contains(last_char) && is_next_vocal {
                unit.push_str(&flat_syls[j+1]); 
                if flat_rhythm[j+1] { unit_is_tonic = true; } // Fusión de fuerza
                j += 1;
            } else { break; }
        }
        poetic_units.push(unit);
        poetic_rhythm_vec.push(if unit_is_tonic { "●" } else { "◯" });
        j += 1;
    }

    // 3. Ley del Acento Final (Recalibrado)
    let last_word = words.last().unwrap_or(&"").to_lowercase()
        .chars().filter(|c| c.is_alphabetic() || "áéíóúüÁÉÍÓÚÜ".contains(*c)).collect::<String>();
    
    let mut kind = "grave";
    let mut adjustment = 0;

    if !last_word.is_empty() {
        let last_word_syls = get_word_syllables(&last_word);
        let falsos_adverbios = [
            "mente", "amente", "demente", "clemente", "inclemente", "vehemente", "simiente",
            "lamente", "comente", "alimente", "ornamente", "pavimente", "aumente", "fomente", "experimente", 
            "fermente", "fragmente", "implemente", "documente", "parlamente", "pigmente"
        ];
        
        if last_word.ends_with("mente") && !falsos_adverbios.contains(&last_word.as_str()) {
            kind = "doble acento"; 
            adjustment = 0;
        } else if last_word_syls.len() == 1 {
            kind = "aguda"; 
            adjustment = 1;
        } else {
            let mut has_tilde = false;
            let mut tilde_pos = 0; 

            for (i, syl) in last_word_syls.iter().rev().enumerate() {
                if syl.chars().any(|c| "áéíóúÁÉÍÓÚ".contains(c)) {
                    has_tilde = true;
                    tilde_pos = i + 1;
                    break;
                }
            }

            if has_tilde {
                match tilde_pos {
                    1 => { kind = "aguda"; adjustment = 1; },
                    2 => { kind = "grave"; adjustment = 0; },
                    _ => { kind = "esdrújula"; adjustment = -1; },
                }
            } else {
                let last_char = last_word.chars().last().unwrap_or(' ');
                if "nsaeiou".contains(last_char) { kind = "grave"; adjustment = 0; } 
                else { kind = "aguda"; adjustment = 1; }
            }
        }
    }

    let final_count = (poetic_units.len() as i32 + adjustment).max(0) as usize;
    // --- NUEVO: EXTRAER LA TERMINACIÓN ---
    let last_word_syls = get_word_syllables(&last_word); // Recuperamos las sílabas de la última palabra
    let terminacion = extract_rhyme_block(&last_word, &last_word_syls);

    Analysis {
        gramatical: gramatical_str,
        gramatical_count,
        adjustment,
        poetica: poetic_units.join("-"),
        count: final_count,
        kind: kind.into(),
        ritmo: poetic_rhythm_vec.join(" "), // Enviamos los círculos al frontend
        terminacion,
    }
}

// Punto de entrada de Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_line])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}