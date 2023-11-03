use crate::{
    error::Result,
    ical::{convert_ical, get_ical},
};

pub async fn handler() -> Result<String> {
    let mut calender = get_ical().await?;

    convert_ical(&mut calender)?;

    let ev = &calender as &dyn ical::generator::Emitter;

    Ok(ev.generate())
}
