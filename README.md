# qk

qk allows you to quickly start new projects using templates

## Usage

```
qk template project
```

## Installation

Using cargo:

```
cargo install qk
```

## Example

Consider the following config:

```toml
[templates.rs]
projects_dir = '/home/yourusername/projects/rs'
commands = ['cargo new $QK_PROJECT_NAME #{lib?}']
editor = 'vim'
```

> The config is located in `~/.config/qk/qk.toml` on Linux
> and in `%appdata%\qk\config\qk.toml` on Windows.

> `/home/yourusername/projects/rs/` will be created if it does not exist.
> On Windows it would be `C:\Users\yourusername\projects\rs\`.

> `$QK_PROJECT_NAME` on Windows would be `$Env:QK_PROJECT_NAME`.

Now you can start a new rust project by typing `qk rs myproject`.

The command `cargo new myproject` will get executed in `/home/yourusername/projects/rs`, creating a
new cargo package in `/home/yourusername/projects/rs/myproject`. Notice how `$QK_PROJECT_NAME` was
replaced by `myproject`.

The commands are run with a shell, on windows it is PowerShell and on linux it is read from the
environment variable `$SHELL` or if not set, the default is `sh`. So in windows you would use
`$Env:QK_PROJECT_NAME` instead. You can also set the shell in the configuration file at the
beginning of the file or in a specific template, as you would with the editor option.

These are the available environment variables:

- `$QK_PROJECT_NAME` the name of the project (`myproject`)
- `$QK_PROJECT_DIR` the directory of the project (`/home/yourusername/projects/rs/myproject`)
- `$QK_PROJECTS_DIR` the template's projects_dir (`/home/yourusername/projects/rs`)

After all the commands in the `commands` field are executed successfully, the command in the field
`editor` will get executed with `$QK_PROJECT_DIR` as the argument, in this case opening vim in the
directory of `myproject`.

The only required field is `projects_dir`, which is the directory where new projects will be
located (i.e. where the commands will get executed). So you can also define a template like this:

```toml
[templates]
example = '/path/to/example'

# other templates...
```

And the only thing it will do is execute the editor in the project's dir.

See `--editor` help for information on what editor is used when not specified. You can set a
default editor in the config by adding `editor = 'your_editor'` at the beginning of the config, for
example:

```toml
editor = 'vim'

# your templates...
```

Next time you use `qk rs myproject` it will open the editor in the project's dir without executing
any of the commands in the `commands` field.

And finally, there is another thing in the command that we haven't addressed yet: `#{lib?}`. This
is a custom argument, `lib` is the name of the argument and `?` means it is a flag; so if you use
`qk rs myproject -- --lib`, the command executed will look like this: `cargo new myproject --lib`.
The `--` before the custom argument is required for arguments that start with `-`.

## Custom arguments

Custom arguments can be specified with `#{arg}` in a command of a template in the config and, when
calling `qk`, specified after `--` (positional arguments don't need to be after `--`).

- A number followed by a colon at the beginning makes it a positional argument:
`#{1:arg}`, `#{2:arg2}`, ...

- Just the name makes it an optional option: `#{arg}` (`--arg value`)

- Name followed by a comma and a single character adds a short version `#{arg,a}`
(`--arg value` or `-a value`)

- Only comma followed by a single character disables long version `#{,a}` (`-a
value`)

- `*` allows empty values: `#{arg*}` (`--arg value` or `--arg ""`),
`#{arg,a*}`, `#{,a*}`, `#{1:arg*}` ...

- `!` makes it required: `#{arg!}`, `#{1:arg!}`, `#{arg,a!}`, `#{,a!}`,
`#{arg!*}`, `#{arg,a*!}`, `#{,a!*}`, ...

- `?` makes it a flag: `#{arg?}` (`--arg`), `#{arg,a?}` (`--arg` or `-a`),
`#{,a?}` (`-a`), ...

### Example

`~/.config/qk/qk.toml`:

```toml
[templates.example]
projects_dir = '/path/to/example'
commands = ['echo hello #{1:name!} #{2:lastname} #{num} #{color,c}']
```

```
> qk example project
error: The following required arguments were not provided:
    <name>

USAGE:
     [OPTIONS] <name> [lastname]

For more information try --help
> qk example project john
$ echo hello john
hello john
> qk example project john -- --num 30
$ echo hello john  30
hello john 30
> qk example project john -- -c red
$ echo hello john   red
hello john red
> qk example project john -- -c red --num 10
$ echo hello john  10 red
hello john 10 red
> qk example3 project john -- -c red --num 10 doe
$ echo hello john doe 10 red
hello john doe 10 red
> qk example3 project -- --help


USAGE:
     [OPTIONS] <name> [lastname]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -c, --color <color>
        --num <num>

ARGS:
    <name>
    <lastname>
```
