use std::collections::HashMap;

use buildkit_frontend::{run_frontend, Bridge, Frontend, FrontendOutput};
use buildkit_llb::prelude::*;

use async_trait::async_trait;

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
        let root_img = Source::image("debian:latest");
        let mut last_output = (OutputIdx(0), root_img.output());
        for (k, v) in options.into_iter() {
            last_output.1 = exec::Command::run("echo")
                .args(&[&k, "=", &v.to_string()])
                .mount(Mount::Layer(last_output.0, last_output.1, "/"))
                .ref_counted()
                .output(0);
        }
        let output = bridge.solve(Terminal::with(last_output.1)).await?;
        Ok(FrontendOutput::with_ref(output))
    }
}
