use component::Interface;
use component::sub::com::Component;

fn main() -> anyhow::Result<()> {
    let component = Component;

    env_logger::init();
    component.interface()?;

    Ok(())
}
