use std::result::Result;

use grid_beehive::GridBeehive;
// use regex::Regex;

pub mod dictionary;
// mod waffle;
pub mod beehive_swap;
pub mod grid;
pub mod grid_beehive;

use leptos::*;

fn main() {
    use beehive_swap::ui::BeehiveSwapComponent;
    // let mut grid = GridBeehive::new(7,7);
    // grid.set_row(0, "___safe".to_string());
    // grid.set_row(1, "__t_i_d".to_string());
    // grid.set_row(2, "_ew_dig".to_string());
    // grid.set_row(3, "p_o___e".to_string());
    // grid.set_row(4, "oh_mss_".to_string());
    // grid.set_row(5, "o_i_o__".to_string());
    // grid.set_row(6, "path___".to_string());


    let mut grid = GridBeehive::new(6,6);
    grid.set_row(0, "__yeah".to_string());
    grid.set_row(1, "_h__so".to_string());
    grid.set_row(2, "sofa_t".to_string());
    grid.set_row(3, "t_r_i_".to_string());
    grid.set_row(4, "a_ex__".to_string());
    grid.set_row(5, "the___".to_string());
    // let grid = GridBeehive::new_344_honeycomb();
    // let grid = GridBeehive::new_6444_honeycomb();
    // let grid = GridBeehive::new_spotted_champfered(11,15);
    leptos::mount_to_body(move || view! { <BeehiveSwapComponent beehive=grid.clone().into()/> })
}

// fn main() -> Result<(), ()> {
//     let mut input = String::new();
//     println!("number of rows:");
//     let _b1 = std::io::stdin().read_line(&mut input).unwrap();
//     input = input.replace("\n", "");
//     let rows: usize = input
//         .parse()
//         .expect(format!("cant parse {} as usize", input).as_str());
//     let mut input = String::new();
//     println!("number of cols:");
//     let _b1 = std::io::stdin().read_line(&mut input).unwrap();
//     input = input.replace("\n", "");
//     let cols: usize = input
//         .parse()
//         .expect(format!("cant parse {} as usize", input).as_str());

//     let mut input = String::new();
//     println!("number of threads:");
//     let _b1 = std::io::stdin().read_line(&mut input).unwrap();
//     input = input.replace("\n", "");
//     let thread_cnt: usize = input
//         .parse()
//         .expect(format!("cant parse {} as usize", input).as_str());

//     let mut handles = vec![];
//     for i in 0..thread_cnt {
//         let r = *&rows;
//         let c = *&cols;
//         let thread_index = *&i;
//         let handle = std::thread::spawn(move || {
//             let start = Instant::now();
//             // println!("generating a {}x{} grid on thread {}", r, c, thread_index);
//             let res = _gen_grid_beehive(r, c).expect("could not get a working grid");

//             let elapsed = start.elapsed();
//             println!("{}", res);
//             println!("generated in {:?} on thread {}", elapsed, thread_index);
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         let _r = handle.join();
//     }

//     Ok(())
// }

