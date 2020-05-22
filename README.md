Rust "The Book" exercices
=========================

A collection of the examples/exercices/... found in the rust tutorial "The Book", collected for my own reference.

Interesting commands
--------------------

### Creating ###

We can create a new package/application with:

```
$> cargo new <name_of_package>
```

An application has main.rs as its crate root.

We can also create a new library:

```
$> cargo new --lib <name_of_package>
```

A library has lib.rs as its crate root.

These are not mutually exclusive, we can have a library and application in the same package!

### Run ###

Running an application.

```
$> cargo run
```

### Compile ###

1. Debug
```
$> cargo build
```

2. Release
```
$> cargo build --release
```

3. Check

This is usefull in f.e. CI's to quickly check if our project can compile:
```
$> cargo check
```

### Documentation ###

Make and open the documentation in your favorite web browser:
```
$> cargo doc --open
```

### Format ###

To format the code:
```
$> cargo fmt
```

Interesting things to know
--------------------------

* A char is 4 bytes in Rust!

* Breaking with a value is only possible inside a 'loop' not a 'while/for' loop!
