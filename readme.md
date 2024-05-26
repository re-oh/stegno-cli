Usage: stegno-cli.exe

Commands:

encode | Encodes data from the specified file into the specified PNG image
EXAMPLE: cargo run -- encode -e path/to/image.png -f path/to/data.txt

decode | Decodes data from the specified PNG image into a new file
EXAMPLE: cargo run -- decode -e path/to/image.png -o path/to/output.txt

help | Print this message or the help of the given subcommand(s)

Options:
-h, --help Print help
-V, --version Print version

quick commands:

encode "alamakota" into stegno_apple_test
cargo run -- encode -e src/stegno_apple_test.png -f src/secret.txt

decode a png file with no data
cargo run -- decode -e src/stegno_apple_test_plain.png -o src/output.txt

decode a file with data : output should be alamakota + a buch of fucked up utf-8
cargo run -- decode -e src/stegno_apple_test.png -o src/output.txt
