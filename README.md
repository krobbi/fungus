# Fungus
Fungus is a [Befunge](https://esolangs.org/wiki/Befunge) interpreter which
accelerates runtime performance with a parsing and optimization stage.

> [!NOTE]
> Fungus is being rewritten and is not yet functional.

## Usage
Fungus is run from the command line:
```shell
fungus <FILE>
```

### Arguments
| Argument | Usage            |
| :------- | :--------------- |
| `<FILE>` | Source file path |

A positional argument is expected for the path to the Befunge source file.
<!--The source file must be formatted as UTF-8. -->

### Options
| Short | Long        | Usage                 |
| :---- | :---------- | :-------------------- |
| `-h`  | `--help`    | Print help            |
| `-V`  | `--version` | Print version         |
<!--
| `-d`  | `--dump`    | Print pseudo-assembly |

If the `--dump` flag is set, then the program will be printed as
pseudo-assembly instead of being interpreted.
-->

If the `--help` or `--version` flag is set, then information will be displayed
but no action will be performed.

## Dependencies
These libraries are used:
* [clap](https://crates.io/crates/clap) - Command line argument parsing
* [thiserror](https://crates.io/crates/thiserror) - Error handling attributes

## Credits
These resources were used for implementation and testing:
* [Befunge Esolang Page](https://esolangs.org/wiki/Befunge)
* [Befunge-93 Specification](https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown)
* [Funge-98 Specification](https://catseye.tc/view/Funge-98/doc/funge98.markdown)
* [BedroomLan Befunge Interpreter](https://www.bedroomlan.org/tools/befunge-playground/)
