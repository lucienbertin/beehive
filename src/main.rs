use std::{fs::File, io::{Read, Result, BufReader, Write}, time::Instant};

use beehive::Beehive;
use regex::Regex;

mod dictionary;
// mod waffle;
mod beehive;
mod grid;

fn main() -> Result<()> {
    let start = Instant::now();

    gen_beehive()?;

    let elapsed = start.elapsed();
    println!("beehive generated in {:?}", elapsed);

    Ok(())
}

fn gen_beehive() -> std::io::Result<()> {
    let dictionary = dictionary::Dictionary::new()?;

    let empty_beehive = beehive::MediumBeehive::gen_empty();

    let res = beehive::recursive_generate(&dictionary, empty_beehive, 0);

    match res {
        Some(mut beehive) => {
            beehive.shuffle();
            println!("{:?}", beehive);
            println!("{}", beehive);
        },
        None => println!("no beehive found")
    };

    Ok(())
}

fn _parse_whole_xml() -> std::io::Result<()> {
    let start = Instant::now();
    let docs = open_xml("samples/enwiktionary-latest-abstract.xml")?;
    let elapsed = start.elapsed();
    println!("parsed in {:?}", elapsed);
    println!("{:?} words al languages", docs.len());

    // english
    let en_words = filter_lang(&docs, "==English==")?;
    println!("{:?} words not cleaned up", en_words.len());
    for word in &en_words[0..3] {
        println!("{:?}", word);
    };
    let en_words = clean_up(&en_words)?;
    println!("{:?} words cleaned up", en_words.len());
    for word in &en_words[0..3] {
        println!("{:?}", word);
    };
    let max_len = get_max_len(&en_words)?;
    println!("{:?} max length", max_len);

    let path = "dictionaries/english/all_words";
    write_dictionary(path, &en_words)?;

    for i in 1..(max_len + 1) {
        let i_lenght_words = filter_len(&en_words, i)?;
        if i_lenght_words.len() > 0 {
            let path = format!("dictionaries/english/{}_letters_words", i);
            write_dictionary(path.as_str(), &i_lenght_words)?;
        }
    }
    let elapsed = start.elapsed();
    println!("english processed in {:?}", elapsed);

    println!("{:?} words", en_words.len());

    // frenchh
    let fr_words = filter_lang(&docs, "==French==")?;
    println!("{:?} words not cleaned up", fr_words.len());
    for word in &fr_words[0..3] {
        println!("{:?}", word);
    };
    let fr_words = clean_up(&fr_words)?;
    println!("{:?} words cleaned up", fr_words.len());
    for word in &fr_words[0..3] {
        println!("{:?}", word);
    };
    let max_len = get_max_len(&fr_words)?;
    println!("{:?} max length", max_len);

    let path = "dictionaries/french/all_words";
    write_dictionary(path, &fr_words)?;

    for i in 1..(max_len + 1) {
        let i_lenght_words = filter_len(&fr_words, i)?;
        if i_lenght_words.len() > 0 {
            let path = format!("dictionaries/french/{}_letters_words", i);
            write_dictionary(path.as_str(), &i_lenght_words)?;
        }
    }
    let elapsed = start.elapsed();
    println!("french processed in {:?}", elapsed);

    println!("{:?} words", fr_words.len());
    Ok(())
}

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
pub struct Feed {
    #[serde(rename = "$value")]
    pub docs: Vec<Doc>,
}
#[derive(Debug, Deserialize)]
pub struct Doc {
    pub title: String,
    #[serde(rename = "abstract")]
    pub abs: String,
}

pub fn open_xml(file_path: &str) -> Result<Vec<Doc>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let feed: Feed = quick_xml::de::from_reader(reader).unwrap();

    Ok(feed.docs)
}

pub fn filter_lang(docs: &Vec<Doc>, lang: &str) -> Result<Vec<String>> {
    let filtered_docs = docs
        .into_iter()
        .filter(|d| d.abs.as_str() == lang)
        .map(|d| d.title.clone())
        .collect();

    Ok(filtered_docs)
}

pub fn open_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn extract_words(contents: String) -> Result<Vec<String>> {
    let mut split = contents.split('\n');
    split.next(); // remove header line

    let lines = split.collect::<Vec<&str>>();

    let words: Vec<String> = lines
        .iter()
        .map(|&line| {
            let mut split = line.split('\t');
            split.next(); // skip 1st col
            let word = match split.next() {
                Some(s) => String::from(s),
                None => String::from(""),
            };

            word
        }).collect();
    
    Ok(words)
}

pub fn clean_up(words: &Vec<String>) -> Result<Vec<String>> {
    // let lower_case_only = Regex::new(r"^[a-z]+$").unwrap();
    // let all_case = Regex::new(r"^[a-zA-Z]+$").unwrap();
    let entry_reg = Regex::new(r"^Wiktionary: [a-zA-Z]+$").unwrap();

    let clean_words = words
        .into_iter()
        .filter(|w| entry_reg.is_match(w))
        .map(|s| String::from(s))
        .map(|s| s.as_str()[12..].to_string())
        .collect::<Vec<String>>();

    Ok(clean_words)
}

pub fn filter_len(words: &Vec<String>, len: usize) -> Result<Vec<String>> {
    let correct_len = words
        .into_iter().filter(|w| w.len() == len)
        .map(|s| String::from(s))
        .collect();

    Ok(correct_len)
}

