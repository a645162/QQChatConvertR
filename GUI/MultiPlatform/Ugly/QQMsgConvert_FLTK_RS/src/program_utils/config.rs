use std::collections::HashMap;
use std::path::Path;
use ini::Ini;

pub fn read_ini_hashmap<P: AsRef<Path>>(ini_file_path: P)
                                        -> HashMap<String, HashMap<String, String>> {
    let ini_file =
        Ini::load_from_file(ini_file_path)
            .expect("Error loading ini file!");

    let mut ini_hashmap: HashMap<String, HashMap<String, String>> =
        HashMap::new();

    for (section, prop) in ini_file.iter() {
        match section {
            Some(section_str) => {
                let section_string = String::from(section_str.trim());

                let mut child_hashmap: HashMap<String, String> = HashMap::new();

                for (key, value) in prop.iter() {
                    child_hashmap.insert(
                        String::from(key.trim()),
                        String::from(value.trim()),
                    );
                }

                ini_hashmap.insert(section_string, child_hashmap);
            }
            None => {
                // 忽略掉 None
                continue;
            }
        }
    }

    return ini_hashmap;
}

pub fn get_value(
    ini_hashmap: HashMap<String, HashMap<String, String>>,
    section: &str,
    key: &str,
    default_value: &str,
) -> String {
    match ini_hashmap.get(section) {
        Some(s_hashmap) => {
            match s_hashmap.get(key) {
                Some(s) => {
                    String::from(s.trim())
                }
                None => {
                    String::from(default_value.trim())
                }
            }
        }
        None => {
            String::from(default_value.trim())
        }
    }
}