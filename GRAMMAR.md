# JASN Grammar Specification

JASN (Just Another Serialization Notation) extends JSON with explicit integer and binary types, plus convenient JSON5-inspired syntax features.

## Overview

- **Integers**: Distinct 64-bit signed integer type, supporting decimal, hexadecimal, binary, and octal notation
- **Binary**: Byte array type with base64 (`b64"..."`) and hex (`h"..."`) encoding
- **JSON5 Features**: Trailing commas, single quotes, unquoted keys, liberal number parsing, comments
- **Comments**: Line comments (`//`) and block comments (`/* */`)

## EBNF Grammar

```ebnf
(* Root *)
value = null | boolean | integer | float | string | binary | list | map ;

(* Primitives *)
null = "null" ;

boolean = "true" | "false" ;

(* Numbers *)
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

(* Comments *)
line_comment = "//" , { ? any character except newline ? } , ( "\n" | end_of_file ) ;
block_comment = "/*" , { ? any character ? - ( "*/" ) } , "*/" ;
comment = line_comment | block_comment ;
```

## Type Resolution Rules

JASN distinguishes between integers and floats at parse time:

### Integer Type (64-bit signed integer)
- Decimal digits only: `42`, `-123`, `+99`, `1_000_000`
- Hexadecimal notation: `0xFF`, `0x10`, `-0xDEAD_BEEF`
- Binary notation: `0b1010`, `0b1111_1111`, `-0b1000`
- Octal notation: `0o755`, `0o644`, `+0o777`
- Underscores allowed between digits for readability (including multiple consecutive: `1__000`, `1___000`)
- Underscores not allowed at the start or end of the number
- No decimal point, no exponent

### Float Type (IEEE 754 binary64)
- Contains decimal point: `42.0`, `.5`, `5.`
- Contains exponent: `1e10`, `2.5e-3`, `5E+2`
- Special values: `inf`, `+inf`, `-inf`, `nan` (lowercase only)

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
// Configuration file example
{
  // Version information
  version: 1,
  count: 0x100,  // Hex integer
  ratio: 3.14,
  name: "JASN Example",
  active: true,
  metadata: null,
  
  /* Binary data can be encoded
     in multiple formats */
  binary_data: b64"SGVsbG8=",
  
  items: [
    { id: 1, value: 10.5, },  // First item
    { id: 2, value: 20.0, },  // Second item
    { id: 3, value: .5, },    // Third item
  ],
  
  config: {
    timeout: 30,           // seconds
    'max-retries': 5,      /* quoted key with dash */
    enabled: true,
  },
}
```

## Differences from JSON

1. **Integer type**: Numbers without decimal point/exponent are 64-bit signed integers, not double-precision floats
2. **Binary type**: New `b64"..."` and `h"..."` literals for binary data
3. **Trailing commas**: Allowed in lists and maps
4. **Single quotes**: Strings can use `'...'` or `"..."`
5. **Unquoted keys**: Map keys can be identifiers, including reserved words (`null`, `true`, `false`, `inf`, `nan`)
6. **Duplicate keys**: Not allowed in maps (parse error)
7. **Multiple radix integers**: `0x` (hex), `0b` (binary), `0o` (octal) prefixes (case-insensitive)
7. **Liberal numbers**: Leading/trailing decimal points (`.5`, `5.`), explicit sign (`+42`), underscores in integers (`1_000`, `1__000`)
8. **Special floats**: `inf`, `nan` with signs (lowercase only)
9. **Comments**: Line comments `//` and block comments `/* */`

## JSON Compatibility

JASN is designed to accept most valid JSON with the following important limitations:

### Integer Range Restriction

**Numbers without decimal points or exponents are parsed as 64-bit signed integers** with range:
- Minimum: `-9,223,372,036,854,775,808` (-2^63)
- Maximum: `9,223,372,036,854,775,807` (2^63 - 1)

**Valid JSON documents containing integers outside this range will be rejected** as parse errors.

Examples:
```jasn
9223372036854775807   // ✓ Valid (max i64)
9223372036854775808   // ✗ Parse error (overflow)
-9223372036854775808  // ✓ Valid (min i64)
-9223372036854775809  // ✗ Parse error (underflow)
```

**Workaround:** Use float notation for numbers outside the i64 range:
```jasn
9223372036854775808.0   // ✓ Valid as float
1e20                     // ✓ Valid as float
```

### Type Distinction

Unlike JSON (where all numbers are typically implemented as doubles), JASN distinguishes:
- `42` → 64-bit signed integer
- `42.0` → IEEE 754 binary64 float

This means `42` and `42.0` are **different types** in JASN, though mathematically equivalent.

### Compatibility Summary

- ✓ All valid JSON strings, booleans, null
- ✓ All JSON objects (maps) and arrays (lists)
- ✓ All JSON whitespace and escape sequences
- ✓ JSON numbers within i64 range (may become integer type)
- ✗ JSON integers outside ±2^63-1 (rejected)
- ✓ More permissive: allows leading zeros, trailing commas (JSON forbids these)

## Differences from JSON5

1. **Integer/Float split**: Explicit type distinction based on syntax
2. **Binary literals**: New `b64"..."` and `h"..."` types
3. **Multi-line strings**: Not supported (standard JSON escaping only)
4. **Infinity/NaN**: Supported with simpler syntax (`inf`, `nan` vs `Infinity`, `NaN`)
5. **Additional integer radixes**: Binary (`0b`) and octal (`0o`) literals beyond JSON5

## Future Considerations

- Additional binary encodings: `b"..."` for Python-style b-strings
- Multi-line strings with proper indentation handling
- BigInt support for arbitrary precision integers
