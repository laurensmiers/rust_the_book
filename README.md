Rust "The Book" exercices
=========================

A collection of the examples/exercices/... found in the rust tutorial "The Book", collected for my own reference.

Interesting commands
--------------------

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
