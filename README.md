# JessieScript

JessieScript (JSS) is a new interpreted language similar to 6502 Assembly, but uses a stack instead of traditional registries. This allows for quicker implementation of certain features and makes it easier to compile into.

## Getting Started

1. Clone this repository:
```
git clone git@github.com:alexburroughs/JessieScript.git
```

2. Create a Hello World program in `scripts`. The standard file extension is `.jss`:
```
push 1010101010
print
```

3. Run the program (using [rustup](https://rustup.rs/))
```
cargo run ./scripts/helloworld.jss
```

4. You're done! The output should be:
```
1010101010
```

## Contributing to JessieScript

Anyone can contribute to JessieScript. The JSS interpretor is written in [Rust](https://www.rust-lang.org). It is easy to understand and start adding features to. Please see our issues first to make sure your ideas don't already exist, or to see how you can contribute if you don't have any ideas.

## Possible Future Implementations

1. User Input
2. Print ASCII characters
3. Arrays (ref addresses)