fn _gen_grid(rows: usize, cols: usize) -> Result<grid::Grid, ()> {
    let dictionary = dictionary::Dictionary::new().unwrap();

    let empty = grid::Grid::new(rows, cols);

    let full = empty.recursive_generate(&dictionary);

    match full {
        Some(g) => Ok(g),
        None => Err(()),
    }
}
fn _gen_grid_beehive(rows: usize, cols: usize) -> Result<grid_beehive::GridBeehive, ()> {
    let dictionary = dictionary::Dictionary::new().unwrap();

    let full = match (rows, cols) {
        (1, 1) => {
            let mut empty = GridBeehive::new(4, 4);
            empty.set_row(0, "n\0\0\0".to_string());
            empty.set_row(1, "_\0_\0".to_string());
            empty.set_row(2, "_\0\0\0".to_string());
            empty.set_row(3, "___s".to_string());

            empty.recursive_generate(&dictionary, false)
        }
        (1, 2) => {
            let mut empty = GridBeehive::new(7, 4);
            empty.set_row(0, "___n".to_string());
            empty.set_row(1, "__\0_".to_string());
            empty.set_row(2, "_\0\0_".to_string());
            empty.set_row(3, "\0_\0_".to_string());
            empty.set_row(4, "\0\0__".to_string());
            empty.set_row(5, "\0___".to_string());
            empty.set_row(6, "t___".to_string());

            empty.recursive_generate(&dictionary, false)
        }
        (1, 3) => {
            let mut empty = GridBeehive::new(4, 7);
            empty.set_row(0, "______s".to_string());
            empty.set_row(1, "___\0\0\0_".to_string());
            empty.set_row(2, "__\0_\0__".to_string());
            empty.set_row(3, "t\0\0\0___".to_string());

            empty.recursive_generate(&dictionary, false)
        }
        (3, 4) => GridBeehive::new_343_honeycomb().recursive_generate(&dictionary, false),
        (4, 3) => GridBeehive::new_344_honeycomb().recursive_generate(&dictionary, false),
        (4, 4) => GridBeehive::new_444_honeycomb().recursive_generate(&dictionary, false),
        (5, 5) => GridBeehive::new_5x5_honeycomb().recursive_generate(&dictionary, false),
        (5, 6) => GridBeehive::new_5x6_honeycomb().recursive_generate(&dictionary, false),
        (6, 4) => GridBeehive::new_6444_honeycomb().recursive_generate(&dictionary, true),
        (6, 6) => GridBeehive::new_6x6_honeycomb().recursive_generate(&dictionary, true),
        (7, 7) => GridBeehive::new_7x7_honeycomb().recursive_generate(&dictionary, false),
        (r, c) => GridBeehive::new_spotted_champfered(r, c).recursive_generate(&dictionary, true),
    };

    match full {
        Some(g) => Ok(g),
        None => Err(()),
    }
}

// fn _parse_whole_xml() -> std::result::Result<(), ()> {
//     let start = Instant::now();
//     let docs = open_xml("samples/enwiktionary-latest-abstract.xml")?;
//     let elapsed = start.elapsed();
//     println!("parsed in {:?}", elapsed);
//     println!("{:?} words al languages", docs.len());

//     // english
//     let en_words = filter_lang(&docs, "==English==")?;
//     println!("{:?} words not cleaned up", en_words.len());
//     for word in &en_words[0..3] {
//         println!("{:?}", word);
//     }
//     let en_words = clean_up(&en_words)?;
//     println!("{:?} words cleaned up", en_words.len());
//     for word in &en_words[0..3] {
//         println!("{:?}", word);
//     }
//     let max_len = get_max_len(&en_words)?;
//     println!("{:?} max length", max_len);

//     let path = "dictionaries/english/all_words";
//     write_dictionary(path, &en_words)?;

//     for i in 1..(max_len + 1) {
//         let i_lenght_words = filter_len(&en_words, i)?;
//         if i_lenght_words.len() > 0 {
//             let path = format!("dictionaries/english/{}_letters_words", i);
//             write_dictionary(path.as_str(), &i_lenght_words)?;
//         }
//     }
//     let elapsed = start.elapsed();
//     println!("english processed in {:?}", elapsed);

//     println!("{:?} words", en_words.len());

//     // frenchh
//     let fr_words = filter_lang(&docs, "==French==")?;
//     println!("{:?} words not cleaned up", fr_words.len());
//     for word in &fr_words[0..3] {
//         println!("{:?}", word);
//     }
//     let fr_words = clean_up(&fr_words)?;
//     println!("{:?} words cleaned up", fr_words.len());
//     for word in &fr_words[0..3] {
//         println!("{:?}", word);
//     }
//     let max_len = get_max_len(&fr_words)?;
//     println!("{:?} max length", max_len);

//     let path = "dictionaries/french/all_words";
//     write_dictionary(path, &fr_words)?;

//     for i in 1..(max_len + 1) {
//         let i_lenght_words = filter_len(&fr_words, i)?;
//         if i_lenght_words.len() > 0 {
//             let path = format!("dictionaries/french/{}_letters_words", i);
//             write_dictionary(path.as_str(), &i_lenght_words)?;
//         }
//     }
//     let elapsed = start.elapsed();
//     println!("french processed in {:?}", elapsed);

//     println!("{:?} words", fr_words.len());
//     Ok(())
// }

// use serde_derive::Deserialize;

