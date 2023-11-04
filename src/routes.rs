use axum::headers::HeaderMap;
use axum::http::header;
use crate::{
    error::Result,
    ical::{convert_ical, get_ical},
};

pub async fn handler() -> Result<(HeaderMap, String)> {
    log::info!("handler called");
    let mut calender = get_ical().await?;

    convert_ical(&mut calender)?;

    let ev = &calender as &dyn ical::generator::Emitter;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/calender".parse().unwrap());
    log::info!("successfull remap");
    Ok((headers,ev.generate()))
}
