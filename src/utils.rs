use regex;

use std::collections::HashMap;
use std::io::BufRead;

/// Parse a config file that's made up of lines like "foo=bar".
/// Takes an optional regex filter that will be applied per line.
pub fn parse_config_file<R>(buf: R, filter: Option<&str>, map: &mut HashMap<String, String>)
where
    R: BufRead,
{
    for line in buf.lines().filter_map(Result::ok) {
        let line = line.trim();
        if let Some(f) = filter {
            let re = regex::Regex::new(f).unwrap();
            if !re.is_match(line) {
                continue;
            }
        }
        if let Some(pos) = line.find('=') {
            map.insert(line[..pos].to_string(), line[(pos + 1)..].to_string());
        }
    }
}
