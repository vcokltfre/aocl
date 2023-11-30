# AOCL Docs

## Comments

Comments are written as follows:

```aocl
# This is a comment
```

## Data types

There are 4 literal data types in AOCL:

- `int` - integer
- `float` - floating point number
- `str` - string
- `bool` - boolean

These are written as you'd expect:

```aocl
# int
1
-1
0

# float
1.0
-1.0
0.0

# str
"Hello, world!"
"Hello\nworld!"

# bool
true
false
```

There is also one non-literal data type:

- `array` - ordered sequence of values

Note: arrays are accessible only through the standard library. There is no syntax for them.

## Variables

Variables are delcared by assigning a value to an identifier:

```aocl
a = 1
b = 2
c = a + b
```

There are 3 valid forms of assignment:

- Literal assignment (i.e. `a = 1`)
- BinOp assignment (i.e. `a = 1 + b`)
- Call assignment (i.e. `a = @foo:bar`)

Note: call assignment only works with non-void functions, as there is no void/null type.

## Function calls

Function calls are written as follows:

```aocl
@foo:bar <args>

# Examples
@foo:bar 1 2 3
@foo:bar 1 "2" true abc

c = @foo:bar 1 2 3
```

Functions that can be called in this way can only be defined in the standard library, via Rust.

Standard library documentation can be found [here](https://aocl.vco.sh/).

## Gotos

Gotos can be defined as follows:

```aocl
~label
```

And can be jumped to as follows:

```aocl
goto label # unconditional
goto label if <literal or identifier> == <literal or identifier> # conditional

# Examples
goto label
goto label if a == b
goto label if a == 1
```

You may also `call` a label, which will push the current instruction pointer to the stack, and jump to the label:

```aocl
call label
```

You may also `ret` from a call, which will pop the instruction pointer from the stack, and jump to it:

```aocl
ret
```

## Examples

### Hello, world!

```aocl
@io:println "Hello, world!"
```

### FizzBuzz

```aocl
i = 0

~loop

any = false

div3 = i % 3
goto not3 if div3 != 0

@io:print "Fizz"
any = true

~not3

div5 = i % 5
goto not5 if div5 != 0

@io:print "Buzz"
any = true

~not5

goto no_print_number if any == true

@io:print i

~no_print_number

@io:print "\n"

i = i + 1

goto loop if i < 100
```
