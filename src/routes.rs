use crate::{
    error::Result,
    ical::{convert_ical, get_ical},
};

pub async fn handler() -> Result<String> {
    log::info!("handler called");
    let mut calender = get_ical().await?;

    convert_ical(&mut calender)?;

    let ev = &calender as &dyn ical::generator::Emitter;

    log::info!("successfull remap");
    Ok(ev.generate())
}
