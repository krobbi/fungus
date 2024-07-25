# Fungus
_Compiling the uncompilable._

# Contents
1. [About](#about)
   * [How to Compile Befunge](#how-to-compile-befunge)
   * [The Hard Part](#the-hard-part)
2. [License](#license)

# About
Fungus is a project that aims to be an implementation of
[Befunge](https://esolangs.org/wiki/Befunge) that speeds up runtime with an
initial compilation stage. This is a challenge because Befunge is an esoteric
programming language that is designed to be difficult to compile.

```befunge
<v"Hello, world!"+910
 >:#,_@
```

In Befunge, a program consists of a playfield
(a 2D grid containing the characters from the source code) with an imaginary
instruction pointer moving through it.

Starting in the top-left corner, and moving to the right, the instruction
pointer processes each character it encounters as a command
(unknown commands do nothing). The instruction pointer can change direction
(`^`, `v`, `<`, `>`), including conditionally (`_`, `|`, `?`), jump over the
next command (`#`), and even wrap around the playfield.

Additionally, the instruction pointer can toggle between command mode and
string mode (`"`). In string mode, the instruction pointer pushes characters it
encounters to the stack until it exits string mode.

There are other commands for math, logic, stack manipulation, input, and
output, but there are too many to list here.

## How to Compile Befunge
Despite its complexity, the instruction pointer has a representable state
(position, direction, and mode), and always starts in the same state. Given a
state and a playfield, it is possible to build a
[basic block](https://en.wikipedia.org/wiki/Basic_block) that performs an
instruction and exits into a set of possible next states.

The following algorithm can be used to build a representation of the program:
1. Add the initial state to a set of unexplored states.
2. While there are unexplored states:
   1. Remove a state from the set of unexplored states.
   2. Build a basic block for the state.
   3. For each state in the basic block's possible next states:
      1. If there is no basic block for the state:
         1. Add the state to the set of unexplored states.

This reduces the entire program to a finite state machine, which is much more
machine-friendly, but wouldn't give great performance if compiled right away.
Almost every useful instruction would have one or more useless jumps between
them. This would be a good place to start merging blocks together and applying
optimizations.

## The Hard Part
This strategy hits a roadblock when it comes to the nasty `g` command, and the
even nastier `p` command. These commands get and put characters from and to the
playfield. This means that Befunge programs can not only read their own source
code, but modify themselves at runtime.

The current plan is to just ignore these commands until the rest of the project
is implemented.

# License
Fungus is released under the MIT License:  
https://krobbi.github.io/license/2024/mit.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