// #[derive(Debug, Deserialize)]
// #[serde(rename = "feed")]
// pub struct Feed {
//     #[serde(rename = "$value")]
//     pub docs: Vec<Doc>,
// }
// #[derive(Debug, Deserialize)]
// pub struct Doc {
//     pub title: String,
//     #[serde(rename = "abstract")]
//     pub abs: String,
// }

// pub fn open_xml(file_path: &str) -> Result<Vec<Doc>, ()> {
//     let file = File::open(file_path).map_err(|_e| ())?;
//     let reader = BufReader::new(file);

//     let feed: Feed = quick_xml::de::from_reader(reader).unwrap();

//     Ok(feed.docs)
// }

// pub fn filter_lang(docs: &Vec<Doc>, lang: &str) -> Result<Vec<String>, ()> {
//     let filtered_docs = docs
//         .into_iter()
//         .filter(|d| d.abs.as_str() == lang)
//         .map(|d| d.title.clone())
//         .collect();

//     Ok(filtered_docs)
// }

// pub fn open_file(path: &str) -> Result<String, ()> {
//     let file = File::open(path).map_err(|_e| ())?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents).map_err(|_e| ())?;

//     Ok(contents)
// }

// pub fn extract_words(contents: String) -> Result<Vec<String>, ()> {
//     let mut split = contents.split('\n');
//     split.next(); // remove header line

//     let lines = split.collect::<Vec<&str>>();

//     let words: Vec<String> = lines
//         .iter()
//         .map(|&line| {
//             let mut split = line.split('\t');
//             split.next(); // skip 1st col
//             let word = match split.next() {
//                 Some(s) => String::from(s),
//                 None => String::from(""),
//             };

//             word
//         })
//         .collect();

//     Ok(words)
// }

// pub fn clean_up(words: &Vec<String>) -> Result<Vec<String>, ()> {
//     // let lower_case_only = Regex::new(r"^[a-z]+$").unwrap();
//     // let all_case = Regex::new(r"^[a-zA-Z]+$").unwrap();
//     let entry_reg = Regex::new(r"^Wiktionary: [a-zA-Z]+$").unwrap();

//     let clean_words = words
//         .into_iter()
//         .filter(|w| entry_reg.is_match(w))
//         .map(|s| String::from(s))
//         .map(|s| s.as_str()[12..].to_string())
//         .collect::<Vec<String>>();

//     Ok(clean_words)
// }

// pub fn filter_len(words: &Vec<String>, len: usize) -> Result<Vec<String>, ()> {
//     let correct_len = words
//         .into_iter()
//         .filter(|w| w.len() == len)
//         .map(|s| String::from(s))
//         .collect();

//     Ok(correct_len)
// }

// pub fn get_max_len(words: &Vec<String>) -> Result<usize, ()> {
//     let mut max_len = 0;
//     for word in words {
//         max_len = std::cmp::max(max_len, word.len());
//     }

//     Ok(max_len)
// }

// pub fn write_dictionary(path: &str, words: &Vec<String>) -> Result<(), ()> {
//     let mut file = File::create(path).map_err(|_e| ())?;
//     for word in words {
//         file.write_all(word.as_bytes()).map_err(|_e| ())?;
//         file.write_all(b"\n").map_err(|_e| ())?;
//     }

//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use regex::Regex;
//     use std::time::Instant;

//     use crate::{
//         clean_up, extract_words, filter_lang, filter_len, get_max_len, open_file, open_xml,
//         write_dictionary,
//     };

//     #[test]
//     fn parse_300_lines() -> std::result::Result<(), ()> {
//         let start = Instant::now();

//         let contents = open_file("samples/300-first-lines")?;

//         let words = extract_words(contents)?;
//         let words = clean_up(&words)?;

//         for word in &words {
//             println!("{:?}", word);
//         }
//         println!("{:?} words", words.len());

//         let elapsed = start.elapsed();
//         println!("parsed and serialized in {:?}", elapsed);
//         Ok(())
//     }
//     #[test]
//     fn parse_100k_lines() -> std::result::Result<(), ()> {
//         let start = Instant::now();

//         let contents = open_file("samples/100k-first-lines")?;

//         let words = extract_words(contents)?;
//         println!("{:?} words not cleaned up", words.len());
//         let words = clean_up(&words)?;

//         for word in &words {
//             println!("{:?}", word);
//         }
//         println!("{:?} words", words.len());

