use rand::seq::SliceRandom;
use bip39::Mnemonic;
use rand_core::{OsRng, RngCore};


#[tauri::command]
pub fn generate_password(length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
    ambiguous: bool) -> String {
        let mut dictbuilder = String::new();

        if uppercase {
            dictbuilder.push(' ');
            dictbuilder.push_str("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        }

        if lowercase {
            dictbuilder.push(' ');
            dictbuilder.push_str("a b c d e f g h i j k l m n o p q r s t u v w x y z");
        }

        if numbers {
            dictbuilder.push(' ');
            dictbuilder.push_str("0 1 2 3 4 5 6 7 8 9");  
        }

        if symbols {
            dictbuilder.push(' ');
            dictbuilder.push_str("! @ # $ % ^ & *");  
        }

        if ambiguous {
            dictbuilder = {    
                let mut result = String::new();
                
                for c in dictbuilder.chars(){
                    if !("i l o I O 1 0".contains(c)) {
                        result.push(c);
                        result.push(' ');
                    }
                }

               result
            }
        }

        let dict: Vec<&str> = dictbuilder.split_whitespace().collect();
        let mut result = String::new();

        for _ in 0..length {
            result.push_str(choose_from_dict(&dict, false, "", false).as_str());
        }

        result 
}

fn choose_from_dict(dict: &Vec<&str>, capitalise: bool, seperator: &str, seperate: bool) -> String {
    let mut result = String::new();
    if let Some(word) = dict.choose(&mut rand::thread_rng()) {
        if capitalise {
            let mut word = word.to_string();
            word[..1].make_ascii_uppercase();
            result.push_str(&word);
        } else {
            result.push_str(word);
        }

        if seperate{
            result.push_str(seperator);
        }
    }
    result
}

#[tauri::command]
pub fn generate_passphrase(length: usize, seperator: &str , capitalise: bool) -> String {      
    let dict: Vec<&str> = include_str!("./dict.txt").split_whitespace().collect();
    let mut result = String::new();

    for _ in 0..length-1 {
        result.push_str(choose_from_dict(&dict, capitalise, seperator, true).as_str());
    }

    result.push_str(choose_from_dict(&dict, capitalise, seperator, false).as_str());

    result
}

#[tauri::command]
pub fn generate_bip(words: usize) -> String {
    let ent = (352 * words) / 33 / 8 ;
    let mut share = vec![0u8; ent];
 
    OsRng.fill_bytes(&mut share);   
    Mnemonic::from_entropy(&share).unwrap().to_string()
}