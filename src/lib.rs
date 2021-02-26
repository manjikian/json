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

enum Value {
    Null,
    True,
    False,
    String(String),
    Number(Number),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

enum Number {
    Integer(isize),
    Float(f64),
}

struct Json {
    root: Value,
}


impl Json {
    fn new() -> Json {
        return Json { root: Value::Object(HashMap::new()) };
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
