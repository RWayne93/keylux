//! Read-only receiver configuration diagnostics.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kb = aula_protocol::dongle::open(&[])?;
    let a = kb.read_onboard_config()?;
    let b = kb.read_onboard_config()?;
    let bytes = a.bytes();
    let mode = u16::from_be_bytes([bytes[9], bytes[10]]);
    println!("Two validated reads agree: {}", bytes == b.bytes());
    println!("Active mode: {mode} (0x{mode:04x})");
    println!("Mode header: {:02x?}", &bytes[8..14]);
    println!("Lighting table: {:02x?}", &bytes[56..104]);
    println!("Decoded settings: {:?}", a.active());
    Ok(())
}
