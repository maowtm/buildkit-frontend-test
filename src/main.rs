use std::{env, time::Duration};

use buildkit_frontend::{run_frontend, Bridge, Frontend, FrontendOutput};
use buildkit_llb::prelude::*;

use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut infostr = String::new();
    use std::fmt::Write;
    write!(&mut infostr, "Args:\n")?;
    for arg in env::args_os() {
        write!(&mut infostr, "  {}\n", arg.to_string_lossy())?;
    }

    write!(&mut infostr, "Envs:\n")?;
    for (key, value) in env::vars_os() {
        write!(
            &mut infostr,
            "  {} = {}\n",
            key.to_string_lossy(),
            value.to_string_lossy()
        )?;
    }

    run_frontend(MyFrontend(infostr)).await?;

    Ok(())
}

struct MyFrontend(String);

#[async_trait]
impl Frontend for MyFrontend {
    async fn run(
        self,
        bridge: Bridge,
        options: buildkit_frontend::Options,
    ) -> Result<FrontendOutput, failure::Error> {
        let root_img = Source::image("debian:latest");
        let terminal = Terminal::with(
            exec::Command::run("echo")
                .args(&[&self.0])
                .mount(Mount::Layer(OutputIdx(0), root_img.output(), "/"))
                .ref_counted()
                .output(0),
        );
        let output = bridge.solve(terminal).await?;
        Ok(FrontendOutput::with_ref(output))
    }
}
