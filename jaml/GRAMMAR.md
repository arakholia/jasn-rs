# JAML Grammar Specification

> **Note:** This specification is still under active development and may be subject to change.

JAML (Just Another Markup Language) is a YAML-inspired serialization format with explicit integer and binary types, sharing the same data model as JASN but using indentation-based syntax instead of braces and brackets.

## Overview

- **Integers**: Distinct 64-bit signed integer type, supporting decimal, hexadecimal, binary, and octal notation
- **Binary**: Byte array type with base64 (`b64"..."`) and hex (`hex"..."`) encoding
- **Timestamps**: ISO8601/RFC3339 timestamp literals with `ts"..."` syntax
- **Indentation-Based**: Uses indentation to denote structure (like YAML/Python), not braces/brackets
- **Explicit Strings**: All strings must be quoted (avoids "The Norway Problem")
- **Unquoted Keys**: Map keys can be unquoted identifiers
- **Comments**: Line comments (`#`)

## Indentation Rules

JAML uses **strict 2-space indentation**:

### 1. Fixed Indentation
- **Exactly 2 spaces** per indentation level
- **No tabs allowed** - only spaces
- **No mixing**: Every indent must be exactly 0, 2, 4, 6, 8... spaces

### 2. Whitespace Requirements
- **No trailing whitespace**: Lines must not have spaces or tabs after the last non-whitespace character
- **Blank lines**: May contain only a newline, no spaces
- **After `-`**: Exactly one space required, or immediate newline for nested content
- **After `:`**: Exactly one space required before inline value, or immediate newline for block value

### 3. Whitespace Significance
- **Line start**: Indentation (spaces or tabs) determines structure depth
- **After `-`**: Exactly one space required before the list item value
- **After `:`**: Exactly one space required before inline value (or newline for block value)
- **Blank lines**: May contain only a newline, no spaces or tabs
- **Comments**: `#` can appear after indentation or inline (with space before `#`)

### 3. Examples

**Valid (2-space indentation):**
```jaml
# Root level (indent 0)
key: "value"
nested:
  # First indent level (2 spaces)
  child: "value"
  deeper:
    # Second indent level (4 spaces)
    grandchild: "value"
```

**Invalid (tabs):**
```jaml
key: "value"
nested:
	child: "value"    # tab - ERROR! Must use spaces
```

**Invalid (wrong indent amount):**
```jaml
key: "value"
nested:
   child: "value"    # 3 spaces - ERROR! Must be exactly 2
```

**Invalid (trailing whitespace):**
```jaml
key: "value"    
# ERROR: spaces after "value"
```

## EBNF Grammar

