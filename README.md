<p align=center>
    <img src="resources/logo.svg" height=70>
</p>

Devinit is a command-line utility written in Rust that provides the ability to write templates for files and projects and interactively apply them to
make boilerplate less annoying.


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
