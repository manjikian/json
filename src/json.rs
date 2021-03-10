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
    pub element: Box<dyn Value>,
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
    pub fn new() -> Box<Json> {
        return Box::new(Json { element: Box::new(JObject { value: HashMap::new() }) });
    }

    /// Returns a `String` representation of the `Json`
    ///
    /// # Examples
    /// ```
    /// use json::{Json, Value};
    ///
    /// let mut json = Json::new();
    /// json.element.insert("item".to_string(), Value::True);
    /// assert_eq!(json.to_string(), "{\n\t\"item\" : True\n}".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        self.element.to_string()
    }

    /// Replaces the `Value` in the `Json.element`
    ///
    /// # Examples
    /// ```
    /// use json::{Json, JTrue};
    ///
    /// let mut json = Json::new();
    /// json.replace_element(Box::new(JTrue));
    /// assert_eq!("True".to_string(), json.to_string());
    /// ```
    pub fn replace_element(&mut self, value: Box<dyn Value>) {
        self.element = value;
    }
}

/*
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
*/

fn array_to_string(vector: &Vec<Box<dyn Value>>) -> String {
    if vector.len() == 0 { return "[]".to_string(); }
    let mut result = String::new();
    result.push('[');
    for item in vector {
        result.push_str(item.as_ref().to_string().as_str());
        result.push_str(", ");
    }
    result.pop();
    result.pop();
    result.push(']');
    result
}

fn hashmap_to_string(hashmap: &HashMap<JString, Box<dyn Value>>) -> String {
    if hashmap.len() == 0 { return "{}".to_string(); }
    let mut result = String::new();
    result.push_str("{\n");
    for (k, v) in hashmap {
        result.push_str("\t\"");
        result.push_str(k.value.as_str());
        result.push_str("\" : ");
        result.push_str(v.as_ref().to_string().as_str());
        result.push_str(",\n")
    }
    result.pop();
    result.pop();
    result.push_str("\n}");
    result
}

/// A value can be a string, or a number, or true or false or null, or an object or an array.
/// These structures can be nested.
pub trait Value {
    fn to_string(&self) -> String;
}

pub struct JTrue;

impl Value for JTrue {
    fn to_string(&self) -> String {
        "True".to_string()
    }
}

pub struct JFalse;

impl Value for JFalse {
    fn to_string(&self) -> String {
        "False".to_string()
    }
}


pub struct JNull;

impl Value for JNull {
    fn to_string(&self) -> String {
        "Null".to_string()
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct JString {
    value: String,
}

impl Value for JString {
    fn to_string(&self) -> String {
        let mut str = "\"".to_string();
        str.push_str(self.value.as_str());
        str.push('\"');
        str
    }
}

impl Value for JNumber {
    fn to_string(&self) -> String {
        match self.value {
            Number::Integer(i) => { i.to_string() }
            Number::Float(f) => { f.to_string() }
        }
    }
}

pub struct JNumber {
    value: Number
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
    Integer(i64),
    Float(f64),
}

pub struct JArray {
    value: Vec<Box<dyn Value>>
}

impl Value for JArray {
    fn to_string(&self) -> String {
        array_to_string(&self.value)
    }
}

pub struct JObject {
    value: HashMap<JString, Box<dyn Value>>
}

impl Value for JObject {
    fn to_string(&self) -> String {
        hashmap_to_string(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `to_string()` method of `Value`
    #[test]
    fn to_string() {
        let null_value = JNull;
        assert_eq!(null_value.to_string(), "Null".to_string());
        let true_value = JTrue;
        assert_eq!(true_value.to_string(), "True".to_string());
        let false_value = JFalse;
        assert_eq!(false_value.to_string(), "False".to_string());
        let string_value = JString { value: String::from("abc") };
        assert_eq!(string_value.to_string(), "\"abc\"".to_string());
        let int_value = JNumber { value: Number::Integer(-32i64) };
        assert_eq!(int_value.to_string(), "-32".to_string());
        let float_value = JNumber { value: Number::Float(2.3) };
        assert_eq!(float_value.to_string(), "2.3".to_string());
        let array_value = JArray { value: vec![Box::new(JTrue), Box::new(JFalse)] };
        assert_eq!(array_value.to_string(), "[True, False]".to_string());
        let object_value = JObject { value: HashMap::new() };
        assert_eq!(object_value.to_string(), "{}".to_string());
    }


    #[test]
    fn hashmap() {
        let mut h: HashMap<JString, Box<dyn Value>> = HashMap::new();
        h.insert(JString { value: "bool_entry".to_string() }, Box::new(JFalse));
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

    #[test]
    fn array_test() {
        let mut a = JArray { value: Vec::new() };
        a.value.push(Box::new(JTrue));
        a.value.push(Box::new(JFalse));
        assert_eq!(a.value.len(), 2);
        assert_eq!(a.value[0].to_string(), "True");
        assert_eq!(a.value[1].to_string(), "False");
    }
}
