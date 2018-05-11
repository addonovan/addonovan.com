use mwf;
use mwf::{RequestHandler, View, RouteMap};

use config::CONFIG;
use decs::Processor;

pub struct ServiceController
{
    processor: Processor,
    services: Vec<&'static str>,
}

impl ServiceController
{
    pub fn new() -> Self
    {
        ServiceController {
            processor: Processor::new(),
            services: vec!["website", "factorio", "minecraft"],
        }
    }

    fn status(&self, name: &str) -> bool
    {
        use std::process::{Command, Child, Stdio};

        let systemctl = Command::new("systemctl")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to invoke systemctl")
            .stdout
            .unwrap();

        let grep = Command::new("grep")
            .arg(name)
            .stdin(systemctl)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to invoke grep")
            .stdout
            .unwrap();

        let awk = Command::new("awk")
            .arg("{print $4}")
            .stdin(grep)
            .output()
            .expect("Failed to invoke awk")
            .stdout;

        String::from_utf8(awk)
            .map(|it| it.trim() == "running")
            .unwrap_or(false)
    }
}

impl RequestHandler for ServiceController
{
    fn handle(&self, route_map: RouteMap) -> mwf::Result<View>
    {
        let output = self.services
            .iter()
            .map(|it| format!("{} = {}", it, self.status(it)))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(View::raw(output))
    }
}

