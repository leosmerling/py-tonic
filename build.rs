fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("proto/")
        .build_server(false)
        .build_client(false)
        //.type_attribute(".", "#[::pyo3::prelude::pyclass]")
        //.type_attribute(".", "#[::pyo3_prost::pyclass_for_prost_struct]")
        .compile(&["proto/hw.proto"], &["proto/"])?;
    Ok(())
}
