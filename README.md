# Slide

A powerful command line calculator.

## Features

- Does math
- Arbitrary precsion integers
- Multiple precision floats
- Variables (builtin and user defined)
- Powerful readline interface with keybindings and syntax highlighting (using [rustyline](https://github.com/kkawakam/rustyline))

### Sample

A (bad) example showing the entire language

```
<< (fn x<foo>{foo*2;1.0})+[0x2](0b10)
=> 4
<< x<let bar=[fn y<>{#}]+[2]>
=> 1
```

## Todo

- [ ] Precedence reparsing
- [x] Functions
- [ ] Infix operators
- [ ] More language features
- [ ] CAS?
