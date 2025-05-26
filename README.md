# Fungus
Fungus is a [Befunge](https://esolangs.org/wiki/Befunge) interpreter that
accelerates runtime performance with a parsing and optimization stage.

Fungus mostly targets the original Befunge-93 standard, with some differences:
* The playfield may be an arbitrary size.
* The values stored in the playfield are signed integers and are not limited to
being valid characters.
* Characters are represented as Unicode scalar values, not ASCII characters.
<!--* Potentially self-modifying code is not allowed.-->

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

# About Befunge
Befunge is an esoteric programming language that is intentionally designed to
be difficult to compile:
```befunge
<v"Hello, world!"+910
 >:#,_@
```

In Befunge, a program consists of a playfield
(a 2D grid containing the characters from the source code) with an imaginary
program counter moving through it.

Starting in the top-left corner, and moving to the right, the program counter
processes each character it encounters as a command. The program counter can
change direction (`^`, `v`, `<`, `>`), including conditionally (`_`, `|`, `?`),
jump over the next command (`#`), and even wrap around the playfield.

Additionally, the program counter can toggle between command mode and string
mode (`"`). In string mode, the program counter pushes characters it encounters
to the stack until it exits string mode.

Other commands are available for math, logic, stack manipulation, input, and
output.

# Playfield Stage
The first stage of Fungus is to take the one-dimensional string of source code
characters and convert it to a rectangular grid of integers that can be looked
up by 2D coordinates (the 'playfield'.)

The source code is split into lines, with trailing empty lines ignored. The
height of the playfield in cells is the number of lines, with a minimum of 1.
The width of the playfield in cells is the length of the longest line in
characters, with a minimum of 1.

Tabs are counted as a single character, so they should not be used for aligning
Befunge code.

The playfield is filled with `32` (the space character as an integer.) The
lines of source code are converted from characters to integers and superimposed
over the playfield. This results in lines that are shorter than the longest
line being padded with spaces.

The playfield should always be a rectangle and should always have a size of at
least 1x1.

# Parsing Stage
The program could easily be interpreted using only the playfield, but a lot can
be done to improve performance. To enable these optimizations, the playfield is
parsed into a more conventional
[control-flow graph](https://en.wikipedia.org/wiki/Control-flow_graph).

Parsing a Befunge program is similar to interpreting it. The program counter
starts in a known state
(in the top-left corner, facing right, in command mode.) The program counter is
followed, and an action is performed based on its mode and the command it
points to. Instead of executing commands,
[basic blocks](https://en.wikipedia.org/wiki/Basic_block) are created that
contain instructions and an exit point that may lead to more program counter
states. When conditional branches are encountered, all possible branches are
followed. States that have already been parsed do not need to be parsed again.

The following algorithm is used for parsing the program:
1. Add the initial program counter state to a set of unvisited states.
2. While there are unvisited states:
   1. Remove a state from the set of unvisited states.
   2. If there is no basic block for the removed state:
      1. Parse a basic block for the state.
      2. Add the basic block's exit states to the set of unvisited states.

The resulting graph is much easier for Fungus to analyze, but has poor
runtime performance and memory usage, since every command is separated by
mostly useless jumps. This issue will be addressed in an optimization stage.
<!-- TODO: Change previous sentence to present tense. -->

<!--
# Optimization Stage
After the graph has been parsed, multiple techniques are used to reduce its
complexity and improve its eventual runtime performance:
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

# The Hard Part
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
-->

# Credits
Fungus uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.

The following resources were helpful for implementing Fungus:
* [Befunge Esolang Page](https://esolangs.org/wiki/Befunge)
* [Befunge 93 Specification](https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown)
* [Funge 98 Specification](https://codeberg.org/catseye/Funge-98/src/branch/master/doc/funge98.markdown)
\- Funge 98 is not implemented by Fungus, but this specification is more clear
than the Befunge 93 specification.
* [BedroomLan Befunge Interpreter](https://www.bedroomlan.org/tools/befunge-playground/)
\- Not fully compliant with Befunge 93, but useful for testing.

# License
Fungus is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
