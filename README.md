# Slide

A powerful command line calculator.

## Features

- Does math
- Arbitrary precsion integers
- Multiple precision floats
- Variables (builtin and user defined)
- Has a symbol for recalling the previous value
- Can do basic trigonometry
- Runs shell commands
- Strings (sorta)
- Powerful readline interface with keybindings and syntax highlighting (using [rustyline](https://github.com/kkawakam/rustyline))

### Sample

A (bad) example showing the entire language

```
<< (fn x<foo>{foo*2;1.0})+[0x2](0b10)
=> 4
<< x<let bar=[fn y<>{#}]+[2]>
=> 1
<< fn x<y>{y+2}-("FooBarBaz")*#*($"exit 5"+x<6>)
=> 256696426534446775444557840240
```

## Todo

- [ ] Precedence reparsing
- [x] Functions
- [ ] Infix operators
- [ ] More language features
- [ ] CAS?
