use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "proto/gtfs-realtime-OVapi.proto",
            "proto/gtfs-realtime.proto",
        ],
        &["proto/"],
    )?;
    Ok(())
}
