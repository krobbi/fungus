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
fungus [OPTIONS] <PATH>
```

The source source file at `<PATH>` will be loaded and interpreted as a Befunge
program.

## Arguments
| Argument | Usage            |
| :------- | :--------------- |
| `<PATH>` | Source file path |

The source file at `<PATH>` must be formatted as UTF-8.

## Options
| Short | Long        | Usage                 |
| :---- | :---------- | :-------------------- |
| `-d`  | `--dump`    | Print pseudo-assembly |
| `-h`  | `--help`    | Print help            |
| `-V`  | `--version` | Print version         |

If the `--dump` flag is set, then the program will be printed as
pseudo-assembly instead of being interpreted.

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

### Jump Threading
When Befunge programs with converging branches are parsed, multiple basic
blocks can lead into a single basic block containing only an unconditional
jump. Any labels targeting these basic blocks can be redirected to the jump's
target.

This step affects the macro-scale structure of the program's basic blocks, so
it is run early, after basic block merging.

### Dead Code Elimination
Redirecting labels in the jump threading step can cause some basic blocks to
become unreachable. A
[dead code elimination](https://en.wikipedia.org/wiki/Dead-code_elimination)
step is run after the jump threading step to remove these basic blocks.

The easiest way to find if a basic block is unreachable is to find the set of
all reachable basic blocks and compare it with this set. The following
algorithm is used to find the set of all reachable basic blocks:
1. Add the main entry point label to a set of pending labels.
2. While there are pending labels:
   1. Remove a label from the set of pending labels.
   2. If the label is already in the set of reachable labels:
      1. Skip this label and continue to the next one.
   3. Add the label to a set of reachable labels.
   4. Add the labels from the labeled basic block's exit point to the set of
      pending labels.

Any basic blocks that are not labeled by the set of reachable labels can be
safely removed.

### Peephole Optimization
[Peephole optimization](https://en.wikipedia.org/wiki/Peephole_optimization) is
a technique where small windows (peepholes) of instructions are replaced with
more optimal equivalents.

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

#### Instruction Folding
Fungus is allowed to have a more complex instruction set than Befunge supports.
This allows sequences of instructions to be folded into a single, more
specialized instruction. For example, `1.` is folded into a single instruction
that prints `"1 "` without touching the stack.

#### Instruction Bubbling
Stack operations (instructions that have stack effects but no side effects) and
statements (instructions that have side effects but no stack effects) can be
swapped so that statements come first. This is mostly done to unblock other
optimizations.

### Jump to Exit Optimization
Unconditional jumps to basic blocks containing only an exit point can be
replaced with the exit point if it is more optimal:
* Program endings are 'lighter' than jumps, so these will always replace them.
* Conditional branches will replace jumps if the instructions before the jump
  contribute to branch optimization.

### Branch Optimization
If a conditional branch is taken with equal branches or a constant value on the
top of the stack, then the condition can be popped and the branch can be
replaced with anunconditional jump. If a conditional branch follows a not
instruction (`!`), then the not instruction can be removed and the branches can
be swapped.

# Self-Modifying Code
Befunge can get values from the playfield with the `g` command and put values
to the playfield with the `p` command. The program counter should respond to
these changes, meaning that Befunge supports self-modifying code.

In practice, self-modifying code is avoided and the playfield is used for
storing static variables. Fungus checks for constant position arguments to `g`
and `p` commands and verifies that changes made by `p` commands can't later be
reached by the program counter. If a `p` command is self-modifying, then Fungus
supports the worst-case scenario by recompiling the program at the state
following the `p` command.

There are plans to create a lower-level representation of the program that only
accepts constant positions for `g` and `p` commands and reduces them to static
variables, but this is not yet implemented.

# Credits
Fungus uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [rand](https://crates.io/crates/rand) - Randomness for interpreting the `?`
  command.

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
