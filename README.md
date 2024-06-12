# Notes CLI
Notes CLI is a simple command-line interface (CLI) tool designed to streamline your note-taking experience through the creation of a customized note system. It introduces two fundamental building blocks: templates and subcommands, offering flexibility and extensibility.

# Development
## Run application
Run in development
```bash
cargo run -- [new|ls|..]
```

Create note passing alternative config file
```bash
notes-cli -c ./test-config
```

# Installation
Install CLI Notes using Homebrew:
```bash
brew tap MaxHill/tap
brew install notes-cli
```

#  Update
```bash
brew unlink notes-cli
brew install maxhill/tap/notes-cli
```

# Creating a new note
To create a new note, use the following command:
```bash
note new name_of_note
```

## More examples
Create note passing alternative config file
```bash
notes-cli -c ./test-config new note-name
```

Create note passing additional metadata that can be used in template/name:
```bash
notes-cli -m "name:max" -m "lastName:hill" new note-name 
# or
notes-cli --meta-data "name:max" --meta-data "lastName:hill" new note-name 
```

Create note passing additional metadata that can be used in template/name:
```bash
notes-cli -m "name:max" -m "lastName:hill" new note-name 
# or
notes-cli --meta-data "name:max" --meta-data "lastName:hill" new note-name 

# or in json format
notes-cli --meta-data-json "{\"name\": \"max\"}" --meta-data-json "{\"lastName\":\"hill\"}" new note-name 

# or in one json object
notes-cli --meta-data-json "{\"name\": \"max\", \"lastName\":\"hill\"}" new note-name 
```

All of the above can also be passed to the new sub command:
```bash
notes-cli new note-name -m "name:max" -m "lastName:hill"
```



Explore all available options using the help command:
```bash
$ notes-cli new --help
Usage: notes-cli new [OPTIONS] <name>

Arguments:
  <name>  Name of the note

Options:
  -t, --template <TEMPLATE_NAME>
          Handlebars template file to be used. Ex. given template file: ./template/my-template.hbs Flag should look like this: --template my-template
  -n, --name-template <TEMPLATE_STRING>
          Handlebars template string for name. Ex. --name_template {{date now}}_{{name}}
  -m, --meta-data <KEY:VALUE>
          Key value to be passed to template. Ex. --meta-data name:John
      --meta-data-json <json>
          Key value to be passed to template in json format. Ex. --meta-data-json "{"name": "John"}"
  -h, --help
          Print help
  -V, --version
          Print version
```

## Template usage
Let's consider an example template located at `~/.config/notes-cli/templates/simple.hbs` with the following content: `Simple note created using {{template}}`

To create a new note using this template, use the following command:
```bash
notes-cli new note-name --template simple
```

This command will generate a new note named note-name based on the specified 
simple template. Feel free to explore and create your own templates to suit 
your note-taking preferences.

## Note templates
Note templates are stored in the folder specified in 
`~/.config/notes-cli/config.toml`, defaulting to `~/.config/notes-cli/templates/`.

Each template receives the following object:
```rust
NewNote {
     template: String, // Name of template to be used
     name: String, // Name of the note
     name_template: String, // Template string for name
     now: String, // ISO8601 timestamp for when note is created
     meta: HashMap<String, String> // Merger of --meta-data,--meta-date-json flag and meta table in config
     config: { // Config object specified in config.toml
         note_file_type: String,
         editor: String,
         notes_dir: PathBuf, // Use like a string
         templates_dir: PathBuf, // Use like a string
         subcommands: {
           // Commands from config.toml
           "command-name": "command-template-string"
         },
         meta: {
           // Key value pairs from config and --meta-data flag
           "key": "Value"
         },
    },
}
```

### Date formatting:
Date formatting is achieved with the custom Handlebars helper called date. 

Formats supported are the following:
| Formats  |    Example    |
|----------|:-------------:|
| Iso8601  |  2023-01-01T16:00:000 |
| Rfc3339  |  2023-01-01T16:00:00.52Z   |
| Rfc2822  | Thu Nov 01 16:00:00 CET 2023 |
| [year]-[month]-[day] [hour]:[minute]:[second]  | 2023-01-01 16:00:00 |
| [year]-[month]-[day] [hour]:[minute]  | 2023-01-01 16:00 |
| [year]-[month]-[day] [hour]  | 2023-01-01 16 |

For formatting strign see: https://time-rs.github.io/book/api/format-description.html

Example usage:
```
Format with default format: {{date now}} 
Format with custom formatter: {{date now fmt="[day padding:zero]/[month padding:zero]/[year]"
Format with custom formatter alternative: {{date now fmt="[day]/[month]/[year repr:last_two]"}}
```

# Subcommands

Subcommands are encouraged for extended functionality. 
Define custom subcommands in the configuration file.

## Example subcommand 
```toml
# ~/.config/notes-cli/config.toml
...
[subcommands]
# Search filenames with fzf and open in default editor
find = "cd {{config.notes_dir}} && $EDITOR $(fzf {{flags}})"

```

## Subcommand tamplates
Each subcommand in the config file is a template-string that receives the following object
```rust
SubCommand {
    cmd: String, // Command template-string
    args: String, // Arguments and flags passed to the command
    config: { // Config object specified in config.toml
        note_file_type: String,
        editor: String,
        notes_dir: PathBuf, // Use like a string
        templates_dir: PathBuf, // Use like a string
        subcommands: {
          // Commands from config.toml
          "command-name": "command-template-string"
        },
        meta: {
          // Key value pairs from config and --meta-data flag
          "key": "Value"
        },
    }
}
```
