extern crate iron;
extern crate markdown;

use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{Read,BufReader, Error};

use iron::prelude::*;
use iron::{Url, Handler, status};
use iron::modifiers::Redirect;
use iron::headers::ContentType;

pub struct IndexHandler {
    pub listening_port:i32,
    pub default_file: String
}

impl IndexHandler {
    pub fn new(listening_port:i32, default_file:String) -> IndexHandler {
        IndexHandler {listening_port: listening_port, default_file:default_file}
    }
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let url_string = format!("http://localhost:{}/{}", self.listening_port, self.default_file);
        let url = Url::parse(url_string.as_str()).unwrap();
        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }
}

pub struct MarkdownFileHandler {

}

impl MarkdownFileHandler {
    pub fn new() -> MarkdownFileHandler {
        MarkdownFileHandler {}
    }
}

impl Handler for MarkdownFileHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let markdown_file_path = map_request_url_path_to_local_file(req);
        let file_content = itry!(read_file_to_string(markdown_file_path.as_path()));
        let html : String = markdown::to_html(file_content.as_str());
        Ok(Response::with((ContentType::html().0, status::Ok, html)))
    }

}

fn read_file_to_string(path: &Path) -> Result<String, Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn map_request_url_path_to_local_file(req: &mut Request) -> PathBuf {
    let markdown_file_path_segments = req.url.path();
    let mut path_buf = PathBuf::from(".");
    for path_segment in markdown_file_path_segments {
        path_buf.push(path_segment);
    }
    path_buf
}
