use std::fs::File;
use std::io::Read;
use toml::Table;

pub fn read_toml_to_table(file_path: String) -> Table {
    let mut fobj = match File::open(file_path) {
        Ok(fobj) => fobj,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    let mut contents = String::new();
    fobj.read_to_string(&mut contents).expect("failed to read file to string");

    contents.parse::<Table>().expect("failed to parse toml to table")
}
