use std::fmt::{Display, Formatter};

use axum::http::{Method, Uri};
use chrono::Utc;
use chrono_tz::Tz::Asia__Novosibirsk;

use crate::context::AuthTokenContext;
use crate::error::{ClientError, Error};

const TIME_FORMAT: &str = "%d.%m.%Y %H:%M:%S";

pub fn log_layer(layer: &str, message: &str) {
    println!(
        "{}: {layer:<20} {message}",
        Utc::now().with_timezone(&Asia__Novosibirsk)
            .format(TIME_FORMAT)
    );
}

pub async fn log_request(
    request_method: Method,
    uri: Uri,
    context: Option<&AuthTokenContext>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) {
    let request = Request {
        user_id: context.map(|context| context.user_id()),
        request_path: uri.to_string(),
        request_method: request_method.to_string(),
        service_error: service_error
            .map(|service_error| service_error.to_string()),
        client_error: client_error
            .map(|error| error.as_ref().to_string())
    };

    println!("{}", request);
}

struct Request {
    user_id: Option<u32>,

    request_path: String,
    request_method: String,

    service_error: Option<String>,
    client_error: Option<String>,
}

impl Display for Request {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let current_time = Utc::now()
            .with_timezone(&Asia__Novosibirsk)
            .format(TIME_FORMAT);

        writeln!(
            formatter,
            "{}: {} {};",
            current_time,
            self.request_path,
            self.request_method
        )?;

        if self.user_id.is_some() {
            writeln!(
                formatter,
                "User ID: {};",
                self.user_id.unwrap()
            )?
        }

        if self.service_error.is_some() {
            writeln!(
                formatter,
                "Error: {};\n\
                Client error: {}.",
                self.service_error.as_ref().unwrap(),
                self.client_error.as_ref().unwrap()
            )?;
        }

        writeln!(formatter)
    }
}