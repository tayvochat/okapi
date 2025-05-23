# Okapi

Okapi: [![Download](https://img.shields.io/crates/v/okapi)](https://crates.io/crates/okapi/)
[![API Docs](https://img.shields.io/badge/docs-okapi-blue)](https://docs.rs/okapi/latest/okapi/)

Rocket-Okapi: [![Download](https://img.shields.io/crates/v/rocket_okapi)](https://crates.io/crates/rocket_okapi)
[![API Docs](https://img.shields.io/badge/docs-rocket_okapi-blue)](https://docs.rs/rocket_okapi/latest/rocket_okapi/)

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Automated OpenAPI (AKA Swagger) document generation for Rust/Rocket projects.

Never have outdated documentation again.
Okapi will generate documentation for you while setting up the server.
It uses a combination of [Rust Doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments)
and programming logic to document your API.

The generated [OpenAPI][openapi_3.0.0] files can then be used by various programs to
visualize the documentation. Rocket-okapi currently includes [RapiDoc][rapidoc] and
[Swagger UI][swagger_ui], but others can be used too.

Supported OpenAPI Spec: [3.0.0][openapi_3.0.0]<br/>
Supported Rocket version (for `rocket_okapi`): [0.5.0-rc.1](https://crates.io/crates/rocket/0.5.0-rc.1)

Example of generated documentation using Okapi:

- DF Storyteller: [RapiDoc](https://docs.dfstoryteller.com/rapidoc/),
  [Swagger UI](https://docs.dfstoryteller.com/swagger-ui/)
- ...[^1]

[^1]: More examples will be added, please open an issue if you have a good example.

## Basic Usage

```rust
use rocket::{get, post, serde::json::Json};
use tayvo_rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[serde(default)]
    email: Option<String>,
}

// Add #[openapi] attribute to your routes
#[openapi]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

// You can tag your routes to group them together
#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

// You can skip routes that you don't want to include in the openapi doc
#[openapi(skip)]
#[get("/hidden")]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

pub fn make_rocket() -> rocket::Rocket {
    rocket::build()
        // openapi_get_routes![...] will host the openapi document at `openapi.json`
        .mount(
            "/",
            openapi_get_routes![get_user, create_user, hidden],
        )
        // You can optionally host swagger-ui too
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
```

## More examples

- [Json web API](examples/json-web-api): Simple example showing the basics of Okapi.
- [UUID](examples/uuid): Simple example showing basics, but using UUID's instead of
  normal `u32`/`u64` id's.
- [Custom Schema](examples/custom_schema): Shows how to add more/custom info to OpenAPI file
  and merge multiple modules into one OpenAPI file.
- [Secure Request Guard](examples/secure_request_guard): Shows how to implement authentication
  methods into the OpenAPI file.
  It shows: No authentication, API keys, HTTP Auth, OAuth2, OpenID and Cookies.
- [Special types](examples/special-types): Showing use of some more obscure types and there usage.
  (Still work in progress)

## FAQ

- **Q: Can I generate code from my OpenAPI file?**<br/>
  A: No, this crate only allows you to automatically generate the OpenAPI file from your code.
  There are other crates that (attempt to) do this.
  So:

  - Rust code (Rocket) --> OpenAPI == Okapi
  - OpenAPI --> Rust code != Okapi

- **Q: My (diesel) database does not implement `OpenApiFromRequest`.**<br/>
A: This is because the parameter does not show up in the path, query or body.
So this is considered a [Request Guard](https://rocket.rs/v0.5-rc/guide/requests/#request-guards).
There is a [derive macro](https://github.com/GREsau/okapi/blob/master/examples/secure_request_guard/src/no_auth.rs)
for this, but this does not work in combination with the `#[database("...")]` marco.
You can solve this my implementing it manually, like this:
<details>
    <summary>Implement `OpenApiFromRequest` for Diesel DB</summary>

```rust
use tayvo_rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use tayvo_rocket_okapi::gen::OpenApiGenerator;
use rocket_sync_db_pools::{diesel, database};

#[database("sqlite_logs")]
pub struct MyDB;

impl<'r> OpenApiFromRequest<'r> for MyDB {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> tayvo_rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
```

</details>

- **Q: ... does not implement `JsonSchema`?**<br/>
  A: The [`JsonSchema`](https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html) implementation
  is handled by [`Schemars`][schemars], make sure you enabled the right
  [feature flags](https://github.com/GREsau/schemars#optional-dependencies) for it.
  If it is still not implemented open an issue in the `Schemars` repo.

- **Q: Can I add custom data to my OpenAPI spec?**<br/>
  A: Yes, see the [Custom Schema](examples/custom_schema) example. Okapi also has build in functions
  if you want to merge the [`OpenAPI`](https://docs.rs/okapi/latest/okapi/openapi3/struct.OpenApi.html)
  objects manually.

- **Q: Can I use this with other web frameworks then Rocket?**<br/>
  A: Yes, but not there are no other implementations right now. But you can use the `Okapi` crate
  independently and use Serde to create the json or yaml file.

## Feature Flags

Okapi:

- `impl_json_schema`: Implements [`JsonSchema`](https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html)
  for [`Schemars`][schemars] and `Okapi` types themselves.
- `preserve_order`: Keep the order of struct fields in `Schema` and all parts of the
  `OpenAPI` documentation.

Rocket-Okapi:

- `preserve_order`: Keep the order of struct fields in `Schema` and all parts of the
  `OpenAPI` documentation.
- `swagger`: Enable [Swagger UI][swagger_ui] for rendering documentation.
- `rapidoc`: Enable [RapiDoc][rapidoc] for rendering documentation.
- `uuid`: Enable UUID support in Rocket and Schemars.
- `msgpack`: Enable [msgpack support for Rocket](https://docs.rs/rocket/0.5.0-rc.1/rocket/serde/msgpack/struct.MsgPack.html).
  (when same Rocket feature flag is used.)
- `secrets`: Enable [secrets support for Rocket](https://rocket.rs/v0.5-rc/guide/requests/#secret-key).
  (when same Rocket feature flag is used.)

Note that not all feature flags from [`Schemars`][schemars] are re-exported or enabled.
So if you have objects for which the `JsonSchema` trait is not implemented,
you might need to enable a [feature flag in `Schemars`](https://github.com/GREsau/schemars#optional-dependencies).
For an example see [the "uuid" example](examples/uuid/Cargo.toml). (Make sure crate versions match)

## How it works

This crate automatically generates an OpenAPI file when the Rocket server starts.
The way this is done is shortly described here.

The [`Schemars`][schemars] crate provides us with the schemas for all the different
structures and enums. Okapi does not implement any schemas directly, this is all handled by `Schemars`.

The `Okapi` crate just contains all the structures needed to create an OpenAPI file.
This crate does not contain any code for the creation of them, just the structure and code to merge
two [`OpenAPI`](https://docs.rs/okapi/latest/okapi/openapi3/struct.OpenApi.html) structured together.
This crate can be reused to create OpenAPI support in other web framework.

`Rocket-Okapi` crate contains all the code for generating the OpenAPI file and serve it once created.
This code is usually executed using macro's like: [`mount_endpoints_and_merged_docs!{...}`,
`openapi_get_routes![...]`, `openapi_get_routes_spec![...]` and `openapi_get_spec![...]`
](https://docs.rs/rocket_okapi/latest/rocket_okapi/#macros).

When the Rocket server is started (or wherever macro is placed) the OpenAPI file is generated once.
This file/structure is then stored in memory and will be served when requested.

The `Rocket-Okapi-codegen` crate contains code for
[derive macros](https://doc.rust-lang.org/book/ch19-06-macros.html).
`#[openapi]`, `rocket_okapi::openapi_spec![...]`, `rocket_okapi::openapi_routes![...]`
and `#[derive(OpenApiFromRequest)]` in our case.
This needs to be in a separate crate because of Rust restrictions.
Note: `derive` or `codegen` crates are usually a bit hard to work with then other crates.
So it is recommended to get some experience with how derive macros work before you
change things in here.

## TODO

- [ ] Tests
- [ ] Documentation
- [ ] Benchmark/optimise memory usage and allocations
  - Note to self: https://crates.io/crates/graphannis-malloc_size_of looks useful
- [x] Implement `OpenApiFrom___`/`OpenApiResponder` for more rocket/rocket-contrib types
- [x] Allow customizing openapi generation settings, e.g.
  - [x] custom json schema generation settings
  - [x] change path the document is hosted at

## License

This project is licensed under the [MIT license](LICENSE).

All contributions to this project will be similarly licensed.

[schemars]: https://github.com/GREsau/schemars
[openapi_3.0.0]: https://spec.openapis.org/oas/v3.0.0
[rapidoc]: https://mrin9.github.io/RapiDoc/
[swagger_ui]: https://swagger.io/tools/swagger-ui/
