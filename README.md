# rmjunk

rmjunk is a tool to remove junk files such as `.DS_Store` and `Thumbs.db` from directories.

## Features

- Removes junk files from a specified directory.
- Option to search and remove junk files recursively within directories.
- Offers a dry-run mode to preview files that will be removed without actually deleting them.

## Installation

```bash
cargo install rmjunk
```

## Usage
Basic usage:

```bash
rmjunk [DIR]
```

## Options:

- `-r`, `--recursive`: Search and remove junk files from directories recursively.
- `--dry-run`: Preview files that will be removed without actually deleting them.

## Examples

Remove junk files from a specific directory:

```bash
rmjunk ./my_folder
```

Search and remove files recursively:

```bash
rmjunk -r ./my_folder
```

Use the dry-run mode:

```bash
rmjunk --dry-run ./my_folder
```

## License
MIT OR Apache-2.0

