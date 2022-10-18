use std::iter::Iterator;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::env;

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

    
    // récupère les arguments entrés à la suite de cargo run
    // arg[1] -> localisation fichier dictionnaire
    // arg[2] -> nom + localisation fichier sortie resultats

    let args: Vec<String> = env::args().collect();



    let mut v = Vec::new(); 
    //"src/liste_anglais.txt"

    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        v.push(line);
    }

    //"./src/tata.txt"

    let mut file = File::create(&args[2])
    .expect("Error encountered while creating file!");



    // vec<String> to vec<Mot> avec char_to_string_alphabeticaly

    let mut dico: Vec<Mots> = v.iter().map(|x|Mots::new(x.to_string(),char_to_string_alphabeticaly(x.to_string().to_lowercase()))).collect();

    // Trie le vecteur par ordre alphabetique en fonction de Mots.nom

    dico.sort_by(|a, b| a.nom.cmp(&b.nom));

    
    // On créé un vecteur de vecteurs de mots 
    // Le but est de regrouper dans un vecteur tous les mots.nom qui sont égaux


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

    
    // Les vecteurs ayant une taille > 1 indique qu'ils sont composé d'anagram, On peut donc supprimer les autres mots
    dico_final.retain(|x| x.len() > 1);

    // On joint chacun des vecteurs dans un seul vec<String>    
    let dictionnaire = dico_final.join(&[][..]);



    // On ecrit les mots dans le fichier texte préalablement inséré en argument
    for word in dictionnaire {
        let local = word.nom_origine.clone();
        file.write(local.as_bytes())
   .expect("Error while writing to file");
   
   file.write(b"\n")
   .expect("Error while writing to file");
    }

  

}

// Prends en paramêtre un vecteur de chaine de caractere

fn char_to_string_alphabeticaly(x: String) -> String {
    let mut local = x.clone().chars().collect::<Vec<char>>();
        local.sort();
        String::from_iter(local)
}

