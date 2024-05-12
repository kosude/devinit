<p align=center>
    <img src="resources/logo.svg" height=70>
</p>

Devinit is a command-line utility written in Rust that provides the ability to write templates for files and projects and interactively apply them to
make boilerplate less annoying.

## Command-line examples

```bash
# Configure a file at path 'licence.txt' with the 'LicenceBlock' template,
# specifying values for the variables 'name' and 'year'.
$ devinit file licence.txt "LicenceBlock" -Dname="John Doe" -Dyear="2024"

# Process the template 'EditorConfig' and output the result to stdout.
$ devinit file --dry-run "EditorConfig"

# Create and populate a folder called 'dotnet-project' from the project
# template 'DotNet', specifying the variable 'projectName'.
$ devinit project dotnet-project/ "DotNet" -DprojectName="Utils"
```


## Templating

A templating engine is provided to allow your configurations to use a simple format similar to [Jinja](https://jinja.palletsprojects.com/en/3.1.x/).
The added syntax can be seen below:

| Syntax      | Use case                         |
|-------------|----------------------------------|
| `{: ... :}` | Preprocessed directives          |
| `{{ ... }}` | Variable output                  |
| `{# ... #}` | Comments                         |


## User configuration

Devinit is configured using the `devinitrc.yml` file, which can be stored at either `~/.devinit/` or `~/.config/devinit/`. Otherwise, it can be
explicitly specified using the `--config` command-line option.

`devinitrc.yml` enforces a standard [YAML](https://yaml.org/) syntax. The following fields are required:

| Field                   | Value                                                          |
|-------------------------|----------------------------------------------------------------|
| `file_templates_loc`    | File template directory, relative to the configuration file    |
| `project_templates_loc` | Project template directory, relative to the configuration file |
