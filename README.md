# Fungus
Fungus is a [Befunge](https://esolangs.org/wiki/Befunge) interpreter which
accelerates runtime performance with a parsing and optimization stage.

> [!NOTE]
> Fungus is being rewritten and is not yet functional.

## Usage
Fungus is run from the command line:
```shell
fungus
```

A hello world message will be printed.

<!--
The source file at `<SOURCE>` will be loaded and interpreted as a Befunge
program.

### Arguments
| Argument   | Usage            |
| :--------- | :--------------- |
| `<SOURCE>` | Source file path |

A positional argument is expected for the path to the Befunge source file. The
source file must be formatted as UTF-8.

### Options
| Short | Long        | Usage                 |
| :---- | :---------- | :-------------------- |
| `-d`  | `--dump`    | Print pseudo-assembly |
| `-h`  | `--help`    | Print help            |
| `-V`  | `--version` | Print version         |

If the `--dump` flag is set, then the program will be printed as
pseudo-assembly instead of being interpreted.

If the `--help` or `--version` flag is set, then information will be printed
but no action will be performed.

## Dependencies
These libraries are used:
* [clap](https://crates.io/crates/clap) - Command line argument parsing
* [rand](https://crates.io/crates/rand) - Randomness for interpreting the `?`
  command
* [thiserror](https://crates.io/crates/thiserror) - Error handling attributes
-->

## Credits
These resources were used for implementation and testing:
* [Befunge Esolang Page](https://esolangs.org/wiki/Befunge)
* [Befunge-93 Specification](https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown)
* [Funge-98 Specification](https://catseye.tc/view/Funge-98/doc/funge98.markdown)
* [BedroomLan Befunge Interpreter](https://www.bedroomlan.org/tools/befunge-playground/)
