# devinit


## Templating language format

Devinit's templating engine enforces a similar format to [Jinja](https://jinja.palletsprojects.com/en/3.1.x/).

| Syntax      | Use case                         |
|-------------|----------------------------------|
| `{: ... :}` | Preprocessed directives          |
| `{% ... %}` | Evaluated statements (no output) |
| `{{ ... }}` | Variable output                  |
| `{# ... #}` | Comments                         |
