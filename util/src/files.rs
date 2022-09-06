use super::lsystem::LSystem;

use project_root;


pub fn get_lsystems(size: &str) -> Vec<LSystem> {
    let mut lsystems = Vec::new();

    let filenames = get_config_filenames(size);

    if let Ok(root) = project_root::get_project_root() {
        if let Some(path_str) = root.as_path().to_str() {
            for fname in filenames {
                lsystems.push(LSystem::from_file(&format!("{}/config/{}", path_str, fname)));
            } 
        }
    }

    lsystems
}

pub fn get_config_filenames(starts_with: &str) -> Vec<String> {
    let mut config_filenames = Vec::new();
    if let Ok(path) = project_root::get_project_root() {
        if let Ok(mut reader) = path.read_dir() {
            if let Some(config_dir) = reader.find(|dir|{
                if let Ok(dir) = dir {
                    if let Some(s) = dir.file_name().to_str() {
                        if s == "config" {
                            return true;
                        }
                    }
                    return false;
                }
                return false;
            }) {
                if let Ok(config_dir) = config_dir {
                    if let Ok(reader) = config_dir.path().read_dir() {
                        for file in reader {
                            if let Ok(file) = file {
                                if let Some(s) = file.file_name().to_str() {
                                    if let Some(pair) = s.split_once('_') {
                                        if pair.0 == starts_with {
                                            config_filenames.push(String::from(s));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    config_filenames
}


pub fn get_file_chooser_string(index: u32) -> String {
    match index {
        0 => String::from("small"),
        1 => String::from("medium"),
        _ => String::from(""),
    }
}
