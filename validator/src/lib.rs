extern crate valico;
extern crate serde_json;
#[macro_use] extern crate error_chain;

use serde_json::{Value};
use valico::json_schema;
pub use valico::common::error::ValicoError;

pub mod errors;

use errors::{Result, ErrorKind};

pub fn get_schema(api_version: &str) -> Result<Value> {
    let schema = match api_version {
        "0.13" => include_str!("../schema/13.json"),
        "0.12" => include_str!("../schema/12.json"),
        "0.11" => include_str!("../schema/11.json"),
        "0.9" => include_str!("../schema/9.json"),
        "0.8" => include_str!("../schema/8.json"),
        _ => return Err(ErrorKind::WrongApiVersion(api_version.into()).into())
    }.parse().expect(&format!("Parsing of schema for {} failed", api_version));
    Ok(schema)
}

pub fn validate_spaceapi_json(json: &str) -> Result<json_schema::ValidationState> {
    let json_value: Value = json.parse()?;

    let json_obj = json_value.as_object().ok_or("Parameter is not a JSON object")?;

    let version = json_obj.get("api").ok_or("api field missing")?
        .as_str().ok_or("api value not a string")?;

    let json_schema: Value = get_schema(version)?;
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_schema, false).unwrap();
    let validation_result = schema.validate(&json_value);

    Ok(validation_result)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_coredump() {
        let obj = include_str!("../coredump.json");
        let validated = validate_spaceapi_json(obj);
        println!("{:?}", validated);
    }

    macro_rules! test_schema {
        ( $x:ident, $y:expr ) => {
            #[test]
            fn $x() {
                let schema = get_schema($y);
                assert!(schema.is_ok());
            }
        };
    }

    test_schema!(test_schema_0_8, "0.8");
    test_schema!(test_schema_0_9, "0.9");
    test_schema!(test_schema_0_11, "0.11");
    test_schema!(test_schema_0_12, "0.12");
    test_schema!(test_schema_0_13, "0.13");
}
