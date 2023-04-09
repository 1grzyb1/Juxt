# Juxt
Juxt is a powerful JavaScript-based template engine that allows you to embed JavaScript code within templates to generate any type of text. It provides a flexible and extensible way to create and manage dynamic content.

## Features
- Evaluate JavaScript code within templates
- Supports importing other tempaltes and js
- If/Else statements within templates
- Each operation within templates

## Getting Started
### Installation

Add the Juxt library to your project as a dependency.

### Usage

#### CLI

To use the Juxt command-line tool, you can provide the following arguments:

- `--path` (short: `-p`): The path from which the template will be read. By default, it is set to an empty string. Example: `--path /path/to/templates`
- `--main` (short: `-m`): The main template name. By default, it is set to "main.juxt". Example: `--main my-main-template.juxt`
- `--context` (short: `-c`): The context in JSON format to be passed to the template. By default, it is set to an empty JSON object (`{}`). Example: `--context '{"key": "value"}'`

Here's an example command that demonstrates how to use these arguments:

```bash
juxt -- --path /path/to/templates --main my-main-template.juxt --context '{"key": "value"}'
```

This command reads the template from the specified path, uses the main template named `my-main-template.juxt`, and passes a context containing a single key-value pair.

#### Library
To evaluate template you need to call `compile_and_execute` providing 
- `main` template which will be entrypoint
- `dependencies` - all other templates that main template depends on
- `context` - json object that will be template context

Additionally you can only compile templates for future use and improving performance.

To do that use `compile_juxt` method and later on pass result to `execute_js` with provided context.

### Example

Following template
```
{#script}
    function addOne(n) {
        return n + 1;
    }
{/script}
{#each n in [0, 1, 2]}
${addOne(n)}
{/each}
```

will be evaluated to:

```
1
2
3
```

## Syntax
Here's a summary of the [Syntax](syntax.md)

## Contributing
If you'd like to contribute to the Juxt project, please submit a pull request or open an issue on the project's GitHub repository. We welcome any feedback and contributions!
