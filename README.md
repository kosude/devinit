<p align=center>
    <img src="resources/logo.svg" height=70>
</p>

Devinit is a command-line utility written in Rust that provides the ability to write templates for files and projects and interactively apply them to
make boilerplate less annoying.


## Command-line usage examples

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


## User configuration

Devinit is configured using the `devinitrc.yml` file, which can be stored at either `~/.devinit/` or `~/.config/devinit/`. Otherwise, it can be
explicitly specified using the `--config` command-line option.

`devinitrc.yml` enforces a standard [YAML](https://yaml.org/) syntax. The following fields are required:

| Field                   | Value                                                          |
|-------------------------|----------------------------------------------------------------|
| `file_templates_loc`    | File template directory, relative to the configuration file    |
| `project_templates_loc` | Project template directory, relative to the configuration file |


## Templating

A templating engine is provided to allow your configurations to use a simple format similar to [Jinja](https://jinja.palletsprojects.com/en/3.1.x/).
The added syntax can be seen below:

| Syntax      | Use case                         |
|-------------|----------------------------------|
| `{: ... :}` | Preprocessed directives          |
| `{{ ... }}` | Dynamic expressions              |
| `{# ... #}` | Comments                         |

In expressions, each token is interpreted as a string, and are space-separated - to include spaces in a single token, enclose it in double or single
quotation marks.


### Variables

Variables are read with dollar signs `$`. This can be done in expressions, either stand-alone or as part of a function parameter list:
```
foo            = {{ $foo }}
func(foo, bar) = {{ func $foo $bar }}
```


### Functions

Functions, as briefly mentioned before, are available in Devinit templates. This interface is **not** currently extensible - any additional functions
should be submitted to this repo as PRs *(an improved + extensible system could be developed in a future release, if I work on this project for
that long)*.

Some **built-in** functions are provided, and are documented below.


#### `licence <id> <element> [params]`

Expand a software licence, where `id` is a valid [SPDX id](https://spdx.org/licenses/). `params` is a variable list of parameters in `foo=bar` format,
where each key is replaced with its value in the licence text (for example, `year=2024` replaces instances of "\<year\>" in a licence body with
"2024").

`element` describes which part of the licence should be output, and is one of the following:
 - `name` - the licence name
 - `header` - the standard licence header, if available
 - `text` - the complete licence body


## Planned features

 - VS Code integration (extension)
 - Project templates
 - Function parameter validation (i.e. check no. of parameters passed to template funcs)
 - Line breaking - function to split a multi-line string down into max-length lines
 - Comment blocks - function to take a multi-line string and append a prefix (e.g. `//` before each line)
