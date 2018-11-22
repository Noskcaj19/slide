# Slide parsing

This crate provides all the parsing for the slide calculator language, and has been split off to (hopefully) help with compile times.

The token module uses a simple, flat pest PEG grammar as a (overpowered) lexer and emits a token stream.
This token stream contains both tokens and spans, for use in error reporting.

The ast module uses a lalrpop grammar to generate a parser that is able to parse a single, or multiple expressions.
The lalrpop parser also emits ranges, which are used to display detailed error messages.
