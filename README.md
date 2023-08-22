[![.github/workflows/rust.yml](https://github.com/saizo80/rtoml/actions/workflows/rust.yml/badge.svg)](https://github.com/saizo80/rtoml/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/saizo80/rtoml/status.svg)](https://deps.rs/repo/github/saizo80/rtoml)
# RTOML

command line toml file parser written in rust

## Usage

can be called on the command line after installation with `rtoml`

the first positional argument should be a file and the second should be the key in dot notation

### Examples

if the toml file has the following content:

```toml
value1 = "foo"

array_value = [
    "1",
    "2",
    "3",
]

[table]
value2 = "bar"

["table with spaces"]
"value with spaces" = "space"
```

the following can be accessed by the following commands

`rtoml <filename> value1`

`rtoml <filename> table.value2`

for arrays each value will be printed on a separate newline, therefore the command

`rtoml <filename> array_value`

will produce the output

```text
1
2
3
```

spaces in table and value names are also supported as long as the entire passed value is quoted like so

`rtoml <filename> "table with spaces.value with spaces"`

## Installation

`cargo install rtoml-cli`

precompiled binaries are also available on github if rust is not installed on the computer
