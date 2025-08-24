# todo

A humble to-do list manager written in Rust.



## Getting Started

These instructions will get you a copy of the project up and running
on your local machine for development and testing purposes. See
the Installing section for notes on how to deploy the project on a
live system.


### Prerequisites

Rust and Cargo are required to build and install this project. See
the [Rust website](https://www.rust-lang.org/) for more information.

## Building

### Compiling

Compiling the project should be as simple as running `cargo build` in the
root directory of the project.

### Installing

Installing the project should be similarly as simple, just run `cargo
install` in the root directory of the project.

## Usage

A quick reference of the command line interface can be obtained at any time
by running `todo help`. This help text is also provided here for completeness:
```
Usage: todo [OPTIONS] <COMMAND>

Commands:
  list      Lists all the tasks
  add       Adds a new task
  remove    Removes a task
  complete  Completes a task
  help      Print this message or the help of the given subcommand(s)

Options:
  -l, --location <FILE>        The location to sync tasks with
  -v, --verbosity <VERBOSITY>  Sets the log level
  -h, --help                   Print help
  -V, --version                Print version
```

Each subcommand also has a help text that you can read by using `todo help [SUBCOMMAND]`.

### Verbosity

The verbosity flag is a little bit different from normal. Instead of 
being a binary flag, it takes a string value from a predefined list.
The lower down the list, the more verbose the output will be.
- `error`
- `warn`
- `info`
- `debug`
- `trace`

Logging can be disabled entirely by setting the verbosity to `off`.

## Contributing

Please read
[CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426)
for details on our code of conduct and the process for submitting
pull requests to us.


## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions
available, see the
[tags on this repository](https://github.com/Melephas/todo/tags).


## Authors

* **Sam Miller** - *Initial work* -
  [Melephas](https://github.com/Melephas)

See also the list of
[contributors](https://github.com/Melephas/todo/graphs/contributors)
who participated in this project.


## License

This project is licensed under the BSD 3-Clause Clear License - see the
[LICENSE](LICENSE) file for details

