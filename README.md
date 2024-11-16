# ini_file_parser

This project is a Rust-based parser for INI configuration files. INI files are a simple, human-readable format for storing configuration data, where data is organized into sections, each containing key-value pairs.

## Overview

The parser reads an INI file and parses its contents into a structured format. The output is a `Config` structure, which contains a collection of `Section` structs, each holding key-value pairs. This parser is useful for reading and manipulating configuration files commonly used in applications, games, and system settings.

## Parsing Process

The parsing process follows these steps:

1. **File Parsing**: The parser first reads the entire content of the INI file. The file is divided into sections, which are enclosed in square brackets (`[section_name]`).
2. **Section Parsing**: For each section, the parser extracts the section name and the key-value pairs defined within that section.
3. **Key-Value Pair Parsing**: Each key-value pair is parsed by splitting the string at the equals sign (`=`), with the key on the left and the value on the right.
4. **Comment Handling**: Comments in the INI file, which start with a semicolon (`;`), are ignored during parsing.

The result of the parsing is stored in a `Config` structure, which contains multiple `Section` structs. Each `Section` contains a collection of `KeyValue` pairs.

## Usage

To use the INI parser, simply provide the content of the INI file as a string to the `parse_ini` function. The function will return a `Config` object, which can be further used to access the parsed sections and key-value pairs.

## Grammar

```pest
WHITESPACE = _{ " " | "\t" }

file = { SOI ~ (NEWLINE* ~ (comment | section) ~ NEWLINE*)* ~ EOI }

section = {"[" ~ name ~ "]" ~ NEWLINE+ ~ (NEWLINE* ~ (comment | pair))* ~ NEWLINE+}
name = @{ UPPERCASE_LETTER+ }

pair = { key ~ "=" ~ (array_value | value) }
key = @{ (ALPHABETIC)+ }
value = @{ (!("," | WHITESPACE | NEWLINE | "[" | "]") ~ ANY)+ }
array_value = { "[" ~ WHITESPACE* ~ value ~ (WHITESPACE* ~ "," ~ WHITESPACE* ~ value)* ~ WHITESPACE* ~ "]" }
comment = { ";" ~ (!NEWLINE ~ ANY)* }
```

### Example

```rust
let ini_content = r#"
; This is a comment
[section1]
key1=value1
key2=value2

[section2]
keyA=valueA
"#;

let config = parse_ini(ini_content)?;
let first = config.get_value("key1") // result is Some("value1")
let second = config.get_value("aaaaaa") // result is None
let third = config.get_value_in_section("section1", "key2") // result is Some("value2")
```

### Documentation and crate:

Documentation: https://docs.rs/ini_file_parser/latest/ini_file_parser/index.html

Crate: https://crates.io/crates/ini_file_parser