# What `#[derive(FromStr)]` generates

Deriving `FromStr` only works for enums with no fields
or newtypes, i.e structs with only a single
field. The result is that you will be able to call the `parse()` method on a
string to convert it to your newtype. This only works when the type that is
contained in the type implements `FromStr`.




## Example usage

```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
struct MyInt(i32);

#[derive(FromStr, Debug, Eq, PartialEq)]
struct Point1D{
    x: i32,
}

assert_eq!(MyInt(5), "5".parse().unwrap());
assert_eq!(Point1D{x: 100}, "100".parse().unwrap());
```




## Tuple structs

When deriving `FromStr` for a tuple struct with one field:

```rust
# use derive_more::FromStr;
#
#[derive(FromStr)]
struct MyInt(i32);
```

Code like this will be generated:

```rust
# struct MyInt(i32);
impl ::core::str::FromStr for MyInt {
    type Err = <i32 as ::core::str::FromStr>::Err;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        return Ok(MyInt(i32::from_str(src)?));
    }
}
```




## Regular structs

When deriving `FromStr` for a regular struct with one field:

```rust
# use derive_more::FromStr;
#
#[derive(FromStr)]
struct Point1D {
    x: i32,
}
```

Code like this will be generated:

```rust
# struct Point1D {
#     x: i32,
# }
impl ::core::str::FromStr for Point1D {
    type Err = <i32 as ::core::str::FromStr>::Err;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        return Ok(Point1D {
            x: i32::from_str(src)?,
        });
    }
}
```




## Enums

When deriving `FromStr` for an enums with variants with no fields it will
generate a `from_str` method that converts strings that match the variant name
to the variant. If using a case insensitive match would give a unique variant
(i.e you dont have both a `MyEnum::Foo` and a `MyEnum::foo` variant) then case
insensitive matching will be used, otherwise it will fall back to exact string
matching.

Since the string may not match any vairants an error type is needed so one
will be generated of the format `Parse{}Error`.

e.g. Given the following enum:

```rust
# use derive_more::FromStr;
#
#[derive(FromStr)]
enum EnumNoFields {
    Foo,
    Bar,
    Baz,
}
```

Code like this will be generated:

```rust
# enum EnumNoFields {
#     Foo,
#     Bar,
#     Baz,
# }
#
#[derive(Clone, Debug, Eq, PartialEq)]
struct ParseEnumNoFieldsError;

impl std::fmt::Display for ParseEnumNoFieldsError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("invalid enum no fields")
    }
}

impl std::error::Error for ParseEnumNoFieldsError {}

impl ::core::str::FromStr for EnumNoFields {
    type Err = ParseEnumNoFieldsError;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(match src.to_lowercase().as_str() {
            "foo" => EnumNoFields::Foo,
            "bar" => EnumNoFields::Bar,
            "baz" => EnumNoFields::Baz,
            _ => return Err(ParseEnumNoFieldsError{}),
        })
    }
}
```
