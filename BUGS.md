# Spoilers

There are 2 main bugs in the code:

## Multi-byte UTF-8 characters

When incrementing the string slice, the code currently increments by just 1 each time, rather than incrementing by the next character size. Rust panics when it tries to increment into the middle of a multi-byte character.

When you encounter this one, you can checkout to the `fix-utf8` branch for a simple quick fix.

## Stack overflow

The code currently uses recursion to parse the json, which means that a deeply nested json file will cause a stack overflow. This can be fixed by using a stack instead of recursion, or other kinds of bottom-up parsing.

There is no branch fixing this one as it requires a significant rewrite to fix.
