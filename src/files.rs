use crate::game_logic::Game;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

impl Game {
    pub fn archive(&self) {
        std::fs::create_dir_all("./data/old").unwrap();
        let archive_dir = Path::new("./data/old");
        let max_file_num = archive_dir
            .read_dir()
            .expect("Impossible de lire le contenu de `/data/old`")
            .into_iter()
            .filter_map(|dir| dir.ok())
            .map(|dir| dir.file_name())
            .max();
        let mut file = match max_file_num {
            None => File::create("./data/old/0001.json").unwrap(),
            Some(num) => {
                let num = num
                    .to_str()
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                File::create(&format!("./data/old/{:0>4}.json", num + 1)).unwrap()
            }
        };
        file.write_all(serde_json::to_vec_pretty(self).unwrap().as_slice())
            .expect("Impossible d'écrire dans le fichier de sauvegarde");
    }

    pub fn load_current() -> Self {
        let path = Path::new("./data/current.json");
        match path.exists() {
            false => Self::new(),
            true => {
                let reader = BufReader::new(File::open(path).unwrap());
                serde_json::from_reader(reader).unwrap()
            }
        }
    }

    pub fn save_current(&self) {
        let file = File::create("./data/current.json").unwrap();
        let mut writer = BufWriter::new(file);
        writer
            .write_all(serde_json::to_vec_pretty(self).unwrap().as_slice())
            .unwrap();
    }
}
