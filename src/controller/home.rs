use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse, Responder};
use controller::Controller;
use template::MainTemplate;
use util::PageBuilder;

pub struct Home;

impl Home {
    pub fn new() -> Self {
        Home {}
    }

    pub fn get_server_ip(&self) -> Result<String> {
        use std::process::Command;
        let instance_info = Command::new("gcloud")
            .arg("compute")
            .arg("instances")
            .arg("list")
            .arg("--filter=game-server")
            .arg("--format=config")
            .output()?
            .stdout;
        let instance_info = String::from_utf8(instance_info)?;

        let natip_line = instance_info.lines()
            .filter(|line| line.starts_with("natIP"))
            .take(1)
            .collect::<Vec<&str>>()
            .pop()
            .ok_or(ControllerError::from("Game server is not running"))?;

        let ip = natip_line.split(' ')
            .skip(2)
            .take(1)
            .next()
            .expect("Illegal data format from gcloud!");

        Ok(ip.to_owned())
    }

}

impl Controller for Home {
    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let builder = match self.match_tail(req) {
            Err(_) => PageBuilder::not_found(&self.hb),
            Ok(path) => PageBuilder::from_file(&self.hb, path),
        };

        let main = MainTemplate::new();
        let server = ServerTemplate::new();

        builder.render_template(main)
            .render_template(server)
            .finish()
    }
}
