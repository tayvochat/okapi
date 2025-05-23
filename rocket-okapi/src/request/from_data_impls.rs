use super::OpenApiFromData;
use crate::gen::OpenApiGenerator;
use tayvo_okapi::{
    openapi3::{MediaType, RequestBody},
    Map,
};
use rocket::data::Data;
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{borrow::Cow, result::Result as StdResult};

type Result = crate::Result<RequestBody>;

macro_rules! fn_request_body {
    ($gen:ident, $ty:path, $mime_type:expr) => {{
        let schema = $gen.json_schema::<$ty>();
        Ok(RequestBody {
            content: {
                let mut map = Map::new();
                map.insert(
                    $mime_type.to_owned(),
                    MediaType {
                        schema: Some(schema),
                        ..MediaType::default()
                    },
                );
                map
            },
            required: true,
            ..tayvo_okapi::openapi3::RequestBody::default()
        })
    }};
}

impl<'r> OpenApiFromData<'r> for String {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, String, "application/octet-stream")
    }
}

impl<'r> OpenApiFromData<'r> for &'r str {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, str, "application/octet-stream")
    }
}

impl<'r> OpenApiFromData<'r> for Cow<'r, str> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, str, "application/octet-stream")
    }
}

impl<'r> OpenApiFromData<'r> for Vec<u8> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, Vec<u8>, "application/octet-stream")
    }
}

impl<'r> OpenApiFromData<'r> for &'r [u8] {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}

impl<'r, T: OpenApiFromData<'r> + 'r> OpenApiFromData<'r> for StdResult<T, T::Error> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        T::request_body(gen)
    }
}

impl<'r, T: OpenApiFromData<'r>> OpenApiFromData<'r> for Option<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Ok(RequestBody {
            required: false,
            ..T::request_body(gen)?
        })
    }
}

// Waiting for https://github.com/GREsau/schemars/issues/103
impl<'r> OpenApiFromData<'r> for rocket::fs::TempFile<'r> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<rocket::fs::TempFile<'r>> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        rocket::fs::TempFile::request_body(gen)
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<Cow<'r, str>> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, str, "application/octet-stream")
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<&'r str> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, str, "application/octet-stream")
    }
}
// See: https://github.com/GREsau/schemars/issues/103
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<&'r rocket::http::RawStr> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        <&'r rocket::http::RawStr>::request_body(gen)
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<&'r [u8]> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<String> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        String::request_body(gen)
    }
}
impl<'r> OpenApiFromData<'r> for rocket::data::Capped<Vec<u8>> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}

// See: https://github.com/GREsau/schemars/issues/103
impl<'r> OpenApiFromData<'r> for &'r rocket::http::RawStr {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}

impl<'r> OpenApiFromData<'r> for Data<'r> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Vec::<u8>::request_body(gen)
    }
}

// `OpenApiFromForm` is correct, not a mistake, as Rocket requires `FromForm`.
impl<'r, T: JsonSchema + super::OpenApiFromForm<'r>> OpenApiFromData<'r> for rocket::form::Form<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, T, "application/octet-stream")
    }
}

impl<'r, T: JsonSchema + Deserialize<'r>> OpenApiFromData<'r> for Json<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, T, "application/json")
    }
}

#[cfg(feature = "msgpack")]
impl<'r, T: JsonSchema + Deserialize<'r>> OpenApiFromData<'r>
    for rocket::serde::msgpack::MsgPack<T>
{
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        fn_request_body!(gen, T, "application/msgpack")
    }
}
