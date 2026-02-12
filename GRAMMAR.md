# JASN Grammar Specification

JASN (Just Another Serialization Notation) extends JSON with explicit integer and binary types, plus convenient JSON5-inspired syntax features.

## Overview

- **Integers**: Distinct 64-bit signed integer type, supporting decimal, hexadecimal, binary, and octal notation
- **Binary**: Byte array type with base64 (`b64"..."`) and hex (`h"..."`) encoding
- **JSON5 Features**: Trailing commas, single quotes, unquoted keys, liberal number parsing
- **No Comments**: Not supported in initial version

## EBNF Grammar

```ebnf
(* Root *)
value = null | boolean | integer | float | string | binary | list | map ;

(* Primitives *)
null = "null" ;

boolean = "true" | "false" ;

(* Numbers *)
integer = [ sign ] , ( decimal_integer | hex_integer | binary_integer | octal_integer ) ;
decimal_integer = digit , { [ "_" ] , digit } ;
hex_integer = ( "0x" | "0X" ) , hex_digit , { [ "_" ] , hex_digit } ;
binary_integer = ( "0b" | "0B" ) , binary_digit , { [ "_" ] , binary_digit } ;
octal_integer = ( "0o" | "0O" ) , octal_digit , { [ "_" ] , octal_digit } ;

float = [ sign ] , ( decimal_float | special_float | infinity | nan ) ;
decimal_float = ( int_part , frac_part , [ exp_part ] )
              | ( int_part , frac_part )
              | ( int_part , exp_part )
              | ( frac_part , [ exp_part ] ) ;
int_part = digit , { digit } ;
frac_part = "." , [ digit , { digit } ] ;
exp_part = ( "e" | "E" ) , [ sign ] , digit , { digit } ;
special_float = int_part , "." ;  (* Trailing dot: "5." *)
infinity = "inf" | "Inf" | "INF" ;
nan = "nan" | "NaN" | "NAN" ;

sign = "+" | "-" ;
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
binary_digit = "0" | "1" ;
octal_digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" ;
hex_digit = digit | "a" | "b" | "c" | "d" | "e" | "f"
                  | "A" | "B" | "C" | "D" | "E" | "F" ;

(* Strings *)
string = double_quoted_string | single_quoted_string ;
double_quoted_string = '"' , { string_char_double | escape_sequence } , '"' ;
single_quoted_string = "'" , { string_char_single | escape_sequence } , "'" ;
string_char_double = ? any Unicode character except '"', '\', or control characters ? ;
string_char_single = ? any Unicode character except "'", '\', or control characters ? ;

escape_sequence = "\\" , ( '"' | "'" | "\\" | "/" | "b" | "f" | "n" | "r" | "t"
                         | unicode_escape ) ;
unicode_escape = "u" , hex_digit , hex_digit , hex_digit , hex_digit ;

(* Binary *)
binary = base64_binary | hex_binary ;
base64_binary = "b64" , '"' , { base64_char } , '"' ;
hex_binary = "h" , '"' , { hex_digit } , '"' ;
base64_char = letter | digit | "+" | "/" | "=" ;
letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M"
       | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
       | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
       | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" ;

(* Lists *)
list = "[" , [ whitespace ] , [ value_list ] , [ "," ] , [ whitespace ] , "]" ;
value_list = value , { [ whitespace ] , "," , [ whitespace ] , value } ;

(* Maps *)
map = "{" , [ whitespace ] , [ member_list ] , [ "," ] , [ whitespace ] , "}" ;
member_list = member , { [ whitespace ] , "," , [ whitespace ] , member } ;
member = key , [ whitespace ] , ":" , [ whitespace ] , value ;

key = string | identifier ;
identifier = id_start , { id_continue } ;
id_start = letter | "_" ;
id_continue = id_start | digit ;

(* Whitespace *)
whitespace = { " " | "\t" | "\n" | "\r" } ;
```

## Type Resolution Rules

JASN distinguishes between integers and floats at parse time:

### Integer Type (64-bit signed integer)
- Decimal digits only: `42`, `-123`, `+99`, `1_000_000`
- Hexadecimal notation: `0xFF`, `0x10`, `-0xDEAD_BEEF`
- Binary notation: `0b1010`, `0b1111_1111`, `-0b1000`
- Octal notation: `0o755`, `0o644`, `+0o777`
- Underscores allowed between digits for readability (no double underscores)
- No decimal point, no exponent

### Float Type (IEEE 754 binary64)
- Contains decimal point: `42.0`, `.5`, `5.`
- Contains exponent: `1e10`, `2.5e-3`, `5E+2`
- Special values: `inf`, `+inf`, `-inf`, `nan` (case-insensitive)

## Examples

### Integers
```jasn
42
-123
+99
1_000_000
0xFF
0x10
-0xDEAD_BEEF
0b1010
0b1111_1111
-0b1000
0o755
0o644
+0o777
0o100_000
```

### Floats
```jasn
42.0
3.14159
-2.5
.5
5.
1e10
2.5e-3
5E+2
inf
+inf
-inf
nan
```

### Binary
```jasn
b64"SGVsbG8gV29ybGQh"
b64"AQIDBA=="
b64""
h"48656c6c6f20576f726c6421"
h"01020304"
h"DEADBEEF"
h""
```

### Strings
```jasn
"double quotes"
'single quotes'
"escaped: \"quote\" and \n newline"
'also escaped: \' and \\'
"unicode: \u0041\u0042\u0043"
```

### Lists (with trailing commas)
```jasn
[1, 2, 3]
[1, 2, 3,]
[
  42,
  "hello",
  true,
]
[]
```

### Maps (with unquoted keys and trailing commas)
```jasn
{
  "quoted": 1,
  'single': 2,
  unquoted: 3,
  _private_123: 4,
}

{
  name: "Alice",
  age: 30,
  data: b64"YmluYXJ5",
}
```

### Complex Example
```jasn
{
  version: 1,
  count: 0x100,
  ratio: 3.14,
  name: "JASN Example",
  active: true,
  metadata: null,
  binary_data: b64"SGVsbG8=",
  items: [
    { id: 1, value: 10.5, },
    { id: 2, value: 20.0, },
    { id: 3, value: .5, },
  ],
  config: {
    timeout: 30,
    'max-retries': 5,
    enabled: true,
  },
}
```

## Differences from JSON

1. **Integer type**: Numbers without decimal point/exponent are 64-bit signed integers, not double-precision floats
2. **Binary type**: New `b64"..."` and `h"..."` literals for binary data
3. **Trailing commas**: Allowed in lists and maps
4. **Single quotes**: Strings can use `'...'` or `"..."`
5. **Unquoted keys**: Map keys can be identifiers
6. **Multiple radix integers**: `0x` (hex), `0b` (binary), `0o` (octal) prefixes
7. **Liberal numbers**: Leading/trailing decimal points (`.5`, `5.`), explicit sign (`+42`), underscores in integers (`1_000`)
8. **Special floats**: `inf`, `nan` with signs

## Differences from JSON5

1. **No comments**: Not supported (yet)
2. **Integer/Float split**: Explicit type distinction based on syntax
3. **Binary literals**: New `b64"..."` and `h"..."` types
4. **Multi-line strings**: Not supported (standard JSON escaping only)
5. **Infinity/NaN**: Supported with simpler syntax (`inf`, `nan` vs `Infinity`, `NaN`)

## Future Considerations

- Additional binary encodings: `b"..."` for Python-style b-strings
- Comments: `//` line and `/* */` block comments
