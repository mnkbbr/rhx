# rhx

A small Rust CLI that generates a file of a given size filled with random bytes.
Optionally hides a word at a random position inside the file and prints its offset.

## Usage

    rhx <file_name> -<flag> <number> ["word"]

| Flag  | Unit      |
|-------|-----------|
| `-b`  | bytes     |
| `-kb` | kilobytes |
| `-mb` | megabytes |
| `-gb` | gigabytes |

## Examples

    rhx out.bin -mb 10
    rhx out.bin -kb 1 "secret"

The second command creates a 1 KB random file with "secret" hidden inside
and prints the offset where it starts.

## Build

    cargo build --release

## Check the hidden word

    grep -a "secret" out.bin