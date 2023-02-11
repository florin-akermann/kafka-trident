use hocon::{Error, Hocon, HoconLoader};

pub fn load_config(config_path: &str) -> Result<Hocon, Error> {
    HoconLoader::new()
        .load_file(config_path)?
        .load_str(include_str!("reference.conf"))?
        .hocon()
}
