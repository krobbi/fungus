# Fungus
**Fungus is being rewritten. This document may not be accurate.**

Fungus is a [Befunge](https://esolangs.org/wiki/Befunge) interpreter that
accelerates runtime with an initial compilation and optimization stage.

Fungus mostly targets the original Befunge-93 standard, with some differences:
* The playfield may be an arbitrary size.
* Characters are represented as Unicode code points, not ASCII bytes.
* Potentially self-modifying code is not allowed.
* To simplify the optimizer, using a command without enough parameters on the
stack is considered undefined behavior. No error will be reported for this.

# Usage
Fungus is run from the command line:
```shell
fungus <PATH>
```

Fungus will load the Befunge source file at `<PATH>` and interpret it as a
program.

## Arguments
| Argument | Usage            |
| :------- | :--------------- |
| `<PATH>` | Source file path |

The source file at `<PATH>` must be formatted as UTF-8.

## Options
| Short | Long        | Usage         |
| :---- | :---------- | :------------ |
| `-h`  | `--help`    | Print help    |
| `-V`  | `--version` | Print version |

If the `--help` or `--version` flag is set, then Fungus will print information
but not perform any action.

# Technical Details
Compiling Befunge can be challenging because Befunge is an esoteric programming
language that is intentionally designed to be difficult to compile.

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

Other commands are available for math, logic, stack manipulation, input, and
output.

## How to Compile Befunge
Despite its complexity, the instruction pointer has a representable state
(position, direction, and mode), and always starts in the same state. Given a
state and a playfield, it is possible to build a
[basic block](https://en.wikipedia.org/wiki/Basic_block) that performs an
instruction and exits into a set of possible next states.

The following algorithm can be used to compile a representation of the program:
1. Add the initial state to a set of unexplored states.
2. While there are unexplored states:
   1. Remove a state from the set of unexplored states.
   2. If there is no basic block for the removed state:
      1. Build a basic block for the state.
      2. Add the basic block's exit states to the set of unexplored states.

This reduces the entire program to a stack-based
[control-flow graph](https://en.wikipedia.org/wiki/Control-flow_graph), which
is much more machine-friendly, but won't give great performance immediately.
Almost every useful instruction has one or more useless jumps between them.
Luckily, this is fixed by the optimization stage.

## Optimizations
After the program has been compiled, multiple techniques are used to improve
runtime performance, memory usage, and analysis of `p` commands:
* Basic block merging - If a basic block's only entry point is an unconditional
jump from another basic block, it can be deleted and have its instructions and
exit appended to its predecessor.
* [Peephole optimization](https://en.wikipedia.org/wiki/Peephole_optimization)
\- Small windows of instructions (currently 2 or 3) are matched against
patterns to be replaced with more optimal instructions that produce the same
effect.
* Branch optimization - If a constant is pushed before an if branch, or if the
branch has equal branches, the condition can be popped and the branch can be
replaced with an unconditional jump. If a not instruction appears before an if
branch, the not instruction can be deleted and the branches can be swapped.
* [Jump threading](https://en.wikipedia.org/wiki/Jump_threading) - Basic block
exits that lead directly to an unconditional jump can be replaced with the
jump's target.
* [Dead code elimination](https://en.wikipedia.org/wiki/Dead_code_elimination)
\- Starting at the main entry point. Any basic blocks that are not reachable
are deleted.

An optimization may not produce ideal results immediately, but may help with
other optimizations. To produce a highly optimized program, the optimizations
are run in a loop until no more changes can be made.

It is important that these optimizations never change any of the program's
defined behaviors.

## The Hard Part
Compiling a program hits a roadblock when it comes to the nasty `g` command,
and the even nastier `p` command. These commands get and put characters to and
from the playfield. This means that Befunge programs can not only read their
own source code, but modify themselves at runtime.

Thanks to peephole optimization, some `g` and `p` commands can be associated
with constant positions. If only constant positions are used and the program
can never modify itself, then the commands can be simplified to accessing
static variables and the playfield can be discarded. This optimization has not
been implemented.

During compilation, it is possible to keep track of which positions in the
playfield may be executed as code, and which can never be reached. If a `p`
command writes to an arbitrary position, or a constant position in the code,
then the program may be self-modifying.

If a `p` command is self-modifying, then the program will need to be recompiled
after the write, with the entry point after the command. This is unimplemented,
so an error for self-modifying code is thrown instead.

This analysis depends on the optimization stage, so it should be a separate
stage after optimization.

# Dependencies
Fungus uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.

# License
Fungus is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
