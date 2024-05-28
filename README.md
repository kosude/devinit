<p align=center>
    <img src="resources/logo.svg" height=70>
</p>

Devinit is a command-line utility written in Rust that provides the ability to write templates for files and projects and interactively apply them to
make boilerplate less annoying.

See the `examples/` directory for examples of writing file and project templates.

A tracker for planned features can be found on [Trello](https://trello.com/b/QkX7i1P1/devinit).

> [!NOTE]
> If you use VS Code, official Devinit integration is available via the extension on the 
> [marketplace](https://marketplace.visualstudio.com/items?itemName=jack-bennett.devinit-vsc).


## Command-line usage examples

```bash
# Configure a file at path 'licence.txt' with the 'LicenceBlock' template,
# specifying values for the variables 'name' and 'year'.
$ devinit file -p licence.txt "LicenceBlock" -Dname="John Doe" -Dyear="2024"

# Process the template 'EditorConfig' and output the result to stdout.
$ devinit file --dry-run "EditorConfig"

# Create and populate a folder called 'dotnet-project' from the project
# template 'DotNet', specifying the variable 'projectName'.
$ devinit project -p dotnet-project/ "DotNet" -DprojectName="Utils"

# List all available templates on the system
$ devinit list

# List all variables found in the file template 'exampletpl' or its parents
$ devinit file --list-vars "exampletpl"
```


## User configuration

Devinit is configured using the `devinitrc.yml` file, which can be stored at either `~/.devinit/` or `~/.config/devinit/`. Otherwise, it can be
explicitly specified using the `--config` command-line option.

`devinitrc.yml` enforces a standard [YAML](https://yaml.org/) syntax. The following fields are required:

| Field                   | Value                                                          |
|-------------------------|----------------------------------------------------------------|
| `file_templates_loc`    | File template directory, relative to the configuration file    |
| `project_templates_loc` | Project template directory, relative to the configuration file |


### Configuring project templates

Any folder in the `project_templates_loc` directory containing a **`templaterc.yml`** file will be registered as a project template. This
configuration file is YAML, the same as the devinitrc file, and describes which files to process and where to output them.

| Field   | Value                                                                                                          |
|---------|----------------------------------------------------------------------------------------------------------------|
| `files` | Dictionary of output paths (relative to evaluated template output) mapped to inputs, e.g. `out.txt: input.txt` |


## Templating

A templating engine is provided to allow your configurations to use a simple format similar to [Jinja](https://jinja.palletsprojects.com/en/3.1.x/),
courtesy of the [Tera](https://keats.github.io/tera/) project - see their documentation for an overview of the syntax.


### Functions and filters

In addition to a multitude of built-in functions/filters provided by Tera (see docs [here](https://keats.github.io/tera/docs/#built-ins)), some extras
are also provided by Devinit, and are documented below.

---

#### FUNCTION `licence(id: String)`

Expand a software licence, where `id` is a valid [SPDX id](https://spdx.org/licenses/).

This function returns an object containing each component of the licence. The returned object is split up into the following elements (accessible by
dot or square bracket notation):
 - `name` - the licence name
 - `header` - the standard licence header\*
 - `text` - the complete licence body

A lot of licences have places to substitute values, like year and copyright holder - it's recommended to use the provided
[replace](https://keats.github.io/tera/docs/#replace) filter to handle these template-side.

*\*Not all licences support the header component (the MIT licence, for example), and attempting to access it on them will result in an error. In these
cases, use the guaranteed `text` field instead.*

```jinja
{% set gpl = licence(id = "GPL-2.0-only") -%}

{{ gpl.name }}
{{ gpl.header }}
{{ gpl.text }} {# A very long string #}
```

---

#### FILTER `wrap(len: i32)`

Break up the given string into lines of maximum length `len`, without breaking individual words.

```jinja
{{ gpl.text | wrap(len = 80) }}
```

---
