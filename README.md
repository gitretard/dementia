see `cargo run -- -cells 2 -debug -delay 100 src/bf`
see the t mutable variable in main()

## Dementia: A brainfuck interpreter (with extra commands!) and Some brainfuck tutorial


## Usage:
```
dementia [OPTIONS] TARGET
TARGET Must be the last argument
    -c      amount of cells
    -dly    delay in millisecs
    -h      help
    -dbg    debug output
EXAMPLE: dementia -c 10 -d 2000 brainfuck.bf
```

## Known issues
1. When using a variable to hold s.chars(). it returns None??????. I need to fix the performance issue Also WHY THE ACTUAL FUCK DOES .nth() REQUIRES THE FUCKING VARIABLE TO BE mut??????????. IT ALSO IGNORE SOME FUCKING CHARACTERS????
 
## Tutorial

Oh? Don't know how to write brainfuck? I'll tell you something

Brainfuck is only comprised of 8 commands (not including the ones i'll add)
: `+<[]>-`

The way brainfuck works is:

There is a single **pointer**. pointing to specific **cell**
Cells are Vectors of bytes (uint8)
Sticking with Vectors for loops ip

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
    After executing:
    [0] [0] [0] [0] [0] [0]
         ^
    As you can see. The {pointer} is incremented to the left by one cell (it is now at cell 1)
```
`"<"`:
```
    Same case with ">"
    All the cells are at their default 0 value and the {pointer} is pointing to cell[1]:
    [0] [0] [0] [0] [0] [0]
         ^
    After executing:
    [0] [0] [0] [0] [0] [0]
     ^
```
`"+"`:
```
    This will increment the cell the {pointer} is pointing at
    [0] [0] [0] [0] [0] [0]
     ^
    After executing:
    [1] [0] [0] [0] [0] [0]
     ^
```

`"-"`:
```
    This will decrement the cell the {pointer} is pointing to.
    Cell 1 is set to 10
    [0] [10] [0] [0] [0] [0]
         ^
    After executing:
    [0] [9] [0] [0] [0] [0]
         ^
```
`","`:
```
    This will accept 1 byte of input from stdin then put it to cell[ptr]
    [0] [0] [0] [0] [0] [0]
     ^
    After executing:
        stdin:
            input: A
    cell[ptr] will now be set to 65 (A in ASCII)
    [65] [0] [0] [0] [0] [0]
     ^
```
`"."`:
```
    This will print out cell[ptr] as a rust char:
    [65] [0] [0] [0] [0] [0]
     ^
    After executing:
        stdout:
            A
```

These are the hardest to use by far. Also i won't give any examples for these . They act like while loops and if statements at the same time

`"["`:
```
    If cell[ptr] is zero, then instead of incrementing the instruction pointer, jump it forward to the matching ].
