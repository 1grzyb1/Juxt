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

## Contributing
If you'd like to contribute to the Juxt project, please submit a pull request or open an issue on the project's GitHub repository. We welcome any feedback and contributions!
