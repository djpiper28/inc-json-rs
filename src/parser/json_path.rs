/// NOT TO BE CONFUSED WITH JSON PATH
use std::string::String;
use std::vec::Vec;

pub enum JsonPathElement {
    Array,
    Object(String),
}

pub type JsonPath = Vec<JsonPathElement>;
