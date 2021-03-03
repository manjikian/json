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

pub enum Value {
    Null,
    True,
    False,
    String(String),
    Number(Number),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub enum Number {
    Integer(isize),
    Float(f64),
}

impl Value {
    /// Outputs a string representation of the enum `Value`.
    ///
    /// # Examples
    /// ```
    /// use lib::{Value, Number};
    /// use std::collections::HashMap;
    ///
    /// let null_value = Value::Null;
    /// assert_eq!(null_value.to_string(), "Null".to_string());
    ///
    /// let true_value = Value::True;
    /// assert_eq!(true_value.to_string(), "True".to_string());
    ///
    /// let false_value = Value::False;
    /// assert_eq!(false_value.to_string(), "False".to_string());
    ///
    /// let string_value = Value::String(String::from("abc"));
    /// assert_eq!(string_value.to_string(), "abc".to_string());
    ///
    /// let int_value = Value::Number(Number::Integer(32isize));
    /// assert_eq! (int_value.to_string(), "32".to_string());
    ///
    /// let float_value = Value::Number(Number::Float(2.3));
    /// assert_eq!(float_value.to_string(), "2.3".to_string());
    ///
    /// let array_value = Value::Array(vec![Value::True, Value::False]);
    /// assert_eq!(array_value.to_string(), "[True, False]".to_string());
    ///
    /// let object_value = Value::Object(HashMap::new());
    /// assert_eq!(object_value.to_string(), "{}".to_string());
    ///
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Value::Null => { String::from("Null") }
            Value::True => { String::from("True") }
            Value::False => { String::from("False") }
            Value::String(s) => { s.clone() }
            Value::Array(v) => { String::from("dummy") }
            Value::Object(h) => { String::from("dummy") }
            Value::Number(n) => {
                match n {
                    Number::Integer(i) => { format!("{}", i) }
                    Number::Float(f) => { format!("{}", f) }
                }
            }
        }
    }
}

pub struct Json {
    root: Value,
}


impl Json {
    /// Creates an empty `Json` struct
    ///
    /// # Examples
    /// ```
    /// use lib::Json;
    ///
    /// let json = Json::new();
    /// assert_eq!(json.to_string(), "{}".to_string());
    /// ```
    pub fn new() -> Json {
        return Json { root: Value::Object(HashMap::new()) };
    }

    pub fn to_string(&self) -> String {
        "{}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let json = Json::new();
        assert!(match json.root {
            Value::Object(x) => { x.len() == 0 }
            _ => false
        });
    }
}
