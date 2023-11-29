
# Create a new note

To create a new note use the command `note new name_of_note`.

```
cargo run -- new --help
   Compiling notes-cli v0.1.0 (/Users/maxhill/code/personal/cli-notes)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/notes-cli new --help`
Creates note from specified template

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

## Templating

Each template get's passed the following object:
```
NewNoteFrom {
     template: String, // Name of template to be used
     name: String, // Name of the note
     name_template: String, // Template string for name
     now: String, // ISO8601 timestamp for when note is created
     meta: HashMap<String, String> // Merger of --meta-data flag and meta table in config
     config: { // Config object specified in config.toml
         note_file_type: String,
         editor: String,
         notes_dir: PathBuf,
         templates_dir: PathBuf,
         subcommands: HashMap<String, String>,
         meta: HashMap<String, String>,
    },
}
```

### Date formatting:
Date formatting can be done with the custom handlebars helper called date.

For formatting strign see: https://time-rs.github.io/book/api/format-description.html

The `date` helper can parse the following formats:
| Formats  |    Example    |
|----------|:-------------:|
| Iso8601  |  2023-01-01T16:00:000 |
| Rfc3339  |  2023-01-01T16:00:00.52Z   |
| Rfc2822  | Thu Nov 01 16:00:00 CET 2023 |
| [year]-[month]-[day] [hour]:[minute]:[second]  | 2023-01-01 16:00:00 |
| [year]-[month]-[day] [hour]:[minute]  | 2023-01-01 16:00 |
| [year]-[month]-[day] [hour]  | 2023-01-01 16 |


Example usage:
```
Format with default format: {{date now}} 
Format with custom formatter: {{date now fmt="[day padding:zero]/[month padding:zero]/[year]"
Format with custom formatter alternative: {{date now fmt="[day]/[month]/[year repr:last_two]"}}
```
