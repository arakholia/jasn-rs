# JAML Grammar Specification

> **Note:** This specification is still under active development and may be subject to change.

JAML (Just Another Markup Language) is a YAML-inspired serialization format with explicit integer and binary types, sharing the same data model as JASN but using indentation-based syntax as its primary structure notation.

## Overview

- **Integers**: Distinct 64-bit signed integer type, supporting decimal, hexadecimal, binary, and octal notation
- **Binary**: Byte array type with base64 (`b64"..."`) and hex (`hex"..."`) encoding
- **Timestamps**: ISO8601/RFC3339 timestamp literals with `ts"..."` syntax
- **Indentation-Based**: Primary syntax uses indentation (like YAML/Python); compact inline `[]` and `{}` also supported
- **Explicit Strings**: All strings must be quoted (avoids ["The Norway Problem"](https://lab174.com/blog/202601-yaml-norway/))
- **Unquoted Keys**: Map keys can be unquoted identifiers
- **Comments**: Line comments (`#`)

## Indentation and Whitespace

JAML uses **flexible indentation** similar to Python:

- **First indent defines the base unit**: The first indented line establishes the indentation size (e.g., 2 spaces, 4 spaces, 1 tab)
- **All indents must be multiples**: Every subsequent indent must be N × (base unit), where N ≥ 0
- **No mixing**: Cannot mix spaces and tabs in the same document
- **After `:` or `-`**: Requires one or more spaces before inline value, or newline for block value
- **Blank lines**: May contain only whitespace (recommended: empty)
- **Comments**: `#` starts a line comment, can appear after values or on its own

**Valid (2-space indentation):**
```jaml
foo:
  a: 1
  b:
    c: 2
bar: 3
```

**Invalid (not a multiple):**
```jaml
foo:
  a: 1        # Base unit: 2 spaces
   b: 2       # ERROR: 3 is not a multiple of 2
```

**Invalid (mixed tabs/spaces):**
```jaml
foo:
  a: 1        # Base unit: spaces
	b: 2        # ERROR: can't switch to tabs
```

## EBNF Grammar

```ebnf
(* Root - a sequence of lines forming an implicit map *)
document = { line } ;
line = indent , ( content | comment ) , newline ;
content = map_entry | inline_value | list_item ;

(* Core Values *)
value = null | boolean | float | integer | string | binary | timestamp | inline_list | inline_map ;

(* Primitives *)
null = "null" ;
boolean = "true" | "false" ;

(* Numbers *)
integer = [ sign ] , ( decimal_integer | hex_integer | binary_integer | octal_integer ) ;
decimal_integer = digit , { [ "_" ] , digit } ;
hex_integer = ( "0x" | "0X" ) , hex_digit , { [ "_" ] , hex_digit } ;
binary_integer = ( "0b" | "0B" ) , binary_digit , { [ "_" ] , binary_digit } ;
octal_integer = ( "0o" | "0O" ) , octal_digit , { [ "_" ] , octal_digit } ;

float = [ sign ] , ( infinity | nan | decimal_float | special_float ) ;
decimal_float = ( int_part , frac_part , [ exp_part ] )
              | ( int_part , exp_part )
              | ( frac_part , [ exp_part ] ) ;
int_part = digit , { digit } ;
frac_part = "." , digit , { digit } ;
exp_part = ( "e" | "E" ) , [ sign ] , digit , { digit } ;
special_float = int_part , "." ;
infinity = "inf" ;
nan = "nan" ;
sign = "+" | "-" ;

(* Character classes *)
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
binary_digit = "0" | "1" ;
octal_digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" ;
hex_digit = digit | "a" | "b" | "c" | "d" | "e" | "f"
                  | "A" | "B" | "C" | "D" | "E" | "F" ;

(* Strings - MUST be quoted *)
string = double_quoted_string | single_quoted_string ;
double_quoted_string = '"' , { string_char | escape_sequence } , '"' ;
single_quoted_string = "'" , { string_char | escape_sequence } , "'" ;
string_char = ? any Unicode character except quotes, backslash, or control characters ? ;
escape_sequence = "\\" , ( '"' | "'" | "\\" | "/" | "b" | "f" | "n" | "r" | "t" | unicode_escape ) ;
unicode_escape = "u" , hex_digit , hex_digit , hex_digit , hex_digit ;

(* Binary *)
binary = base64_binary | hex_binary ;
base64_binary = "b64" , '"' , { base64_char } , '"' ;
hex_binary = "hex" , '"' , { hex_digit } , '"' ;
base64_char = ? A-Z, a-z, 0-9, +, /, = ? ;

(* Timestamps *)
timestamp = "ts" , '"' , iso8601_datetime , '"' ;
iso8601_datetime = ? ISO 8601 / RFC 3339 formatted datetime string ? ;

(* Block Structures - indentation-based *)
list_item = "-" , ( spaces , value | newline , indent , content ) ;
map_entry = key , ":" , ( spaces , value | newline , indent , content ) ;

(* Inline Structures - compact single-line only *)
inline_list = "[" , [ spaces ] , [ value , { [ spaces ] , "," , [ spaces ] , value } , [ "," ] ] , [ spaces ] , "]" ;
inline_map = "{" , [ spaces ] , [ inline_member , { [ spaces ] , "," , [ spaces ] , inline_member } , [ "," ] ] , [ spaces ] , "}" ;
inline_member = key , [ spaces ] , ":" , [ spaces ] , value ;

(* Keys *)
key = string | identifier ;
identifier = id_start , { id_continue } ;
id_start = ? letter or underscore ? ;
id_continue = id_start | digit ;

(* Whitespace - flexible indentation validated at runtime *)
spaces = " " , { " " } ;
indent = { " " | "\t" } ;
newline = "\n" | "\r\n" | "\r" ;

(* Comments *)
comment = "#" , { ? any character except newline ? } ;
```

## Document Structure

JAML documents consist of lines that form an **implicit root map**. Each line can be:
- A map entry (`key: value`)
- A list item (`- value`)
- An inline value (single value, less common at root)
- A comment (`# comment`)

The indentation of each line determines the structure hierarchy.

## Type Resolution Rules

### Integer Type (64-bit signed integer)
- Decimal digits only: `42`, `-123`, `+99`, `1_000_000`
- Hexadecimal notation: `0xFF`, `0x10`, `-0xDEAD_BEEF`
- Binary notation: `0b1010`, `0b1111_1111`, `-0b1000`
- Octal notation: `0o755`, `0o644`, `+0o777`
- Underscores allowed between digits for readability
- No decimal point, no exponent

### Float Type (IEEE 754 binary64)
- Contains decimal point: `42.0`, `.5`, `5.`
- Contains exponent: `1e10`, `2.5e-3`, `5E+2`
- Special values: `inf`, `+inf`, `-inf`, `nan` (lowercase only)

### String Type
- **MUST be quoted**: `"string"` or `'string'`
- **Exception**: Map keys can be unquoted if they are valid identifiers
- No implicit string conversion (avoids YAML's "Norway Problem")

## Examples

### Root-Level Map
```jaml
# Implicit root map
name: "Alice"
age: 30
active: true
balance: 1234.56
```

### Nested Maps
```jaml
person:
  name: "Bob"
  age: 25
  address:
    street: "123 Main St"
    city: "Portland"
    zip: "97201"
```

### Block Lists
```jaml
# Simple list
numbers:
  - 1
  - 2
  - 3
  - 4
  - 5

# List with mixed types (all must be quoted strings except literals)
mixed:
  - 42
  - "hello"
  - true
  - null
```

### Nested Lists
```jaml
# Nested list (value on next indented line)
matrix:
  -
    - 1
    - 2
    - 3
  -
    - 4
    - 5
    - 6
  -
    - 7
    - 8
    - 9
```

### Lists of Maps
```jaml
users:
  - name: "Alice"
    age: 30
    role: "admin"
  - name: "Bob"
    age: 25
    role: "user"
  - name: "Charlie"
    age: 35
    role: "moderator"
```

### All Types Example
```jaml
# Integers
count: 42
hex_value: 0xFF
binary_value: 0b1010
octal_value: 0o755

# Floats
pi: 3.14159
scientific: 1.5e10
special: inf

# Strings (must be quoted)
name: "Alice"
message: 'Hello, World!'
escaped: "quote: \" newline: \n"

# Binary
data_b64: b64"SGVsbG8gV29ybGQh"
data_hex: hex"48656c6c6f"

# Timestamps
created: ts"2024-01-15T12:30:45Z"
updated: ts"2024-01-15T12:30:45.123Z"

# Nested structures
config:
  timeout: 30
  retries: 5
  enabled: true
  
items:
  - id: 1
    value: 10.5
  - id: 2
    value: 20.0
```

### Unquoted vs Quoted Keys
```jaml
# Unquoted keys (identifiers)
simple_key: "value"
camelCase: "value"
snake_case: "value"
_private: "value"
key123: "value"

# Quoted keys (required for special characters)
"quoted-key": "value with dash"
"key with spaces": "spaces require quotes"
"123numeric": "starts with digit, needs quotes"
'single-quoted-key': "single quotes work too"
```

## Avoiding "The Norway Problem"

YAML 1.1 has a famous issue where certain unquoted strings are implicitly converted to booleans. Country codes like `no` (Norway), `on`, `off`, and `yes` become boolean values instead of strings.

**YAML 1.1 (problematic):**
```yaml
countries:
  NO: Norway    # NO becomes false!
  yes: Yemen    # yes becomes true!
```

**JAML (explicit):**
```jaml
countries:
  NO: "Norway"    # NO is unquoted key (identifier), "Norway" is quoted string
  yes: "Yemen"    # yes is unquoted key, "Yemen" is quoted string
  "off": "a country"  # Can also quote keys for special characters
```

JAML avoids this by requiring **all string values to be quoted**. Only these keywords are recognized: `null`, `true`, `false`, `inf`, `-inf`, `+inf`, `nan`, and numeric literals. Everything else must be quoted.

## Differences from YAML

1. **Explicit string values**: All string values must be quoted (keys can be unquoted identifiers)
2. **Integer/Float distinction**: `42` and `42.0` are different types (int vs float)
3. **Binary type**: Native `b64"..."` and `hex"..."` literals for byte arrays
4. **Timestamp type**: Native `ts"..."` literals for ISO8601/RFC3339 timestamps
5. **No implicit type conversion**: No boolean conversion for yes/no/on/off
6. **Flexible indentation**: First indent defines base unit (any size, validated at runtime)
7. **Single document**: No multi-document support (no `---` or `...`)
8. **No anchors/aliases**: No `&anchor` or `*alias` support
9. **No tags**: No `!!type` support
10. **Simpler syntax**: Focused subset of YAML with explicit types and clearer rules

## Differences from JASN

1. **Indentation-based structure**: Primary syntax uses indentation for hierarchy
2. **Inline syntax**: `{}` and `[]` restricted to single-line compact form
3. **Line comments**: Uses `#` instead of block comments `/* */`
4. **Explicit string values**: All string values must be quoted (same data model, stricter syntax)
5. **No trailing commas in block form**: Only needed/allowed in inline `[]` and `{}`

## Future Considerations

- Multi-line strings with proper indentation handling
