#[macro_use] extern crate nickel;
extern crate config;
extern crate pulldown_cmark;

use nickel::Nickel;

use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,BufReader, Error};

use config::reader;
use config::types::Config;
use pulldown_cmark::Parser;
use pulldown_cmark::html::push_html;

use nickel::*;
use nickel::extensions::Redirect;

fn main() {
    let empty_default_config = Config::new(HashMap::new());
    let configuration = reader::from_file(Path::new("Dokiofile")).unwrap_or(empty_default_config);

    let listening_port = configuration.lookup_integer32_or("port", 3000);
    let default_file = String::from(configuration.lookup_str_or("default_file", "README.md"));
    let theme = String::from(configuration.lookup_str_or("theme", "themes/dokio/index.hbs"));

    let mut server = Nickel::new();
    server.get("/", middleware! { |_, response|
        let url_string = format!("http://localhost:{}/{}", listening_port, default_file.as_str());
        return response.redirect(url_string);
    });
    server.get("**.md", middleware! { |request, response|
        let file = request.path_without_query();
        if let Ok(markdown_file) = read_file_to_string(file) {
            let mut data = HashMap::new();

            let parser = Parser::new(markdown_file.as_str());
            let mut html = String::new();
            push_html(&mut html, parser);

            data.insert("markdown_file", html);
            return response.render(theme.as_str(), &data);
        }
    });
    server.utilize(Mount::new("/", StaticFilesHandler::new(".")));

    let _server = server.listen(format!("localhost:{}", listening_port).as_str());
}

fn read_file_to_string(path: Option<&str>) -> Result<String, Error> {
    let file_path = format!(".{}", path.unwrap());
    let file = File::open(Path::new(file_path.as_str()))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
