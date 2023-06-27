fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/gateway")?;
    tonic_build::compile_protos("../protos/common")?;
    Ok(())
}