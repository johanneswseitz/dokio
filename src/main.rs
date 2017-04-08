#[macro_use] extern crate iron;
extern crate config;
extern crate router;

mod handlers;

use std::path::Path;
use std::collections::HashMap;

use router::Router;

use config::reader;
use config::types::Config;
use iron::prelude::*;

use handlers::{IndexHandler, MarkdownFileHandler};


fn main() {
    let empty_default_config = Config::new(HashMap::new());
    let configuration = reader::from_file(Path::new("Dokifile")).unwrap_or(empty_default_config);

    let listening_port = configuration.lookup_integer32_or("port", 3000);
    let default_file = configuration.lookup_str_or("default_file", "README.md");

    let mut router = Router::new();
    router.get("/", IndexHandler::new(listening_port, String::from(default_file)), "index");
    router.get("/*.md", MarkdownFileHandler::new(), "file");

    let _server = Iron::new(router).http(format!("localhost:{}", listening_port).as_str()).unwrap();
    println!("Doki is running on port {}", listening_port);
}

