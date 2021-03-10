// Copyright © 2021 Hovig Manjikian
//
// This file is part of json.
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
use crate::value::*;

/// The JSON structure.
///
/// # Examples
/// ```
/// use json::json::Json;
///
/// let j = Json::new();
/// ```
pub struct Json {
    pub element: Box<dyn JsonValue>,
}

impl Json {
    /// Creates a `Json` that contains an empty `Value::Object(Hashmap<String, Value>)`
    ///
    /// # Examples
    /// ```
    /// use json::json::Json;
    ///
    /// let json = Json::new();
    /// assert_eq!(json.to_string(), "{}".to_string());
    /// ```
    pub fn new() -> Json {
        return Json { element: Box::new(JObject { value: HashMap::new() }) };
    }

    /// Returns a `String` representation of the `Json`
    ///
    /// # Examples
    /// ```
    /// use json::json::Json;
    /// use json::value::{JTrue, JString};
    ///
    /// let mut json = Json::new();
    /// json.element.value.insert(JString{value: "item".to_string()}, JTrue);
    /// assert_eq!(json.to_string(), "{\n\t\"item\" : True\n}".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        self.element.to_string()
    }

    /// Replaces the `Value` in the `Json.element`
    ///
    /// # Examples
    /// ```
    /// use json::json::Json;
    /// use json::value::JTrue;
    ///
    /// let mut json = Json::new();
    /// json.replace_element(Box::new(JTrue));
    /// assert_eq!("True".to_string(), json.to_string());
    /// ```
    pub fn replace_element(&mut self, value: Box<dyn JsonValue>) {
        self.element = value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js() {
        let json = Json::new();
        assert_eq!(json.to_string(), "{}".to_string());
    }

    #[test]
    fn replace_element_test() {
        let mut json = Json::new();
        assert_eq!(json.to_string(), "{}".to_string());
        json.replace_element(Box::new(JTrue));
        assert_eq!(json.to_string(), "True".to_string());
    }
}
