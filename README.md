# Dementia: A brainf**k interpreter

Oh? Don't know how to write brainfuck? I'll tell you something

Brainfuck is only comprised of 8 commands (not including the ones i'll add)
: `+<[]>-`

The way brainfuck works is:

There is a single **pointer**. pointing to specific **cell**

Heres the format ill use to describe stuff:

```
[0] [0] [0] [0] [0] [0]
 ^
```

In this case all 6 cells are null. And the **pointer** is poiting to the first cell

I will now tell you what each command does

`">"`:
```
    All the cells are at their default 0 value and the {pointer} is pointing to cell[0]:
    [0] [0] [0] [0] [0] [0]
     ^
    when the interpreter finds a ">" it will increment the {pointer} by one.
    when a single ">" is executed it will now look something like this:
    [0] [0] [0] [0] [0] [0]
         ^
    As you can see. The {pointer} is incremented to the left by one cell (it is now at cell 0)
```
`"<"`:
```
    Same case with ">"
    All the cells are at their default 0 value and the {pointer} is pointing to cell[1]:
    [0] [0] [0] [0] [0] [0]
         ^
    when the interpreter finds a "<" it will now look something like this:
    [0] [0] [0] [0] [0] [0]
     ^
```

# TODO: finish this doc
