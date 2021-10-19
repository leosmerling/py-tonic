fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[::pyo3_prost::pyclass_for_prost_struct]")
        .compile(&["proto/hw.proto"], &["proto/"])?;
    Ok(())
}
