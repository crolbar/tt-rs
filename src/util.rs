use std::{env, fs::File};
use anyhow::{Context, Result};
use rand::{seq::SliceRandom, thread_rng, Rng};
use ron::de::from_reader;

pub fn get_random_quotes() -> Result<Vec<char>> {
    let file_path = {
        let xdg_conf_path = format!("{}/tt-rs/quotes.ron", env::var("XDG_CONFIG_HOME")?);

        if std::fs::metadata(&xdg_conf_path).is_err() {
            format!("{}/.config/tt-rs/quotes.ron", env::var("HOME")?)
        } else {
            xdg_conf_path
        }
    };

    let conts: Vec<String> = from_reader(
        File::open(file_path).with_context(|| "quotes.ron(~/.config/tt-rs/quotes.ron) file is incorrect or missing")?
    )?;

    let vec_conts: Vec<Vec<char>> = conts.iter()
        .map(|s| s.chars().collect())
        .collect();

    let random_idx = thread_rng().gen_range(0..conts.len());
    Ok(vec_conts[random_idx].clone())
}

pub fn get_random_words(args: &Vec<String>) -> Result<Vec<char>> {
    let file_path = {
        let xdg_conf_path = format!("{}/tt-rs/words.ron", env::var("XDG_CONFIG_HOME")?);

        if std::fs::metadata(&xdg_conf_path).is_err() {
            format!("{}/.config/tt-rs/words.ron", env::var("HOME")?)
        } else {
            xdg_conf_path
        }
    };

    let mut conts: Vec<String> = from_reader(
        File::open(file_path).with_context(|| "words.ron(~/.config/tt-rs/words.ron) file is incorrect or missing")?
    )?;

    conts.shuffle(&mut thread_rng());

    let txt_len = {
        if let Some(length) = args.iter().position(|i| i == &"-w".to_string()) {
            args.get(length + 1)
                .with_context(|| "add number after -w (e.g: -w 30)")?
                .parse()
                .with_context(|| "incorrect number: add number after -w (e.g: -w 30)")?
        } else { 25 }
    };

    let vec_conts: Vec<Vec<char>> = conts.iter()
        .map(|s| s.chars().collect())
        .collect();
    Ok(vec_conts[..txt_len].join(&' '))
}


pub fn get_prev_whitespace(str: &Vec<char>, idx: usize) -> usize {
    for i in (0..idx).rev() {
        if let Some(char) = str.get(i) {
            if *char == ' ' {
                return i;
            }
        }
    }

    return 0;
}

pub fn get_next_whitespace(str: &Vec<char>, idx: usize) -> usize {
    for i in idx..str.len() {
        if let Some(char) = str.get(i) {
            if *char == ' ' {
                return i;
            }
        }
    }

    return idx;
}

pub fn logg(str: String) {
    if let Ok(conts) = std::fs::read_to_string("log") {
        std::fs::write("log", conts + "\n" + &str).unwrap();
    }
}
