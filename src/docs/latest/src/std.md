# Standard Library

> NB: All the standard library is a work in progress.

| Module | Description |
| ------ | ----------- |
| `std/io` | Input/output functions |
| `std/fs` | File handling functions |
| `std/str` | String manipulation functions |
| `std/list` | List manipulation functions |
| `std/time` | Time functions |
| `std/math` | Math functions |

As of now only part of the `std/io` module is implemented and working properly.
When the `v0.5.1` will be released, `std/fs`, `std/str`, `std/io`, `std/math` & `std/time` should be fully implemented and working.

## `std/io`

- `print(val: str) -> unit`: Print a string to the standard output.
- `println(val: str) -> unit`: Print a string to the standard output followed by a newline.
- `input() -> str`: Read a line from the standard input.


## `std/fs`

- `File`: A file type that can be used to read and write files.
> NB: The `File` type is not implemented yet.
- `open(filename: str, mode: str) -> File`: Open a file in the specified mode.
- `read(file: File) -> str`: Read the content of a file.
- `write(file: File, content: str) -> unit`: Write content to a file.
- `close(file: File) -> unit`: Close a file.
- `exists(filename: str) -> bool`: Check if a file exists.
- `remove(filename: str) -> unit`: Remove a file.
- `rename(old: str, new: str) -> unit`: Rename a file.

## `std/str`

- `String`: A string type that can be used to manipulate strings.
> NB: The `String` type is not implemented yet.
- `str_len(s: String) -> u64`: Get the length of a string.
- `concat(s1: String, s2: String) -> String`: Concatenate two strings.
- `split(s: String, sep: str) -> List`: Split a string into a list of substrings.
- `join(lst: List, sep: str) -> String`: Join a list of strings into a single string.
- `replace(s: String, old: str, new: str) -> String`: Replace all occurrences of a substring in a string.
- `to_upper(s: String) -> String`: Convert a string to uppercase.
- `to_lower(s: String) -> String`: Convert a string to lowercase.
- `trim(s: String) -> String`: Remove leading and trailing whitespace from a string.
- `starts_with(s: String, prefix: str) -> bool`: Check if a string starts with a prefix.
- `ends_with(s: String, suffix: str) -> bool`: Check if a string ends with a suffix.
- `contains(s: String, sub: str) -> bool`: Check if a string contains a substring.
- `find(s: String, sub: str) -> i64`: Find the index of the first occurrence of a substring in a string.
- `rfind(s: String, sub: str) -> i64`: Find the index of the last occurrence of a substring in a string.
- `slice(s: String, start: i64, end: i64) -> String`: Get a substring of a string.

## `std/list`

> NB: `std/list` is not implemented yet.


## `std/time`

- `Time`: A time type that can be used to represent time.
```
struct Time {
    sec: u64;
    nsect: u64;
}
```
- `now() -> Time`: Get the current time.
- `format(t: Time, fmt: str) -> str`: Format a time value as a string.
- `sleep(ms: u64) -> unit`: Sleep for the specified number of milliseconds.
- `sleep_us(us: u64) -> unit`: Sleep for the specified number of microseconds.
- `sleep_ns(ns: u64) -> unit`: Sleep for the specified number of nanoseconds.
- `elapsed(start: Time, end: Time) -> u64`: Calculate the elapsed time between two time values in milliseconds.
- `elapsed_us(start: Time, end: Time) -> u64`: Calculate the elapsed time between two time values in microseconds.
- `elapsed_ns(start: Time, end: Time) -> u64`: Calculate the elapsed time between two time values in nanoseconds.
- `parse(s: str, fmt: str) -> Time`: Parse a string into a time value.
- `add(t: Time, ms: u64) -> Time`: Add milliseconds to a time value.

## `std/math`

- `abs(x: f64) -> f64`: Compute the absolute value of a floating-point number.
- `sqrt(x: f64) -> f64`: Compute the square root of a floating-point number.
- `pow(x: f64, y: f64) -> f64`: Compute `x` raised to the power of `y`.
- `exp(x: f64) -> f64`: Compute the exponential function of a floating-point number.
- `log(x: f64) -> f64`: Compute the natural logarithm of a floating-point number.
- `log10(x: f64) -> f64`: Compute the base 10 logarithm of a floating-point number.
- `sin(x: f64) -> f64`: Compute the sine of a floating-point number.
- `cos(x: f64) -> f64`: Compute the cosine of a floating-point number.
- `tan(x: f64) -> f64`: Compute the tangent of a floating-point number.
- `asin(x: f64) -> f64`: Compute the arcsine of a floating-point number.
- `acos(x: f64) -> f64`: Compute the arccosine of a floating-point number.
- `atan(x: f64) -> f64`: Compute the arctangent of a floating-point number.
