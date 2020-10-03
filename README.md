# About

Hasami Shogi board game

# Develop

`gridsome develop` to start a local dev server at `http://localhost:8080`

*Write for readability untill performance becomes a problem*

## Conventions
The list of conventions that have been adapted for this project. 

### Git branch-naming

1. Use issue tracker IDs in branch names
1. Add a short descriptor of the task
1. Use hyphens as separators

Lead with issue tracker ID for the task followed by a short description of what
the task is about.

```sh
<issuenumber>-description-with-multiple-words
```

Source: [https://deepsource.io/blog/git-branch-naming-conventions/](https://deepsource.io/blog/git-branch-naming-conventions/)

### Git commit messages

Structure commit messages as follows: 

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Supported types: `fix`, `feat`, `build`, `chore`, `ci`, `docs`, `style`, 
`refactor`, `perf`, `test`

1. fix: a commit of the type fix patches a bug in your codebase (this correlates
 with PATCH in semantic versioning).
1. feat: a commit of the type feat introduces a new feature to the codebase 
(this correlates with MINOR in semantic versioning).
1. BREAKING CHANGE: a commit that has a footer BREAKING CHANGE:, or appends a ! 
after the type/scope, introduces a breaking API change (correlating with MAJOR 
in semantic versioning). A BREAKING CHANGE can be part of commits of any type.

Source: [https://www.conventionalcommits.org/en/v1.0.0/](https://www.conventionalcommits.org/en/v1.0.0/)

# Production 

`gridsome build` to build the static pages
