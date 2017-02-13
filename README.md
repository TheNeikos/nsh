# nsh
## The Neikos Shell

This shell is for my personal use primarily. It is intended to run on my system, a linux environment.
I do not promise any compatibility to other platforms.

## Goals

**Be fast and correct.**

That's it really, it should be usable too.

## Syntax

The syntax is kept small and meant to be out of the way.

These constructs exist:

- `for`:
```
    $ for var in $(seq 1 5) do
        echo $var;
      end
```

- `{ }`:
```
    $ [ $var = 1 ] && { 
        echo '$var is 1';
        echo "And the path is: $PATH"
    }
```

- `&&` and `||` for chaining
- `|`, `>` and `<` for piping
- `#` are line comments

## Variables

- Each shell has its own set of variables, they are accessed through the `%` prefix.
- `> $var` in command position will resolve the variable and call it's value
- `> $var = <value>` will set the value to the variable.
- Variables are arrays, seperated by spaces
- `> $var = a b c d e f g` will be an array of 7 elements
- `> $var = "a b c" e f g` will be an array of 4 elements
- `> echo $var` will return the whole array
- `> echo $var[0]` will return the first element
- `> echo $var[-1]` will return the last element
- `> echo $var[10]` will not run and error instead
- `> echo $var[10]?` will return the 11th entry or empty string

## Environment Variables

- EnvVars are accessed through the `%` prefix.
- `> %var` resolves and substitutes the value and executes it
- `> %var = <value>` sets the value to the variable
- `> echo %var` substitutes the EnvVar
- If the EnvVar does not exist, an empty string is substituted

## Substitution

- Substitution is done after expressions are formed
- This means that substituting a command with spaces will not be split up
- `> command $variable #FOO`, here `command` will be called with two arguments which might have spaces themselves

## Quotes

- Both type of quotes work similarly, however, double quotes also substitute (and evaluate) any variables or subshells
- Single quotes do not do substitution

## Aliases

Aliases are done through executable files that are earlier in the path with the same name

## Functions

No builtin functions, the idea is to be nimble, executables are pretty nimble

## Autocomplete

Basic autocomplete will exist for commands and from the current path

## History

The history will be saved, unless started with the option to disable it

## Suggestions

History and path should be able to work together and suggest possible paths to be executed