pub fn get_max_len(words:&Vec<String>) -> Result<usize> {
    let mut max_len = 0;
    for word in words {
        max_len = std::cmp::max(max_len, word.len());
    };

    Ok(max_len)
}

pub fn write_dictionary(path: &str, words:&Vec<String>) -> Result<()> {
    let mut file = File::create(path)?;
    for word in words {
        file.write_all(word.as_bytes())?;
        file.write_all(b"\n")?;
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use regex::Regex;

    use crate::{open_file, extract_words, clean_up, filter_len, get_max_len, write_dictionary, open_xml, filter_lang};

    #[test]
    fn parse_300_lines() -> std::io::Result<()> {
        let start = Instant::now();

        let contents = open_file("samples/300-first-lines")?;

        let words = extract_words(contents)?;
        let words = clean_up(&words)?;
        
        for word in &words {
            println!("{:?}", word);
        }
        println!("{:?} words", words.len());

        let elapsed = start.elapsed();
        println!("parsed and serialized in {:?}", elapsed);
        Ok(())
    }
    #[test]
    fn parse_100k_lines() -> std::io::Result<()> {
        let start = Instant::now();

        let contents = open_file("samples/100k-first-lines")?;

        let words = extract_words(contents)?;
        println!("{:?} words not cleaned up", words.len());
        let words = clean_up(&words)?;
        
        for word in &words {
            println!("{:?}", word);
        }
        println!("{:?} words", words.len());

        let elapsed = start.elapsed();
        println!("parsed and serialized in {:?}", elapsed);
        Ok(())
    }

    #[test]
    fn parse_2m_lines() -> std::io::Result<()> {
        let start = Instant::now();

        let contents = open_file("samples/2M-first-lines")?;

        let words = extract_words(contents)?;
        println!("{:?} words not cleaned up", words.len());
        let words = clean_up(&words)?;
        println!("{:?} words cleaned up", words.len());

        let max_len = get_max_len(&words)?;
        println!("{:?} max length", max_len);

        for i in 2..(max_len + 1) {
            let i_lenght_words = filter_len(&words, i)?;
            if i_lenght_words.len() > 0 {
                let path = format!("dictionaries/test_{}", i);
                write_dictionary(path.as_str(), &i_lenght_words)?;
            }
        }


        // let five_letters_words = filter_len(&words, 5)?;
        // for word in &five_letters_words {
        //     println!("{:?}", word);
        // }
        // println!("{:?} 5-letters words", five_letters_words.len());

        // write_dictionary("dictionaries/test_5_letters", &five_letters_words)?;

        // let six_letters_words = filter_len(&words, 6)?;
        // for word in &six_letters_words {
        //     println!("{:?}", word);
        // }
        // println!("{:?} 6-letters words", six_letters_words.len());

        let elapsed = start.elapsed();
        println!("parsed and serialized in {:?}", elapsed);
        Ok(())
    }

    #[test]
    fn extract_all_language_entries() -> std::io::Result<()> {
        let start = Instant::now();

        let contents = open_file("samples/enwiktionary-latest-all-titles")?;

        let words = extract_words(contents)?;
        println!("{:?} words not cleaned up", words.len());
        let words = clean_up(&words)?;
        println!("{:?} words cleaned up", words.len());

        let max_len = get_max_len(&words)?;
        println!("{:?} max length", max_len);

        for i in 2..(max_len + 1) {
            let i_lenght_words = filter_len(&words, i)?;
            if i_lenght_words.len() > 0 {
                let path = format!("dictionaries/all_languages/{}_letters_words", i);
                write_dictionary(path.as_str(), &i_lenght_words)?;
            }
        }

        let elapsed = start.elapsed();
        println!("parsed and serialized in {:?}", elapsed);
        Ok(())
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(r"^[a-zA-Z]+$").unwrap();

        let m =re.is_match("haystack");
        println!("haystack match ? {:?}", m);
        let m =re.is_match("HaySTack");
        println!("HaySTack match ? {:?}", m);

        let m =re.is_match("haystack!");
        println!("haystack! match ? {:?}", m);
        
        let m =re.is_match("hàystack");
        println!("hàystack match ? {:?}", m);
        
        let m =re.is_match("تبریز");
        println!("تبریز match ? {:?}", m);

    }

    #[test]
    fn parse_test_xml() -> std::io::Result<()> {
        let docs = open_xml("samples/test.xml")?;
        let en_words = filter_lang(&docs, "==English==")?;
        for word in &en_words {
            println!("{:?}", word);
        }
        println!("{:?} words", en_words.len());
        Ok(())
    }
    #[test]
    fn parse_whole_xml() -> std::io::Result<()> {
        let start = Instant::now();
        let docs = open_xml("samples/enwiktionary-latest-abstract.xml")?;
        let elapsed = start.elapsed();
        println!("parsed in {:?}", elapsed);

        let en_words = filter_lang(&docs, "==English==")?;
        let elapsed = start.elapsed();
        println!("filtered in {:?}", elapsed);

        println!("{:?} words", en_words.len());
        Ok(())
    }

    #[test]
    fn test_substring() -> std::io::Result<()> {
        let word = "Wiktionary: lol".to_string();
        let word = word.as_str()[12..].to_string();

        println!("{}", word);

        Ok(())
    }
}