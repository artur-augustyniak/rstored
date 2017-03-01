#[macro_use]
extern crate rustless;
extern crate iron;
extern crate time;
extern crate url;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate valico;

use rustless::batteries::swagger;
use rustless::{Nesting};
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use valico::json_dsl;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn main() {
    let mut app = rustless::Application::new(rustless::Api::build(|api| {
        api.prefix("api");
        api.version("v1", rustless::Versioning::Path);
        api.mount(swagger::create_api(""));

        api.get("legacy", |endpoint| {
            endpoint.summary("Legacy stored call");
            endpoint.desc("Legacy stored call");
            endpoint.handle(|client, _| {
                let resp = json!([
                      {
                        "mount_point": "/",
                        "free_space": 43
                      },
                      {
                        "mount_point": "/boot",
                        "free_space": 68
                      },
                      {
                        "mount_point": "/var",
                        "free_space": 60
                      },
                      {
                        "mount_point": "/opt",
                        "free_space": 42
                      },
                      {
                        "mount_point": "/Backup",
                        "free_space": 24
                      }]
                    );
                client.text(resp.to_string())
            })
        });

        api.namespace("probes", |probes_ns| {
            probes_ns.get("df", |endpoint| {
                endpoint.summary("summary");
                endpoint.desc("description");
                endpoint.handle(|client, _| {
                    let resp = json!(
                        {
                             "count": 4,
                             "next": "http://127.0.0.1:4000/api/v1/probes/df?page=2",
                             "previous": null,
                              "results": [
                                    {
                                        "mount_point": "/",
                                        "percent_left": "20",
                                        "url": "http://127.0.0.1:4000/api/v1/probes/df/%2F"
                                    },
                                    {
                                        "mount_point": "/home",
                                        "percent_left": "10",
                                        "url": "http://127.0.0.1:4000/api/v1/probes/df/%2Fhome"
                                    }
                              ]
                        }
                    );
                    client.text(resp.to_string())
                })
            });
            probes_ns.get("df/:name", |endpoint| {
                endpoint.summary("summary");
                endpoint.desc("description");
                endpoint.params(|params| {
                    params.opt_typed("name", json_dsl::string());
                });
                endpoint.handle(|client, params| {
                    let slash = String::from(format!("\"{}\"", "/"));
                    let home = String::from(format!("\"{}\"", "/home"));

                    let in_param = params.find("name");
                    let resp = match in_param {
                        Some(ref p) if slash == p.to_string() => json!({
                                "mount_point": "/",
                                "percent_left": "20",
                                "url": "http://127.0.0.1:4000/api/v1/probes/df/%2F"}),
                        Some(ref p) if home == p.to_string() => json!({
                                "mount_point": "/home",
                                "percent_left": "10",
                                "url": "http://127.0.0.1:4000/api/v1/probes/df/%2Fhome"}),
                        _ =>

                            json!({
                                "error" : in_param.unwrap().to_string()})
                    };
                    client.text(resp.to_string())
                })
            });
            probes_ns.get("mem", |endpoint| {
                endpoint.summary("summary");
                endpoint.desc("description");
                endpoint.handle(|client, _params| {
                    client.text("Everything is OK".to_string())
                })
            });
        })
    }));

    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "rstored API".to_string(),
            description: Some("Server monitoring API".to_string()),
            contact: Some(swagger::Contact {
                name: "Artur Augustyniak".to_string(),
                url: Some("http://aaugustyniak.pl/".to_string()),
                ..std::default::Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..std::default::Default::default()
        },
        ..std::default::Default::default()
    });

    let mut chain = iron::Chain::new(app);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    println!("Server running... http://127.0.0.1:4000/api/v1/");
    iron::Iron::new(chain).http("0.0.0.0:4000").unwrap();
}
