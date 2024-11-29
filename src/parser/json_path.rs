use super::lexer::tokens::{number_token::NumberToken, string_token::StringToken};
use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonPrimitive {
    String(StringToken),
    Number(NumberToken),
    Boolean(bool),
    Null,
}

pub type PrimitiveConsumer = fn(primitive: JsonPrimitive);

#[derive(Clone)]
pub enum UnknownConsumer {
    PrimitiveConsumer(PrimitiveConsumer),
    ObjectConsumer(ObjectConsumer),
}

#[derive(Clone)]
pub struct ObjectConsumer {
    /// Called when .key is a primitive
    primitive_consumers: HashMap<String, PrimitiveConsumer>,
    /// Called when .key is an object
    object_consumers: HashMap<String, ObjectConsumer>,
    /// Called when .key is an array, and for each member of the array
    array_consumers: HashMap<String, UnknownConsumer>,
}

impl ObjectConsumer {
    pub fn new() -> Self {
        return Self {
            primitive_consumers: HashMap::new(),
            object_consumers: HashMap::new(),
            array_consumers: HashMap::new(),
        };
    }

    pub fn primitive(self: &mut Self, key: String, consumer: PrimitiveConsumer) -> &mut Self {
        self.primitive_consumers.insert(key, consumer);
        return self;
    }

    pub fn object(self: &mut Self, key: String, consumer: &ObjectConsumer) -> &mut Self {
        self.object_consumers.insert(key, consumer.clone());
        return self;
    }

    pub fn array(self: &mut Self, key: String, consumer: UnknownConsumer) -> &mut Self {
        self.array_consumers.insert(key, consumer);
        return self;
    }
}

#[cfg(test)]
mod test_string_token {
    use super::*;

    fn example_primitive_consumer(primitive: JsonPrimitive) {
        match primitive {
            JsonPrimitive::String(x) => println!("{}", x.as_string()),
            JsonPrimitive::Number(x) => match x {
                NumberToken::Integer(i) => println!("{}", i),
                NumberToken::Float(f) => println!("{}", f),
            },
            JsonPrimitive::Boolean(x) => println!("{}", x),
            JsonPrimitive::Null => println!("null"),
        }
    }

    #[test]
    fn example_json_object_consumer() {
        ObjectConsumer::new()
            .primitive("id".to_string(), example_primitive_consumer)
            .primitive("created".to_string(), |x| match x {
                JsonPrimitive::String(x) => {
                    println!("Do something with the date {}", x.as_string())
                }
                _ => panic!("Oh no!"),
            })
            .object(
                "owner".to_string(),
                ObjectConsumer::new()
                    .primitive("id".to_string(), example_primitive_consumer)
                    .primitive("user_name".to_string(), example_primitive_consumer),
            );
    }
}
