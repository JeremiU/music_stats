extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashSet;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;

use data_format::LifeTimeEntry;
use data_format::StatField;

mod data_format;

fn main() {
    let k = get_data("data");
    println!("{:#?}", &k);
    println!("Total MS {}", get_total_ms(&k));
    println!("Total songs: {}", get_unique_tracks(&k).len());
}

fn get_data(directory: &str) -> Vec<LifeTimeEntry> {
    let mut data: Vec<LifeTimeEntry> = Vec::new();

    match read_dir(directory) {
        Ok(dir) => {
            println!("hi");
            for file in dir {
                match file {
                    Ok(de) => {
                        if de.file_name().to_str().unwrap().ends_with(".json") {
                            println!("JSON");

                            let f = File::open(de.path());

                            match f {
                                Ok(file) => {
                                    let reader = BufReader::new(file);

                                    let res: Result<Vec<LifeTimeEntry>, serde_json::Error> =
                                        serde_json::from_reader(reader);

                                    match res {
                                        Ok(k) => data.extend(k),
                                        Err(e) => {
                                            println!("Deformed JSON file!");
                                            println!("{:?}", e.classify());
                                            println!("line: {}", e.line());
                                            println!("column: {}", e.column());
                                        }
                                    }
                                }
                                Err(e) => println!("Error: {}", e),
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        Err(e) => {
            println!("Invalid directory! Error: {}", e);
        }
    };
    data
}

fn get_total_ms(data: &Vec<LifeTimeEntry>) -> u64 {
    data.iter().fold(0, |f, x| f + x.ms_played)
}

fn get_unique_field(dataType: StatField, data: &Vec<LifeTimeEntry>) -> HashSet<String> {
    data.iter()
        .fold(HashSet::new(), |mut v: HashSet<String>, x| {
            let mut k;
            let d = "".to_string();

            match dataType {
                StatField::AlbumName => k = &x.master_metadata_album_album_name,
                StatField::ArtistName => k = &x.master_metadata_album_artist_name,
                StatField::Country => k = &Some(x.conn_country),
                StatField::TrackName => k = &x.master_metadata_track_name,
                StatField::Platform => k = &Some(x.platform),
                _ => k = &None,
            }

            match k {
                Some(val) => {
                    v.insert(val.to_string());
                    v
                }
                None => v,
            }
        })
}

fn get_unique_tracks(data: &Vec<LifeTimeEntry>) -> HashSet<String> {
    get_unique_field(StatField::TrackName, data)
}