```

`"]"`:
```
    If cell[ptr] != 0, then instead of incrementing the instruction pointer by one like normally, jump it back to the matching [.
```
### Any more commands after these are non standard. (I made them up and put them into the interpreter)

"`_`"
```
    Extended versoion of the standard output 
    Will print out anything ranging from ptr to the value in ptr 

    [2] [115] [33] [65] [0] [0]
    2 = Buffer size
    115 = s
    33 = !
    65 = A
    
    This will only print out "s!" due to the fact that the value inside the cell is only 2
    so it will look someting like
    [ptr..cell[ptr]] 
```


`"/"`:
```
    Extended version of the normal input.
    Will create a buffer with the size being the value of the current cell and copy them into the cells array
    This will create a read buffer of 4 bytes in size
    [3] [0] [0] [0] [0] [0]
     ^
     After executing:
        Input: r!
    The cells array will now look something lie kthis:
    [3] [114] [33] [10] [0] [0]
    2 = Buffer size
    114 = r
    33 = !
    10 = NEWLINE

    But if we give the same input but this time the buffer size is only 1.
    
    [1] [114] [0] [0] [0] [0]
```

"`~`"'
```
    Resets ptr
```

`";"` : inline comments (like asm)

### Hello, World! 


First we need to know how to multiply numbers (standard commands only!)

For example: 9x9

`+++++++++[>+++++++++<-]>.`

This will output `Q (81)`

The resulting cell array will look like this:

`[0, 81, 0, 0, 0, 0, 0, 0, 0, 0]`

I will now explain in as much detail as possible in how this works:

`+++++++++`: This will set cell[0] to 9

`[>+++++++++`: This will go to the next cell (cell 1). And add 9 to the cell

`<-]`: This will go back to the previous cell and subtract 1 from it. Then loop back to `[` if cell 0 isnt 0

The `]` will exit if cell[ptr] is 0. hence  `<-]` instead of `<->]` or something else

`>.`: This will go to the next cell and prints its contents in ASCII



Okay i will now explain this in simple english:

This will add 9 to cell 1 and, subtract 1 from cell 0 every time it loops. essentially making them multiply

This trnslate to:

```
cell[ptr]+=9
while cell[ptr] != 0{
    // Pretty darn sure this isn't valid C syntax. but i hope you get it
    ptr++;
    cell[ptr]+=9;
    ptr--;
    cell[ptr]--;
}
ptr++
printf("%s",cell[ptr]);
```

So I sincerely hope that you can now comprehend what `+++++++++[>+++++++++<-]>.` does. if not? Sleep already and come back at it tomorrow. Trust me

```
ASCII Chars needed:
H: 72
e: 101
l: 108
o: 111
,: 44
SPACE: 32
W: 87
r: 114
d: 100
!: 33
```

Now. Time to Actually print hello world. Here's how:

We have 2 options. 
1. Use a minimum of 2 bytes (clear cell 1 every for every char we print) OR
2. Actually make use of the space we have (26 bytes minimum)

Y'know what i choose 1

Heres's the code to print "Hello, World!" by multiplying (i hand wrote this myself at midnight)

```
++++++++++[>+++++++<-]>++.<             ; cell[1] += (10 x 7) x 2 (H)
++++++++++[>-------<-]>--<              ; cell[1] -= (10 x 7) x 2 
++++++++++[>++++++++++<-]>+.<           ; cell[1] += (10 x 10) + 1 (e)
++++++++++[>----------<-]>-<            ; cell[1] -= (10 x 10) + 1
++++++++++[>+++++++++++<-]>--.<         ; cell[1] += (10 x 11) + 2 (l)
++++++++++[>----------<-]>--------<     ; cell[1] -= (10 x 11) - 2
++++++++++[>+++++++++++<-]>--.<         ; cell[1] += (10 x 11) + 2 (l)
++++++++++[>----------<-]>--------<     ; cell[1] -= (10 x 10) + 1
++++++++++[>+++++++++++<-]>+.<          ; cell[1] += (10 x 11) +1  (o)
++++++++++[>-----------<-]>- <          ; cell[1] -= (10 x 11) +1  
++++++++++[>++++<-]>++++.<              ; cell[1] += (10 x 4) + 4 (,)
++++++++++[>----<-]>----<               ; cell[1] -= (10 x 4) + 4
++++++++++[>+++<-]>++.<                 ; cell[1] += (10 x 3) + 2 (SPACE)
++++++++++[>---<-]>--<                  ; cell[1] -= (10 x 3) + 2
++++++++++[>+++++++++<-]>---.<          ; cell[1] += (10 x 9) - 3 (W)
++++++++++[>--------<-]>-------<        ; cell[1] -= (10 x 8) - 7 
++++++++++[>+++++++++++<-]>+.<          ; cell[1] += (10 x 11) + 1  (o)
++++++++++[>-----------<-]>- <          ; cell[1] -= (10 x 11) - 1
++++++++++[>+++++++++++<-]>++++.<       ; cell[1] += (10 x 11) + 4 (r)
++++++++++[>-----------<-]>----<        ; cell[1] -= (10 x 11) - 4
++++++++++[>+++++++++++<-]>--.<         ; cell[1] += (10 x 11) + 2 (l)
++++++++++[>----------<-]>--------<     ; cell[1] -= (10 x 11) - 2
++++++++++[>++++++++++<-]>.<            ; cell[1] += 10 x 10 (d)
++++++++++[>----------<-]>.<            ; cell[1] -= 10 x 10
++++++++++[>+++<-]>+++.<                ; cell[1] += (10 x 4) + 4 (!)
++++++++++[>---<-]>---<                 ; cell[1] -= (10 x 4) + 4
++++++++++.                             ; LF
----------

```

Yeah i know. this looks unelegant as fuck (i needed to repeat myself). Also sorry for anything wrong. 

Because:

I wrote this 25 lines of abomination by hand at 1 AM and ill have school at 7:30 AM

So how do you run whatever this is?

`cargo run -- -cells 2 src/bf`

or if you want to know what is going on:

`cargo run -- -cells 2 -delay {whatever you like millisecs} -debug src/bf`

## TODO: finish this doc
