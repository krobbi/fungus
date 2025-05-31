# Fungus
Fungus is a [Befunge](https://esolangs.org/wiki/Befunge) interpreter that
accelerates runtime performance with a parsing and optimization stage.

Fungus mostly targets the original Befunge-93 specification with some
differences:
* The playfield may be an arbitrary size.
* The values stored in the playfield are signed integers and are not limited to
  being valid characters.
* Characters are represented as Unicode scalar values, not ASCII characters.

# Usage
Fungus is run from the command line:
```shell
fungus <PATH>
```

The source source file at `<PATH>` will be loaded and interpreted as a Befunge
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

# Preprocessing Stages
Fungus processes Befunge programs in multiple stages before they are
interpreted:

## Playfield Stage
The first stage is to take the one-dimensional string of source code characters
and convert it to a rectangular grid of integers that can be looked up by 2D
coordinates (the 'playfield'.)

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

## Parsing Stage
The program could easily be interpreted using only the playfield, but a lot can
be done to improve performance. To enable these optimizations, the playfield is
parsed into a more conventional
[control flow graph](https://en.wikipedia.org/wiki/Control-flow_graph).

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

The resulting graph is much easier to analyze and interpret, but has poor
runtime performance and memory usage, since every command is separated by
mostly useless jumps. This issue is addressed in the optimization stage.

## Optimization Stage
After a program has been parsed, multiple optimization algorithms are applied
to reduce its complexity and improve its eventual runtime performance.

Optimization steps will not produce optimal results on their own, but their
effects may allow other steps to have a better effect. To take advantage of
this, every optimization step is run in a loop until no changes can be made.

### Basic Block Merging
Basic blocks with a single entry point from an unconditional jump can be merged
into their predecessor. This involves deleting the basic block and appending
its instructions and exit to the predecessor.

This significantly reduces the number of basic blocks and groups instructions
together, making the program smaller and easier to analyze. Because of this,
the basic block merging step is run first.

The following algorithm performs basic block merging on the first eligable
predecessor:
1. For each basic block in the program (the predecessor:)
   1. If the exit point is not an unconditional jump to a different successor:
      1. Skip this predecessor and continue to the next one.
   2. If the successor has any other predecessors:
      1. Skip this predecessor and continue to the next one.
   3. Remove the successor.
   4. Append the successor's instructions to the predecessor's instructions.
   5. Replace the predecessor's exit point with the successor's exit point.
   6. Break out of the loop.

The optimizer repeats this step until no more basic blocks can be merged.

### Peephole Optimization
[Peephole optimization](https://en.wikipedia.org/wiki/Peephole_optimization) is
a technique where small windows (peepholes) of instructions are replaced with
more optimal equivalents. Peephole optimization is effective for miscellaneous
'cleanup' tasks that don't require much analysis.

The optimizer uses pattern matching to detect various optimization cases:

#### No-ops
A no-op is a sequence of instructions that have no overall effect. These
patterns can be completely removed:
* Push and pop (`0$`, `:$`) - Pushing a value with no side effects before
  popping it has no effect.
* Swap and swap (`\\`) - Swapping the top two values of the stack twice results
  in the same stack.

#### Constant Folding
Push and operate (`12+`) can be replaced with pushing the result of the
operation (`3`.)

#### Push and Duplicate
Push and duplicate (`1:`) can be replaced with pushing the value twice (`11`.)

#### Push and Swap
Push and swap (`12\`) can be replaced with pushing the values in reverse order
(`21`.)

#### Duplicate and Swap
Duplicate and swap (`:\`) can be replaced with duplicate (`:`.) Duplicating the
top value of the stack has no side effect and results in the top two values of
the stack being equal. If the top two values of the stack are equal, then
swapping them has no effect.

#### Operate and Pop
Operations with no side effects followed by a pop (`!$`, `+$`, `g$`...) can be
replaced with popping the number of operands (`$`, `$$`.)

<!--
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
-->

<!--
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
* [Befunge-93 Specification](https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown)
* [Funge-98 Specification](https://catseye.tc/view/Funge-98/doc/funge98.markdown)
  \- Funge-98 is not implemented by Fungus, but it is a superset of Befunge-93
  with a more detailed specification.
* [BedroomLan Befunge Interpreter](https://www.bedroomlan.org/tools/befunge-playground/)
  \- Not fully compliant with Befunge-93, but useful for testing.

# License
Fungus is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
