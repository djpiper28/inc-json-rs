# The JSON Lexer

The lexer provides methods in to turn text into lexemes, this is done by looking at the
current input token and determining what lexeme it represents, then reading it into a
token.

There are many types of token:

- JSON primitives (number, boolean, null, string)
- Array start/end tokens (`[` or `]`)
- Object start/end tokens (`{` or `}`)
- Object value indicator token (`:`)
- Item separator token (`,`)

The scanners for said types of tokens are within the scanners module.

The parsing then processes the stream of tokens and starts to callback parts of the resulting
JSON structure as they are parsed.
