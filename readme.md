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

### Setting up

There is only one extra step required to get the program working as expected,
other than compiling and installing. Setting up the configuration file to specify
where to store the tasks. The `config` subcommand can be used for this:

```shell
todo config "file:///home/<USER>/todos.ron"
```

The file can be named anything you like. Here it ends in `.ron` but this is not required.
See the Configuration section for more information.

## Building

### Compiling

Compiling the project should be as simple as running `cargo build` in the
root directory of the project.

### Installing

Installing the project should be similarly as simple, just run `cargo
install --path $PWD` in the root directory of the project.

## Usage

A quick reference of the command line interface can be obtained at any time
by running `todo help`. This help text is also provided here for completeness:

```
Usage: todo [OPTIONS] <COMMAND>

Commands:
  config    Create a new configuration
  list      Lists all tasks
  add       Adds a new task
  remove    Removes a task
  complete  Completes a task
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbosity <VERBOSITY>  Sets the log level
  -h, --help                   Print help
  -V, --version                Print version
```

Each subcommand also has a help text that you can read by using `todo help [SUBCOMMAND]`.

### Configuration

As of v0.1.6, there is a configuration file that can be used to specify the URL of the task storage.
There is only one location supported for the configuration file: `$HOME/.config/todo/config.toml`.

At present only one configuration option is supported: `storage`. A valid config file can point
to either a file:

```toml
storage = "file:///home/<USER>/.config/todo/default.todo.ron"
```

Or to a Postgres database:

```toml
storage = "postgresql://postgres:password@<HOST>:<PORT>/todo"
```

These examples are given for illustration purposes only and are not valid URLs.

New configuration files can be created easily with the `config` subcommand:

```shell
todo config "file:///home/<USER>/.config/todo/default.todo.ron"
```

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

This project is licensed under the BSD 3-Clause Clear Licence—see the
[LICENSE](LICENSE) file for details