//         let elapsed = start.elapsed();
//         println!("parsed and serialized in {:?}", elapsed);
//         Ok(())
//     }

//     #[test]
//     fn parse_2m_lines() -> std::result::Result<(), ()> {
//         let start = Instant::now();

//         let contents = open_file("samples/2M-first-lines")?;

//         let words = extract_words(contents)?;
//         println!("{:?} words not cleaned up", words.len());
//         let words = clean_up(&words)?;
//         println!("{:?} words cleaned up", words.len());

//         let max_len = get_max_len(&words)?;
//         println!("{:?} max length", max_len);

//         for i in 2..(max_len + 1) {
//             let i_lenght_words = filter_len(&words, i)?;
//             if i_lenght_words.len() > 0 {
//                 let path = format!("dictionaries/test_{}", i);
//                 write_dictionary(path.as_str(), &i_lenght_words)?;
//             }
//         }

//         // let five_letters_words = filter_len(&words, 5)?;
//         // for word in &five_letters_words {
//         //     println!("{:?}", word);
//         // }
//         // println!("{:?} 5-letters words", five_letters_words.len());

//         // write_dictionary("dictionaries/test_5_letters", &five_letters_words)?;

//         // let six_letters_words = filter_len(&words, 6)?;
//         // for word in &six_letters_words {
//         //     println!("{:?}", word);
//         // }
//         // println!("{:?} 6-letters words", six_letters_words.len());

//         let elapsed = start.elapsed();
//         println!("parsed and serialized in {:?}", elapsed);
//         Ok(())
//     }

//     #[test]
//     fn extract_all_language_entries() -> std::result::Result<(), ()> {
//         let start = Instant::now();

//         let contents = open_file("samples/enwiktionary-latest-all-titles")?;

//         let words = extract_words(contents)?;
//         println!("{:?} words not cleaned up", words.len());
//         let words = clean_up(&words)?;
//         println!("{:?} words cleaned up", words.len());

//         let max_len = get_max_len(&words)?;
//         println!("{:?} max length", max_len);

//         for i in 2..(max_len + 1) {
//             let i_lenght_words = filter_len(&words, i)?;
//             if i_lenght_words.len() > 0 {
//                 let path = format!("dictionaries/all_languages/{}_letters_words", i);
//                 write_dictionary(path.as_str(), &i_lenght_words)?;
//             }
//         }

//         let elapsed = start.elapsed();
//         println!("parsed and serialized in {:?}", elapsed);
//         Ok(())
//     }

//     #[test]
//     fn test_regex() {
//         let re = Regex::new(r"^[a-zA-Z]+$").unwrap();

//         let m = re.is_match("haystack");
//         println!("haystack match ? {:?}", m);
//         let m = re.is_match("HaySTack");
//         println!("HaySTack match ? {:?}", m);

//         let m = re.is_match("haystack!");
//         println!("haystack! match ? {:?}", m);

//         let m = re.is_match("hàystack");
//         println!("hàystack match ? {:?}", m);

//         let m = re.is_match("تبریز");
//         println!("تبریز match ? {:?}", m);
//     }

//     #[test]
//     fn parse_test_xml() -> std::result::Result<(), ()> {
//         let docs = open_xml("samples/test.xml")?;
//         let en_words = filter_lang(&docs, "==English==")?;
//         for word in &en_words {
//             println!("{:?}", word);
//         }
//         println!("{:?} words", en_words.len());
//         Ok(())
//     }
//     #[test]
//     fn parse_whole_xml() -> std::result::Result<(), ()> {
//         let start = Instant::now();
//         let docs = open_xml("samples/enwiktionary-latest-abstract.xml")?;
//         let elapsed = start.elapsed();
//         println!("parsed in {:?}", elapsed);

//         let en_words = filter_lang(&docs, "==English==")?;
//         let elapsed = start.elapsed();
//         println!("filtered in {:?}", elapsed);

//         println!("{:?} words", en_words.len());
//         Ok(())
//     }

//     #[test]
//     fn test_substring() -> std::result::Result<(), ()> {
//         let word = "Wiktionary: lol".to_string();
//         let word = word.as_str()[12..].to_string();

//         println!("{}", word);

//         Ok(())
//     }
// }
