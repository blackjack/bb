extern crate toml;

use std::io;
use std::fs;
use std::io::Read;

#[derive(Debug, RustcDecodable)]
pub struct Config {
    api_key: String,
}


#[derive(Debug)]
pub enum ConfigParseError {
    Io(io::Error),
    ParseError(String),
}


impl From<io::Error> for ConfigParseError {
    fn from(err: io::Error) -> ConfigParseError {
        ConfigParseError::Io(err)
    }
}

impl ConfigParseError {
    fn from_toml_errors(parser: &toml::Parser) -> ConfigParseError {
        let errors = &parser.errors;
        let msgs = errors.into_iter()
            .map(|err| {
                let ((loline, locol), (hiline, hicol)) = (parser.to_linecol(err.lo),
                                                          parser.to_linecol(err.hi));
                format!("[{}:{}-{}:{}]: {}", loline, locol, hiline, hicol, err.desc)
            })
            .collect::<Vec<_>>()
            .join(",");

        ConfigParseError::ParseError(msgs)
    }
}

impl Config {
    pub fn parse(path: &str) -> Result<Config, ConfigParseError> {

        let mut file: fs::File = try!(fs::File::open(path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));

        let mut parser = toml::Parser::new(&contents);
        let val = parser.parse();

        if val.is_none() {
            return Err(ConfigParseError::from_toml_errors(&parser));
        }

        return match toml::decode::<Config>(toml::Value::Table(val.unwrap())) {
            Some(conf) => Ok(conf),
            _ => Err(ConfigParseError::from_toml_errors(&parser)),
        };
    }
}
