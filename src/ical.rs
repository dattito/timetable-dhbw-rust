use ical::{generator::IcalEvent, parser::ical::component::IcalCalendar, property::Property};

use crate::error::{self, WebError};

pub async fn get_ical() -> error::Result<IcalCalendar> {
    let response = reqwest::get(dotenvy_macro::dotenv!("ICS_URL")).await?;

    let text = response.text().await?;

    let ical_reader = ical::IcalParser::new(text.as_bytes()).collect::<Vec<_>>();

    let first_calender = ical_reader
        .get(0)
        .ok_or(WebError::InternalServerError)?
        .as_ref()
        .map_err(|_| WebError::InternalServerError)?
        .to_owned();

    Ok(first_calender)
}

pub fn convert_ical(calender: &mut IcalCalendar) -> error::Result<()> {
    calender.events.iter_mut().for_each(|event| {
        patch_address(event);
        patch_lecture(event);
    });
    Ok(())
}

fn patch_address(event: &mut IcalEvent) {
    let location_property = get_property_value(event, "LOCATION");
    let summary_property = get_property_value(event, "SUMMARY");

    if location_property.to_lowercase().contains("online")
        || summary_property.to_lowercase().contains("online")
    {
        let mut location = location_property.clone();
        if location.is_empty() {
            location = String::from("online")
        }

        set_property(
            event,
            "LOCATION",
            format!(
                "({})\n{}",
                location,
                get_property_value(event, "DESCRIPTION")
            ),
        );
    } else {
        let mut room_name = location_property;
        if !room_name.is_empty() {
            room_name = format!("{}, ", room_name)
        }

        set_property(
            event,
            "LOCATION",
            format!("{}Coblitzallee 1-9, 68163 Mannheim, Deutschland", room_name),
        );
    }

    if summary_property.to_lowercase().contains("online") {
        set_property(
            event,
            "SUMMARY",
            summary_property.replace("online", "").trim().to_string(),
        )
    }
}

fn patch_lecture(event: &mut IcalEvent) {
    let summary_value = get_property_value(event, "SUMMARY");
    let (lecturer, summary_without_lecturer) = split_lecturer_from_text(summary_value);
    set_property(event, "SUMMARY", summary_without_lecturer);
    set_property(
        event,
        "DESCRIPTION",
        format!(
            "{}\nDozent: {}",
            get_property_value(event, "DESCRIPTION"),
            lecturer
        ),
    )
}

fn split_lecturer_from_text(text: String) -> (String, String) {
    let mut words = text.split(' ');

    let lecturer_index = words.position(|x| {
        x.contains("Herr")
            || x.contains("Hr.")
            || x.contains("Frau")
            || x.contains("Fr.")
            || x.contains("Dr.")
            || x.contains("Prof")
    });

    if let Some(li) = lecturer_index {
        if words.clone().count() < li + 2 || li == 0 {
            (String::new(), text)
        } else {
            let collection = words.collect::<Vec<_>>();
            let first = &collection[li..li + 2].join("");
            let second = collection[..li].join(" ") + " " + &collection[li + 2..].join(" ");

            (String::from(first), second)
        }
    } else {
        (String::new(), text)
    }
}

fn set_property<'a>(event: &'a mut IcalEvent, name: &'a str, value: String) {
    let filtered = event
        .properties
        .iter_mut()
        .find(|x| x.name.to_lowercase() == name.to_string().to_lowercase());
    match filtered {
        None => event.properties.push(Property {
            name: name.to_string(),
            value: Some(value),
            ..Default::default()
        }),
        Some(property) => property.value = Some(value),
    }
}

// fn get_property<'a>(event: &'a mut IcalEvent, name: &'a str) -> Option<&'a mut Property> {
//     event
//         .properties
//         .iter_mut()
//         .find(|x| x.name.to_lowercase() == name.to_string().to_lowercase())
// }

fn get_property_value<'a>(event: &'a IcalEvent, name: &'a str) -> String {
    let property = event
        .properties
        .iter()
        .find(|x| x.name == *name.to_string());

    match property {
        Some(p) => p.value.clone().unwrap_or(String::new()),
        None => String::new(),
    }
}
