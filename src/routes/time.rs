use serde_derive::Serialize;
use serde_json::json;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use super::RouteHandler;
use super::super::http_request::HttpRequest;
use super::super::http_response::{Status, HttpResponse};

#[derive(Serialize)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
    unixtime: i64
}

impl Time {
    pub fn new() -> Self {
        let current_time: DateTime<Utc> = Utc::now();
        Time {
            hour: current_time.hour(),
            minute: current_time.minute(),
            second: current_time.second(),
            unixtime: current_time.timestamp()
        }
    }

    pub fn parse_iso(time_str: &str) -> Result<Self, chrono::format::ParseError> {
        let parsed_time = DateTime::parse_from_rfc3339(time_str)?;
        Ok(Time {
            hour: parsed_time.hour(),
            minute: parsed_time.minute(),
            second: parsed_time.second(),
            unixtime: parsed_time.timestamp()
        })
    }
}

impl RouteHandler for Time {
    fn handle_route(&self, request: &HttpRequest, response: &mut HttpResponse) -> Status {
        match request.uri.as_str() {
            "/api/unixtime/" => {
                let serialized = serde_json::to_string(&self).unwrap();
                response.send(Status::Ok, serialized)
            },
            "/api/parsetime" => {
                match request.query_params.get("iso") {
                    Some(date) => {
                        let parsed_time = Self::parse_iso(&date).unwrap();
                        let serialized = json!({
                            "hour": parsed_time.hour,
                            "minute": parsed_time.minute,
                            "second": parsed_time.second
                        }).to_string();
                        
                        response.send(Status::Ok, serialized)
                    },
                    _ => response.send(Status::NoContent, "Error: ISO date string was not provided".to_string())
                }
            }
            _ => response.send(Status::BadRequest, "".to_string())
        }
        
    }
}
