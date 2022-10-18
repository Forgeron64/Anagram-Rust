use std::iter::Iterator;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

#[derive(Debug)]
struct Mots {
    nom_origine: String,
    nom: String,
}

impl Mots {
    pub fn new(nom_origine: String, nom: String) -> Self {
        Mots {
            nom_origine,
            nom
        }
    }
    
}

fn main() {

    let mut v = Vec::new(); 


    let filename = "src/liste_anglais.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        v.push(line);
    }

    let mut file = File::create("./src/tata.txt")
    .expect("Error encountered while creating file!");



    // vec<String> to vec<Mot> avec char_to_string_alphabeticaly

    let mut dico: Vec<Mots> = v.iter().map(|x|Mots::new(x.to_string(),char_to_string_alphabeticaly(x.to_string().chars().collect::<Vec<char>>()).to_lowercase())).collect();


    dico.sort_by(|a, b| a.nom.cmp(&b.nom));

    

    let mut string_reference = &dico[0].nom;
    let mut acc: Vec<&Mots> = Vec::new();
    let mut dico_final: Vec<Vec<&Mots>> = Vec::new();



    for word in &dico {
        if word.nom.ne(string_reference) {
            dico_final.push(acc);
            acc = Vec::new();
            string_reference = &word.nom;
        } 
        
        acc.push(word);
    }

    dico_final.push(acc);

    
    
    dico_final.retain(|x| x.len() > 1);

    
    let dictionnaire = dico_final.join(&[][..]);


    // reatribue les mots d'origine 


    for word in dictionnaire {
        let local = word.nom_origine.clone();
       // dico.push(local);
        file.write(local.as_bytes())
   .expect("Error while writing to file");
   
   file.write(b"\n")
   .expect("Error while writing to file");
    }




}

// Prends en paramêtre un vecteur de chaine de caractere

fn char_to_string_alphabeticaly(x: Vec<char>) -> String {
    let mut local = x.clone();
        local.sort();
        String::from_iter(local)
}

