# qk

qk allows you to quickly create new projects using templates

## Example

Consider the following config (CONFIG_DIR/qk/qk.toml):

```toml
[templates.rs]
projects_dir = '/home/yourusername/projects/rs'
commands = ['cargo new $QK_PROJECT_NAME #{lib?}']
editor = 'vim'
```

> `/home/yourusername/projects/rs` should exist and be a directory.

> `CONFIG_DIR` depends on your operating system, for example on linux
> it is `~/.config/`

Now you can start a new rust project by typing `qk rs myproject`.

The command `cargo new myproject` will get executed in
`/home/yourusername/projects/rs`, creating a new cargo package in
`/home/yourusername/projects/rs/myproject`.
Notice how `$QK_PROJECT_NAME` was replaced by `myproject`, the commands are
run with a shell, on windows that is PowerShell and on linux it is
read from the environment variable `$SHELL` or if that is not set, the
default is `sh`. So in windows you would use `$Env:QK_PROJECT_NAME` instead.
These are the available environment variables:

- `$QK_PROJECT_NAME` the name of the project (`myproject`)
- `$QK_PROJECT_DIR` the directory of the project (`/home/yourusername/projects/rs/myproject`)
- `$QK_PROJECTS_DIR` the template's projects_dir (`/home/yourusername/projects/rs`)

After all the commands in the `commands` field are executed successfully,
the command in the field `editor` will get executed with `$QK_PROJECT_DIR` as
the argument, in this case opening vim in the directory of `myproject`.

The only required field is `projects_dir`, which is the directory where
new projects will be located (i.e. where the commands will get executed).
So you can also define a template like this:

```toml
[templates]
example = '/path/to/example'

# other templates...
```

And the only thing it will do is execute the editor in the project's dir.

See `--editor` help for information on what editor is used when not
specified. You can set a default editor in the config by adding
`editor = 'your_editor'` at the beggining of the config, for example:

```toml
editor = 'vim'

# your templates...
```

Next time you use `qk rs myproject` it will open the editor in the project's
dir without executing any of the commands in the `commands` field.

And finally, there is another thing in the command that we haven't
addressed yet: `#{lib?}`. This is a custom argument, `lib` is the name of
the argument and `?` means that this is a flag; so if you use
`qk rs myproject -- --lib`, the command executed will look like this:
`cargo new myproject --lib`. The `--` is required if a custom argument
starts with `-`. Check the wiki for more info on custom arguments.
