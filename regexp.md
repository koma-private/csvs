# Regular Expressions in SQL Queries

Regular expressions are a powerful tool for filtering and transforming data based on flexible matching patterns. In
**csvs**, you can use `regexp()` and `regexf()` to integrate regex-based filtering directly into your SQL queries,
enabling sophisticated text processing.

For details on supported patterns, refer to
the [lazy_regex crate documentation](https://docs.rs/lazy-regex/latest/lazy_regex/index.html).

## Functions Overview

| Function | Description                                                                                    |
|----------|------------------------------------------------------------------------------------------------|
| `regexp` | Basic regex matching without flags. Useful for simple pattern searches.                        |
| `regexf` | Advanced regex matching with support for flags, allowing case-insensitivity and other options. |

---

## `regexp()` function

### Syntax

1. Function format:

```sql
regexp (`regular expression pattern`, `field or value to test`)
```

2. Operator format:

```sql
`field or value to test` REGEXP `regular expression pattern`
```

### Example

The following examples are functionally equivalent:

```sql
SELECT *
FROM "users.csv"
WHERE regexp ('^Ke.', name);

SELECT *
FROM "users.csv"
WHERE name REGEXP '^Ke.';
```

---

## `regexf()` function

The `regexf` function (REGular EXpression with Flags) allows you to specify flags for advanced pattern matching, such as case-insensitivity or Unicode
support. This function is available only in function format.

### Syntax

```sql
regexf (`regular expression pattern`, `field or value to test`, `flags`)
```

### Available flags

| Flag | Description                                               |
|------|-----------------------------------------------------------|
| i    | Enables case insensitive matching for the entire pattern. |
| x    | Treats whitespace in the pattern as insignifcant.         |
| U    | Enables swap-greed mode for the entire pattern.           |
| u    | Enables Unicode support for the entire pattern.           |

### Example

The following example enables both case-insensitivity and Unicode support:

```sql
SELECT *
FROM "users.csv"
WHERE regexf('^Ke.', name, 'iu');
```

---

## Differences Between `regexp()` and `regexf()`

| Feature            | `regexp()`               | `regexf()`           |
|--------------------|--------------------------|----------------------|
| Basic matching     | Yes                      | Yes                  |
| Support for flags  | No                       | Yes                  |
| Syntax flexibility | Supports operator format | Function format only |

---

By using `regexp()` and `regexf()`, **csvs** provides robust tools for integrating regular expressions into SQL queries,
enabling versatile and efficient text filtering. Leverage these functions to handle sophisticated data processing tasks
seamlessly.
