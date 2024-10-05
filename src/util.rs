use std::{env, fs::File};
use anyhow::{Context, Result};
use rand::{seq::SliceRandom, thread_rng, Rng};
use ron::de::from_reader;

pub fn get_random_quotes() -> Result<String> {
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

    Ok(conts[thread_rng().gen_range(0..conts.len())].clone())
}

pub fn get_random_words(args: &Vec<String>) -> Result<String> {
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

    let mut rng = thread_rng();

    conts.shuffle(&mut rng);

    let txt_len = {
        if let Some(length) = args.iter().position(|i| i == &"-w".to_string()) {
            args.get(length + 1)
                .with_context(|| "add number after -w (e.g: -w 30)")?
                .parse()
                .with_context(|| "incorrect number: add number after -w (e.g: -w 30)")?
        } else { 25 }
    };

    Ok(conts[..txt_len].join(" "))
}


pub fn get_prev_whitespace(str: &String, idx: usize) -> usize {
    for i in (0..idx).rev() {
        if let Some(char) = str.chars().nth(i) {
            if char == ' ' {
                return i;
            }
        }
    }

    return 0;
}
