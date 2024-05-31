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

#### FUNCTION `lang_by_filename(filename: String)`

Assume a programming language ID from the specified filename. If `filename` contains an extension, that is checked - otherwise `filename` itself is
checked.

For a comprehensive list of possible language IDs, see the [`language_specifics.rs`](src/templater/language_specifics.rs) source file.

One example use case could be with the `file_name` [builtin](#built-in-variables) - this would give you the assumed file format of the output file,
when using the `--path` option.

If the filename does not conform to any expected languages, "unknown" will be returned instead.

---

#### FUNCTION `comment_by_lang(lang_id: String)`

Return an object containing comment syntax, depending on `lang_id`, which is any one of the language IDs returned by the
[lang_by_filename](#function-lang_by_filenamefilename-string) function. If `lang_id` isn't associated with any comment styles, then "unknown" is
returned instead.

The output object prefers comment blocks, and is in the order [ start, prefix, end ], where `start` and `end` should be on their own lines and
`prefix` should be before each line of commented text.

For example, if the output is equal to ["#", "##", "###"], it is representing comment block syntax that looks like this:
```py
#
## comment block
###
```

See the [copyright](examples/templates/file/copyright) example for a practical use case.

---

#### FILTER `wrap(len: i32)`

Break up the given string into lines of maximum length `len`, without breaking individual words.

```jinja
{{ gpl.text | wrap(len = 80) }}
```

---


### Built-in variables

Devinit provides built-in variables that are evaluated behind-the-scenes at render time. They are all contained in the `BUILTIN` object, so they
can be accessed with dot notation (`BUILTIN.foo`) or square bracket notation (`BUILTIN["foo"]`).


| identifier       | Value                                                                                                              |
|------------------|--------------------------------------------------------------------------------------------------------------------|
| `file_name`*     | If `--path` is used - the name of the output file the template is being rendered to - otherwise, an empty string.  |
| `file_contents`* | If `--path` is used, and it directs to an already-existing file, this contains the contents of that existing file. |

\*Not available in project templates.
