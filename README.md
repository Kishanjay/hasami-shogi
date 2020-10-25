# About
![CI/CD flow](https://github.com/kishanjay/hasami-shogi/workflows/CI/CD%20flow/badge.svg)

**Hasami Shogi board game**
*(please visit the about page of the live preview for a tutorial)*

Live preview: [https://hasami-shogi.netlify.app](https://hasami-shogi.netlify.app)
 
![Screenshot of the application](https://raw.githubusercontent.com/kishanjay/hasami-shogi/master/demo.png "Screenshot of the application")

## Stack
This serverless application was written with `Vue 3` as the frontend framework.
`Tailwind` is used tp add styling to the elements. `Webassembly` (`Rust`) is 
being used to compute the best move for the computer player. For computing the 
best move we're using the `minimax` algorithm. `Webpack` is used to bundle and 
build the application. `Github actions` is used as a `CI/CD` tool. `Netlify` is
used to host the result.


# Develop

`yarn build-wasm` to build the wasm computation engine 

`yarn serve` to start a local dev server at `http://localhost:8080`


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
