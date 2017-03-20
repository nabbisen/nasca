extern crate hyper;
extern crate hyper_router;

use std::fs::File;
use std::io::prelude::*;

use hyper::server::{Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

pub fn root(_: Request, res: Response) {
  serve("www/index.html", res);
}

pub fn serve_static(req: Request, res: Response) {
  match req.uri {
    AbsolutePath(ref path) => {
      serve(path, res);
    }
    _ => {
      panic!();
    }
  };
}

pub fn greet(req: Request, res: Response) {
  println!("{}", req.uri);
  res.send(b"Hello, world.").unwrap();
}

fn serve(path: &str, res: Response) {
  let p = if path.starts_with("/") { [".", path].concat() } else { path.to_string() };
  let mut file = File::open(p).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  res.send(contents.as_bytes()).unwrap();
}