```ebnf
(* Root *)
document = [ whitespace ] , value , [ whitespace ] ;
value = null | boolean | integer | float | string | binary | timestamp | block_list | block_map | inline_value ;

(* Primitives *)
null = "null" ;

boolean = "true" | "false" ;

(* Numbers - same as JASN *)
integer = [ sign ] , ( decimal_integer | hex_integer | binary_integer | octal_integer ) ;
decimal_integer = digit , { { "_" } , digit } ;
hex_integer = ( "0x" | "0X" ) , hex_digit , { { "_" } , hex_digit } ;
binary_integer = ( "0b" | "0B" ) , binary_digit , { { "_" } , binary_digit } ;
octal_integer = ( "0o" | "0O" ) , octal_digit , { { "_" } , octal_digit } ;

float = [ sign ] , ( infinity | nan | decimal_float | special_float ) ;
decimal_float = ( int_part , frac_part , [ exp_part ] )
              | ( int_part , exp_part )
              | ( frac_part , [ exp_part ] ) ;
int_part = digit , { digit } ;
frac_part = "." , digit , { digit } ;
exp_part = ( "e" | "E" ) , [ sign ] , digit , { digit } ;
special_float = int_part , "." ;  (* Trailing dot: "5." *)
infinity = "inf" ;
nan = "nan" ;

sign = "+" | "-" ;
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
binary_digit = "0" | "1" ;
octal_digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" ;
hex_digit = digit | "a" | "b" | "c" | "d" | "e" | "f"
                  | "A" | "B" | "C" | "D" | "E" | "F" ;

(* Strings - MUST be quoted *)
string = double_quoted_string | single_quoted_string ;
double_quoted_string = '"' , { string_char_double | escape_sequence } , '"' ;
single_quoted_string = "'" , { string_char_single | escape_sequence } , "'" ;
string_char_double = ? any Unicode character except '"', '\', or control characters ? ;
string_char_single = ? any Unicode character except "'", '\', or control characters ? ;

escape_sequence = "\\" , ( '"' | "'" | "\\" | "/" | "b" | "f" | "n" | "r" | "t"
                         | unicode_escape ) ;
unicode_escape = "u" , hex_digit , hex_digit , hex_digit , hex_digit ;

(* Binary - same as JASN *)
binary = base64_binary | hex_binary ;
base64_binary = "b64" , '"' , { base64_char } , '"' ;
hex_binary = "hex" , '"' , { hex_digit } , '"' ;
base64_char = letter | digit | "+" | "/" | "=" ;
letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M"
       | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
       | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
       | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" ;

(* Timestamps - same as JASN *)
timestamp = "ts" , '"' , iso8601_datetime , '"' ;
iso8601_datetime = ? ISO 8601 / RFC 3339 formatted datetime string ? ;

(* Block Lists *)
block_list = list_item , { newline , indent , list_item } ;
list_item = "-" , ( space , inline_value | newline , indent , value ) ;

(* Block Maps *)
block_map = map_entry , { newline , indent , map_entry } ;
map_entry = key , ":" , ( space , inline_value | newline , indent , value ) ;

(* Inline values - primitives that can appear on same line *)
inline_value = null | boolean | integer | float | string | binary | timestamp ;

(* Keys - can be unquoted identifiers or quoted strings *)
key = string | identifier ;
identifier = id_start , { id_continue } ;
id_start = letter | "_" ;
id_continue = id_start | digit ;

(* Indentation and Whitespace *)
indent = { space_char , space_char } ;
  (* indentation is always an even number of spaces: 0, 2, 4, 6, ... *)
space_char = " " ;
space = " " ;
newline = "\n" | "\r\n" | "\r" ;
whitespace = { space | newline } ;

(* Comments *)
line_comment = "#" , { ? any character except newline ? } , ( newline | end_of_file ) ;
```

## Type Resolution Rules

JAML uses the same type resolution rules as JASN:

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

YAML 1.1 has a famous issue where certain unquoted strings are implicitly converted to booleans. For example, `no`, `NO`, `yes`, `YES`, `on`, `off`, etc. are treated as booleans rather than strings. This causes problems with country codes:

**YAML 1.1 (problematic):**
```yaml
countries:
  NO: Norway    # NO becomes false!
  yes: Yemen    # yes becomes true!
  off: "a country"  # off becomes false!
```

**JAML (explicit):**
```jaml
countries:
  NO: "Norway"     # NO is a string key, "Norway" is a string value
  yes: "Yemen"     # yes is a string key, "Yemen" is a string value  
  off: "a country" # off is a string key, value is explicitly quoted
```

JAML completely avoids this issue by requiring all non-primitive values to be explicitly quoted. The only unquoted values allowed are: `null`, `true`, `false`, `inf`, `-inf`, `+inf`, `nan`, and numeric literals.

## Differences from YAML

1. **Explicit strings**: All string values must be quoted (keys can be unquoted identifiers)
2. **Integer/Float distinction**: `42` and `42.0` are different types
3. **Binary type**: Native `b64"..."` and `hex"..."` literals
4. **Timestamp type**: Native `ts"..."` literals
5. **No implicit conversion**: Values are never converted to strings automatically
6. **Strict indentation**: First indented line determines the style for entire document
7. **Single document**: No multi-document support (no `---` or `...`)
8. **No anchors/aliases**: No `&anchor` or `*alias` support
9. **No tags**: No `!!type` support
10. **Simpler syntax**: Focused subset of YAML with explicit types

## Differences from JASN

1. **Indentation-based**: Uses indentation instead of `{}` and `[]`
2. **Comments**: Uses `#` instead of `//` and `/* */`
3. **No trailing commas**: Not needed in block syntax
4. **No braces/brackets**: Map and list syntax is based on indentation only

## Future Considerations

- Optional support for JASN-style inline `{}` and `[]` syntax for compact representation
- Flow-style syntax for single-line lists and maps
- Multi-line strings with proper indentation handling
- BigInt support for arbitrary precision integers
