# ini_file_parser grammar

This documentation explains the rules and structure of the ini parser grammar used in this project.

## Rules

### WHITESPACE
This rule matches whitespace characters (space, tab).

```pest
WHITESPACE = _{ " " | "\t" }
```

### file
Rule that represent ini file, and match any count of sections and comments in any order with any count of newline between them.

```pest
file = { SOI ~ (NEWLINE* ~ (comment | section) ~ NEWLINE*)* ~ EOI }
```

### section
Rule that represent one section. It should match name in square brackets, new line after it, and any count of comments and keyvalue pairs in any order with any count of whitespace between them.

```pest
{"[" ~ name ~ "]" ~ NEWLINE+ ~ (NEWLINE* ~ (comment | pair))* ~ NEWLINE+}
```

### name
Rule that match section name, section name contain only uppercase symbols

```
@{ UPPERCASE_LETTER+ }
```

### pair
Rule that match key value pair separated by '='.

```
{ key ~ "=" ~ value }
```

#### Example URL:
```
https://www.example.com:8080/path/to/resource?key=value#section1
```

```pest
url = { SOI ~ protocol ~ "://" ~ (subdomain ~ ".")? ~ domain ~ port? ~ path? ~ query? ~ fragment? ~ EOI }
```