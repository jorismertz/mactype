# MacType

MacType is a utility that brings the MacOS-style keyboard shortcuts for typing special characters, like ë, é, ä, and more, to X11 Linux environments. Designed for users accustomed to MacOS's typing shortcuts, MacTypeX seamlessly integrates this functionality into Linux, enhancing productivity and typing fluency for cross-platform users.

## Features

- MacOS Keyboard Shortcuts: Use familiar MacOS shortcuts to type special characters on Linux.
- Wide Range of Characters: Supports a comprehensive list of special characters, including accents, tildes, umlauts, and more.

## Installation

### Prerequisites

    Linux operating system with X11
    Rust and Cargo (for building from source)

## Building from Source

### Clone the repository:

```sh
git clone https://github.com/jorismertz/mactype.git
cd mactype
```

### Build the application:

```sh
cargo build --release

The executable will be located in ./target/release/. You can move it to a convenient location:
```

```sh
cp ./target/release/mactype ~/bin/ # or any other directory in your PATH
```

### Ussage

You can start using mactype by simple running the executable. For convience you could add this to your window managers startup script.

```sh
./target/debug/mactype &
```

## Special Character Shortcuts

When typing uppercase letters append a shift after the leader, for example: `Alt + e` then `Shift + e` will result in `È`

- **E**

  - `Alt + E` then `E`: é (É for uppercase)
  - `Alt + BackQuote` then `E`: è (È for uppercase)
  - `Alt + I` then `E`: ê (Ê for uppercase)
  - `Alt + U` then `E`: ë (Ë for uppercase)

- **U**

  - `Alt + E` then `U`: ú (Ú for uppercase)
  - `Alt + BackQuote` then `U`: ù (Ù for uppercase)
  - `Alt + I` then `U`: û (Û for uppercase)
  - `Alt + U` then `U`: ü (Ü for uppercase)

- **A**

  - `Alt + E` then `A`: á (Á for uppercase)
  - `Alt + BackQuote` then `A`: à (À for uppercase)
  - `Alt + I` then `A`: â (Â for uppercase)
  - `Alt + U` then `A`: ä (Ä for uppercase)
  - `Alt + N` then `A`: ã (Ã for uppercase)
  - `Alt` then `A`: å (Å for uppercase)

- **I**

  - `Alt + E` then `I`: í (Í for uppercase)
  - `Alt + BackQuote` then `I`: ì (Ì for uppercase)
  - `Alt + I` then `I`: î (Î for uppercase)
  - `Alt + U` then `I`: ï (Ï for uppercase)

- **O**

  - `Alt + E` then `O`: ó (Ó for uppercase)
  - `Alt + BackQuote` then `O`: ò (Ò for uppercase)
  - `Alt + I` then `O`: ô (Ô for uppercase)
  - `Alt + U` then `O`: ö (Ö for uppercase)
  - `Alt + N` then `O`: õ (Õ for uppercase)
  - `Alt` then `O`: ø (Ø for uppercase)

- **Miscellaneous**
  - `Alt` then `C`: ç (Ç for uppercase)
  - `Alt + N` then `N`: ñ (Ñ for uppercase)
  <!-- - `Alt` then `S`: ß (Note: ß does not traditionally have an uppercase variant, though ẞ exists) -->
