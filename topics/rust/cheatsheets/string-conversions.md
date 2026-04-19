---
title: String Conversions Cheatsheet
tags: [rust, cheatsheet, strings]
---

# String Conversions Cheatsheet

## The type zoo

| Type | Owned? | UTF-8? | Null-term? | Use |
|---|---|---|---|---|
| `String` | ✅ | ✅ | ❌ | Owned text |
| `&str` | ❌ | ✅ | ❌ | Borrowed text view |
| `Vec<u8>` | ✅ | ❌ | ❌ | Owned bytes |
| `&[u8]` | ❌ | ❌ | ❌ | Borrowed bytes |
| `CString` | ✅ | ❌ | ✅ | FFI / C interop |
| `CStr` | ❌ | ❌ | ✅ | FFI / C interop |
| `OsString` | ✅ | platform | ❌ | OS paths, env, args |
| `OsStr` | ❌ | platform | ❌ | OS paths, env, args |
| `PathBuf` | ✅ | platform | ❌ | Filesystem paths |
| `Path` | ❌ | platform | ❌ | Filesystem paths |

## The cheatsheet

| From → To | How |
|---|---|
| `&str` → `String` | `.to_string()` / `String::from(s)` / `s.to_owned()` / `s.into()` |
| `String` → `&str` | `&s` (deref) / `s.as_str()` |
| `&str` → `&[u8]` | `.as_bytes()` |
| `String` → `Vec<u8>` | `.into_bytes()` |
| `&[u8]` → `&str` | `std::str::from_utf8(b)?` |
| `Vec<u8>` → `String` | `String::from_utf8(v)?` |
| `&[u8]` → `String` (lossy) | `String::from_utf8_lossy(b).into_owned()` |
| number → `String` | `n.to_string()` / `format!("{n}")` |
| `&str` → number | `.parse::<i32>()?` |
| `&str` → chars | `.chars()` |
| chars → `String` | `.collect::<String>()` |
| `String` → `PathBuf` | `PathBuf::from(s)` |
| `Path` → `&str` | `.to_str()?` (may fail on non-UTF-8) |

## `format!` vs `to_string`

- `format!("{x}")` — allocates a new `String`.
- `x.to_string()` — almost identical; uses `Display` impl.
- Prefer `format!` when you need more than one value or formatting spec.

## Common pitfalls

- `s[0]` → compile error. Use `s.chars().next()` or byte access via `.as_bytes()[0]`.
- `s.len()` is **bytes**, not chars. Use `s.chars().count()` for chars.
- Slicing `s[0..3]` by byte can panic on non-ASCII. Use `s.char_indices()` boundaries.
- `println!("{}", path)` → error — `Path` is `Display`-less. Use `path.display()`.
