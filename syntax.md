# Syntax

Here's a summary of the syntax used in Juxt based on the given tests:

## Simple text output

To output plain text, just include it in the template. The following example outputs "Hello World":

```text
Hello World
```

## Comments

In Juxt, you can add comments to your templates using the // syntax. Everything from the // to the
end of the line is treated as a comment and will not be included in the output.

For example, the following template includes a comment:

text

```
// bla bla
```

This comment does not affect the output in any way. So if you were to compile and execute this
template, the result would be an empty string.

## Accessing the context

To access the context, use the `${context.property_name}` syntax. The following example accesses
the `test` property of the context:

```text
${context.test}
```

## Script declaration

To declare a script, use the `{#script}` and `{/script}` tags.

All the code between the tags will be included as js script.

The following example declares a function named `getPort`:

```text
{#script}
function getPort() {
    return 80;
}
{/script}
```

## Function call

To call a function, use the `${function_name()}` syntax. The following example calls the `getPort`
function:

```text
port: ${getPort()}
```

## Iterating over an array

To iterate over an array, use the `{#each element in array}` and `{/each}` tags. The following
example iterates over an array of ints and outputs them:

```text
{#each n in [0, 1, 2]}
  number: ${n}
{/each}
```

## Function call within an iterator

To call a function within an iterator, use the `${function_name(argument)}` syntax. The following
example calls the `plusOne` function within an iterator:

```text
{#script}
function plusOne(n) {
    return n + 1;
}
{/script}
{#each port in [0, 1, 2]}
  port: ${plusOne(port)}
{/each}
```

## Importing a component

To import a component, use the `{#import component_name}` syntax. The following example imports a
component named `component.juxt` and uses it in the template:

```text
{#import component.juxt}
${component()}
```

## Importing a script

To import a script, use the `{#import script_name}` syntax. The following example imports a script
named `script.js` and uses it in the template:

```text
{#import script.js}
${getPort()}
```

## Conditional rendering

To conditionally render content, use the `{#if condition}` and `{/if}` tags. Optionally, you can add
an `{#else}` and `{/else}` for an alternate content. The following example demonstrates conditional
rendering:

```text
{#if 1 == 1}
  asd
{/if}
{#else}
  dsa
{/else}
```
