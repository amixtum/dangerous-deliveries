use super::lsystem::LSystem;

use project_root;

pub fn get_lsystem_filenames() -> Vec<String> {
    get_config_filenames("lsystem")
}

pub fn get_model_filenames() -> Vec<String> {
    get_config_filenames("model")
}

pub fn get_table_filenames() -> Vec<String> {
    get_config_filenames("table")
}

pub fn get_lsystems() -> Vec<LSystem> {
    let mut lsystems = Vec::new();

    let filenames = get_lsystem_filenames();

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
            if let Some(config_dir) = reader.find(|dirname| {
                if let Ok(dirname) = dirname {
                    if let Some(s) = dirname.file_name().to_str() {
                        return s == "config";
                    }
                }
                false
            }) {
                if let Ok(config_dir) = config_dir {
                    if let Ok(reader) = config_dir.path().read_dir() {
                        reader.for_each(|file| {
                            if let Ok(file) = file {
                                if let Some(filename) = file.file_name().to_str() {
                                    if &filename[0..starts_with.chars().count()] == starts_with {
                                        config_filenames.push(String::from(filename));
                                    }
                                }
                            }
                        });
                    }
                }
            }
        }
    }

    config_filenames
}
