# A Rust CLI tool: files organizer
This is a Rust CLI tool that can organize the files in a directory based on their extensions, creating subdirectories for different types of files. It takes one command line argument: the directory path.

## Project Setup:
1. clone the repo:
```
https://github.com/JuliaJHL/week9-rust-mini-proj.git
```
2. cd into the project:
```
cd week9-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run <directory_path>
```

## examples
* I created some files in `example` folder:

![before](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/before.png)
* I used `cargo run ./example` to organize the files:

![run](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/run.png)
* Here's the result:

![after](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/after.png)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
