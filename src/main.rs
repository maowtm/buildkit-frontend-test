use std::collections::HashMap;

use buildkit_frontend::{
    oci::{Architecture, ImageConfig, OperatingSystem},
    run_frontend, Bridge, Frontend, FrontendOutput,
};
use buildkit_llb::prelude::*;

use async_trait::async_trait;

use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_frontend(MyFrontend).await?;
    Ok(())
}

struct MyFrontend;

#[async_trait]
impl Frontend<HashMap<String, serde_json::Value>> for MyFrontend {
    async fn run(
        self,
        bridge: Bridge,
        options: HashMap<String, serde_json::Value>,
    ) -> Result<FrontendOutput, failure::Error> {
        let root_img = Source::image("europe-docker.pkg.dev/maowtm/modus-test/cat-frontend");
        let mut local_source = Source::local("context").custom_name("Reading local Dockerfile");
        local_source = local_source.add_exclude_pattern("target");
        let local_output = local_source.output();
        let local_ref = bridge.solve(Terminal::with(local_output)).await?;
        let input_filename = match options.get(&"filename".to_owned()).unwrap() {
            serde_json::Value::String(s) => s,
            _ => panic!("Expected option \"filename\" to be a string"),
        };
        let input = bridge.read_file(&local_ref, input_filename, None).await?;
        let mut last_output = root_img.output();
        let mut options = options.into_iter().collect::<Vec<_>>();
        options.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (k, v) in options {
            last_output = exec::Command::run("echo")
                .args(&[&k, "=", &v.to_string()])
                .mount(Mount::Layer(OutputIdx(0), last_output, "/"))
                .ref_counted()
                .output(0);
        }
        // last_output = exec::Command::run("echo")
        //     .args(&["input", ":", &serde_json::to_string(std::str::from_utf8(&input).unwrap()).unwrap()])
        //     .mount(Mount::Layer(OutputIdx(0), last_output, "/"))
        //     .ref_counted()
        //     .output(0);
        last_output = exec::Command::run("echo")
            .args(&["random", "=", &rand::thread_rng().gen::<u32>().to_string()])
            .mount(Mount::Layer(OutputIdx(0), last_output, "/"))
            .ref_counted()
            .output(0);
        let output = bridge.solve(Terminal::with(last_output)).await?;
        Ok(FrontendOutput::with_spec_and_ref(
            buildkit_frontend::oci::ImageSpecification {
                created: None,
                author: None,
                architecture: Architecture::Amd64,
                os: OperatingSystem::Linux,
                config: Some(ImageConfig {
                    cmd: None,
                    entrypoint: Some(vec!["echo".to_owned(), "test".to_owned()]),
                    env: None,
                    exposed_ports: None,
                    labels: None,
                    user: None,
                    working_dir: None,
                    stop_signal: None,
                    volumes: None,
                }),
                rootfs: None,
                history: None,
            },
            output,
        ))
    }
}
