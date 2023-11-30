# Notes CLI
Notes CLI is a simple command-line interface (CLI) tool designed to streamline your note-taking experience through the creation of a customized note system. It introduces two fundamental building blocks: templates and subcommands, offering flexibility and extensibility.

# Installation
Install CLI Notes using Homebrew:
```
brew tap MaxHill/tap
brew install cli-notes
```

# Creating a new note
To create a new note, use the following command:
`note new name_of_note`

Explore all available options using the help command:
```
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
  -h, --help
          Print help
  -V, --version
          Print version
```
## Template usage
Let's consider an example template located at `~/.config/notes-cli/templates/simple.hbs` with the following content:
```
// ~/.config/notes-cli/templates/simple.hbs
Simple note created using {{template}}
```

To create a new note using this template, use the following command:
```
notes-cli new note-name --template simple
```
This command will generate a new note named note-name based on the specified 
simple template. Feel free to explore and create your own templates to suit 
your note-taking preferences.

## Note templates
Note templates are stored in the folder specified in 
`~/.config/notes-cli/config.toml`, defaulting to `~/.config/notes-cli/templates/`.


Each template receives the following object:
```
NewNote {
     template: String, // Name of template to be used
     name: String, // Name of the note
     name_template: String, // Template string for name
     now: String, // ISO8601 timestamp for when note is created
     meta: HashMap<String, String> // Merger of --meta-data flag and meta table in config
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
```
# ~/.config/notes-cli/config.toml
...
[subcommands]
# Search filenames with fzf and open in default editor
find = "cd {{config.notes_dir}} && open -e $(fzf {{flags}})"

```

## Subcommand tamplates
Each subcommand in the config file is a template-string that receives the following object

```
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
