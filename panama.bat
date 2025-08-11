cargo clean
cargo build

jextract --include-dir target/headers --output java/src --target-package self.chera --library rustyplayground target/headers/lib.h --use-system-load-library

javac --source=22 -d java/classes java/src/self/chera/*.java

java --enable-preview --enable-native-access=ALL-UNNAMED -Djava.library.path=./target/debug -cp ./java/classes HelloRustFromJava.java
