// Copyright © 2021 Hovig Manjikian
//
// This file is part of mmap_ci.
//
// json is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// json is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with json.  If not, see <https://www.gnu.org/licenses/>.


use std::collections::HashMap;

pub struct Json {
    pub root: Value,
}

impl Json {
    /// Creates a `Json` that contains an empty `Value::Object(Hashmap<String, Value>)`
    ///
    /// # Examples
    /// ```
    /// use json::Json;
    ///
    /// let json = Json::new();
    /// assert_eq!(json.to_string(), "{}".to_string());
    /// ```
    pub fn new() -> Json {
        return Json { root: Value::Object(HashMap::new()) };
    }

    /// Returns a `String` representation of the `Json`
    ///
    /// # Examples
    /// ```
    /// use json::{Json, Value};
    ///
    /// let mut json = Json::new();
    /// json.root.insert("item".to_string(), Value::True);
    /// assert_eq!(json.to_string(), "{\n\t\"item\" : True\n}".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        self.root.to_string()
    }
}

/// A value can be a string, or a number, or true or false or null, or an object or an array.
/// These structures can be nested.
///
/// # Examples
/// ```
/// use json::{Value, Number};
/// use std::collections::HashMap;
///
/// let null_value = Value::Null;
/// let true_value = Value::True;
/// let false_value = Value::False;
/// let string_value = Value::String("abs".to_string());
/// let number_value = Value::Number(Number::Integer(0));
/// let array_value = Value::Array(Vec::new());
/// let object_value = Value::Object(HashMap::new());
/// ```
pub enum Value {
    Null,
    True,
    False,
    String(String),
    Number(Number),
    Array(Vec<Value>),
    // Would have been much nicer if the key was Value::String
    Object(HashMap<String, Value>),
}

/// A number is either an integer or a floating point
///
/// ```
/// use json::Number;
///
/// let integer_number = Number::Integer(-3isize);
/// let float_number = Number::Float(2.5f64);
/// ```
pub enum Number {
    Integer(isize),
    Float(f64),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Null => { String::from("Null") }
            Value::True => { String::from("True") }
            Value::False => { String::from("False") }
            Value::String(s) => {
                let mut str = "\"".to_string();
                str.push_str(&s.clone());
                str.push('\"');
                str
            }
            Value::Array(v) => { array_to_string(v) }
            Value::Object(h) => { hashmap_to_string(h) }
            Value::Number(n) => {
                match n {
                    Number::Integer(i) => { format!("{}", i) }
                    Number::Float(f) => { format!("{}", f) }
                }
            }
        }
    }
}


fn array_to_string(vector: &Vec<Value>) -> String {
    if vector.len() == 0 { return "[]".to_string(); }
    let mut result = String::new();
    result.push('[');
    for item in vector {
        result.push_str(item.to_string().as_str());
        result.push_str(", ");
    }
    result.pop();
    result.pop();
    result.push(']');
    result
}

fn hashmap_to_string(hashmap: &HashMap<String, Value>) -> String {
    if hashmap.len() == 0 { return "{}".to_string(); }
    let mut result = String::new();
    result.push_str("{\n");
    for (k, v) in hashmap {
        result.push_str("\t\"");
        result.push_str(k.as_str());
        result.push_str("\" : ");
        result.push_str(v.to_string().as_str());
        result.push_str(",\n")
    }
    result.pop();
    result.pop();
    result.push_str("\n}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `to_string()` method of `Value`
    #[test]
    fn to_string() {
        let null_value = Value::Null;
        assert_eq!(null_value.to_string(), "Null".to_string());
        let true_value = Value::True;
        assert_eq!(true_value.to_string(), "True".to_string());
        let false_value = Value::False;
        assert_eq!(false_value.to_string(), "False".to_string());
        let string_value = Value::String(String::from("abc"));
        assert_eq!(string_value.to_string(), "\"abc\"".to_string());
        let int_value = Value::Number(Number::Integer(32isize));
        assert_eq!(int_value.to_string(), "32".to_string());
        let float_value = Value::Number(Number::Float(2.3));
        assert_eq!(float_value.to_string(), "2.3".to_string());
        let array_value = Value::Array(vec![Value::True, Value::False]);
        assert_eq!(array_value.to_string(), "[True, False]".to_string());
        let object_value = Value::Object(HashMap::new());
        assert_eq!(object_value.to_string(), "{}".to_string());
    }


    #[test]
    fn hashmap() {
        let mut h = HashMap::new();
        h.insert("bool_entry".to_string(), Value::False);
        let expected = r#"{
	"bool_entry" : False
}"#.to_string();
        assert_eq!(hashmap_to_string(&h), expected);
    }

    #[test]
    fn js() {
        let json = Json::new();
        assert_eq!(json.to_string(), "{}".to_string());
    }
}
