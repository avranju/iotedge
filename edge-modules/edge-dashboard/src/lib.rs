// Copyright (c) Microsoft. All rights reserved.

mod error;
mod settings;

#[cfg(windows)]
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::Error as ActixError;
use actix_web::*;
use edgelet_config::Settings as EdgeSettings;
use edgelet_docker::DockerConfig;
use futures::future::ok;
use futures::Future;
use structopt::StructOpt;

pub use error::Error;
use settings::Settings;

pub struct Context {
    pub edge_config: Result<EdgeSettings<DockerConfig>, Error>,
    pub settings: Settings,
}

impl Context {
    pub fn new() -> Self {
        let settings = Settings::from_args();
        let edge_config = get_config(settings.config_path.as_ref().map(String::as_str));

        Context {
            edge_config,
            settings,
        }
    }
}

pub struct Main {
    context: Arc<Context>,
}

impl Main {
    pub fn new(context: Context) -> Self {
        Main {
            context: Arc::new(context),
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let address = format!(
            "{}:{}",
            self.context.settings.host, self.context.settings.port
        );

        println!("Server listening at http://{}", address);

        let context = web::Data::new(self.context.clone());
        HttpServer::new(move || {
            App::new()
                .register_data(context.clone())
                .service(web::resource("/api/modules").route(web::get().to(get_modules)))
        })
        .bind(address)?
        .run()?;

        Ok(())
    }
}

fn get_modules(
    context: web::Data<Arc<Context>>,
) -> Box<dyn Future<Item = HttpResponse, Error = ActixError>> {
    let response = context
        .edge_config
        .as_ref()
        .map(|config| {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("{:?}", config.moby_runtime().uri()))
        })
        .unwrap_or_else(|err| {
            HttpResponse::ServiceUnavailable()
                .content_type("text/plain")
                .body(format!("{:?}", err))
        });

    Box::new(ok(response))
}

fn get_default_config_path() -> PathBuf {
    #[cfg(not(windows))]
    {
        Path::new("/etc/iotedge/config.yaml").to_owned()
    }

    #[cfg(windows)]
    {
        Path::new(
            env::var("CSIDL_COMMON_APPDATA")
                .or_else(|| env::var("ProgramData"))
                .unwrap_or("C:/ProgramData/iotedge/config.yaml"),
        )
        .to_owned()
    }
}

fn get_config(config_path: Option<&str>) -> Result<EdgeSettings<DockerConfig>, Error> {
    let config_path = config_path
        .map(|p| Path::new(p).to_owned())
        .unwrap_or_else(get_default_config_path);
    Ok(EdgeSettings::<DockerConfig>::new(Some(&config_path))?)
}
