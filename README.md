# bf3D [![Build & Test](https://github.com/ivabus/bf3d/actions/workflows/rust.yml/badge.svg)](https://github.com/ivabus/bf3d/actions/workflows/rust.yml)
> brainfuck in 3D space

## Instruction set

| Character | Meaning                                                                                                                                                                             |
|:---------:|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    `>`    | Move data pointer on Ox "to right".                                                                                                                                                 |
|    `<`    | Move data pointer on Ox "to left".                                                                                                                                                  |
|    `^`    | Move data pointer on Oy "up".                                                                                                                                                       |
|    `_`    | Move data pointer on Oy "down".                                                                                                                                                     |
|    `/`    | Move data pointer on Oz "deep"                                                                                                                                                      |
|   `\ `    | Move data pointer on Oz "shallow"                                                                                                                                                   |
|    `+`    | Increment the byte at the data pointer.                                                                                                                                             |
|    `-`    | Decrement the byte at the data pointer.                                                                                                                                             |
|    `.`    | Output the byte at the data pointer.                                                                                                                                                |
|    `,`    | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                        |
|    `[`    | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command. |
|    `]`    | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command. |

## License

bf3D is published under [MIT](./LICENSE) license.

## Running

1. `cargo install bf3d`
2. Create file with bf3d code
3. `bf3d <file>` where <file> is your bf3d code

## To Do

1. Shell mode
