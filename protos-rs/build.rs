use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OUT_DIR", "./pkg/src/generated/");
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &[
                "../protos/gateway/assettransfer.proto",
            ],
            &[
                "../protos/",
            ],
        )?;
    Ok(())
}
