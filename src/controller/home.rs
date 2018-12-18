use actix_web::{HttpRequest, HttpResponse};
use controller::{Controller, ControllerError, Result};
use template::{MainTemplate, ServerTemplate};
use util::PageBuilder;

use handlebars::Handlebars;

pub struct Home {
    hb: Handlebars
}

impl Home {
    pub fn new() -> Self {
        Home {
            hb: Handlebars::new(),
        }
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
        let main = MainTemplate::new();
        let server = ServerTemplate::new(self.get_server_ip().ok());

        PageBuilder::new(&self.hb)
            .render_template(server)
            .render_template(main)
            .finish()
    }
}
