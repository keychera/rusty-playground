# rusty

journey
- https://www.rust-lang.org/learn/get-started
    - download rustup-init.exe (64-Bit)
    - install vscode plugin: rust-lang.rust-analyzer
    - [cargo init](https://github.com/rust-lang/cargo/issues/1549)
    - cargo run?
        - error 
        ```shell
        Compiling rusty-playground v0.1.0
        error: linker `link.exe` not found
        |
        = note: program not found

        note: the msvc targets depend on the msvc linker but `link.exe` was not found

        note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.

        note: VS Code is a different product, and is not sufficient.

        error: could not compile `rusty-playground` (bin "rusty-playground") due to 1 previous error
        ```
    
        - ~~will this solve? https://stackoverflow.com/a/64121601/8812880~~ nope, solved with the next step
            - commenter said this is a workaround hmm, haven't tried this yet
    - I was not reading properly, I should install Visual Studio with "Desktop development with C++" as rustup-init.exe says
     
    ```shell
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
    Running `target\debug\rusty-playground.exe`
    Hello, Rusty world!
    ```

- next step, egui