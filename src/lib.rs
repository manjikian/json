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

//! json
//!
//! GNU GPL licensed
//!
//! json is a library for parsing JSON structures from string format to a Rust
//! structure and vice versa. The API is minimalistic and simple. The solution
//! is thoroughly tested and guarantees good reliability. The implemented parser
//! follows the grammar defined in [ECMA-404] standard.
//!
//! [ECMA-404]: https://www.ecma-international.org/publications-and-standards/standards/ecma-404/

/// The module that implements the Json structure
pub mod json;

/// The module that implements the json values. A json value can be a string, or
/// a number, or true or false or null, or an object or an array. These
/// structures can be nested.
pub mod value;
