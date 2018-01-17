[![Build Status](https://travis-ci.org/singaraiona/mia.svg?branch=master)](https://travis-ci.org/singaraiona/mia)
# MIA

Concurrent LISP dialect with vectorization and concurrency under the hood.

## Run MIA's REPL on Linux/MacOS

```
git clone git@github.com:singaraiona/mia.git
cd mia
cargo run --release --bin repl
```

## Running the tests

```
cd mia
cargo test --test mia
```

## Examples

### ariphmetic:
```
(setq res (+ 1 2 3 4))
res
-> 10
```

### loop:
```
(setq cond T)
(while cond 
  (prinl "step: " @)
  (setq cond NIL))
step: T
-> NIL
```

For more examples see examples/ and tests/ directories.

## Authors

[Anton Kundenko](https://github.com/singaraiona)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* http://picolisp.com - simple and lightweight LISP dialect.
* http://kparc.com - 6th version of K vector language.
* https://github.com/Chymyst/chymyst-core - Declarative concurrency in Scala.
