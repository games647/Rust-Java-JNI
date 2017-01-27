# Rust-Java-JNI

## Description

This is a experimenting project on how to connect 
[Rust](https://www.rust-lang.org/) and [Java](http://www.oracle.com/technetwork/java/javase/downloads/index-jsp-138363.html) 
together using the JNI API. So that you can execute native code from Java.

This repository can be used as reference guide on how to get started.

## How to reproduce

1. Add java methods with a native identifier
2. Run `javah class-file` or  `javah -classpath path-to-jar files class-name`

    path-to-jar could also contain multiple jars (`javah -classpath target/* Test`)
        
    class-name only contains the fully qualified class name without the `.class` extension
3. Call your rust methods like the ones in the generated header file
4. Compile the rust binaries into a library
5. Load the library into Java using System.load(LIBRARY_PATH)
6. Now you can access the native methods

### How to compile and run

1. Rust project: `cargo build` > output into target/
2. Java `mvn install` using [Maven](https://maven.apache.org/) (included in Eclipse, IntelliJ, Netbeans)
3. Place the library file (.dll, .so) next to the jar file java/target/
4. Run the jar file `java -jar jar-file`

## Other useful documents

* [Compile Rust-JNI to small library files](http://blog.ragozin.info/2016/07/rust-jni-java.html)
* [First steps with Rust and JNI](http://nitschinger.at/First-Steps-with-Rust-and-JNI/)
* [Java General JNI Documentation](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html)

## Supported OS

* Windows (.dll)
* Linux (.so)
* Mac (.so and .dylib)

## Planning

* More complex examples
* Complete `JNINativeInterface` in Rust according to the `jni.h file`

## Contribution

This project is open for contributions of any kind. If you find a better example, description, [...],
just make a pull request. 

This project is licensed with the unlicense, so are free to do whatever you want. Including using
it in your own project without credits.
