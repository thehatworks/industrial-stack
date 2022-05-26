# Industrial Stack (Remix + Deno + Rust)

Welcome to the industrial stack. feel that rusty metal grinding yet?

For more, check out the [Repository](https://github.com/thehatworks/industrial-stack)

## Install

```sh
npx create-remix@latest --template https://github.com/thehatworks/industrial-stack
```

edit deploy scripts to contain your deno deploy project

## Managing dependencies

Read about [how we recommend to manage dependencies for Remix projects using Deno](https://github.com/remix-run/remix/blob/main/decisions/0001-use-npm-to-manage-npm-dependencies-for-deno-projects.md).

- ✅ You should use `npm` to install NPM packages
  ```sh
  npm install react
  ```
  ```ts
  import { useState } from "react";
  ```
- ✅ You may use inlined URL imports or [deps.ts](https://deno.land/manual/examples/manage_dependencies#managing-dependencies) for Deno modules.
  ```ts
  import { copy } from "https://deno.land/std@0.138.0/streams/conversion.ts";
  ```
- ❌ Do not use [import maps](https://deno.land/manual/linking_to_external_code/import_maps).

## Development

From your terminal:

```sh
npm run dev
```

This starts your app in development mode, rebuilding assets on file changes.

### Type hints

This template provides type hinting to VS Code via a [dedicated import map](./.vscode/resolve_npm_imports.json).

To get types in another editor, use an extension for Deno that supports import maps and point your editor to `./.vscode/resolve_npm_imports.json`.

For more, see [our decision doc for interop between Deno and NPM](https://github.com/remix-run/remix/blob/main/decisions/0001-use-npm-to-manage-npm-dependencies-for-deno-projects.md#vs-code-type-hints).

## Deployment to Denoland Deploy

Building the Deno app (`npm run build`) results in two outputs:

- `build/` (server bundle)
- `public/build/` (browser bundle)

You can deploy these bundles to any host that runs Deno, but here we'll focus on deploying to [Deno Deploy](https://deno.com/deploy).

## Set up Deno Deploy

1. [Make a Github Repo](https://github.com/)

2. Iniitalize the repository.
```shell
git init
git add .
git commit -m "first commit"
git remote add origin <your repository origin url>
```

3. [Sign up](https://dash.deno.com/signin) for Deno Deploy.

4. [Create a new Deno Deploy project](https://dash.deno.com/new) for this app. On the creation page, make sure to link it to the github repository you created and add-origin'ed before and select "GitHub Actions" mode from the dropdown, NOT "Automatic"
### Deploying

```shell
git push origin main
```

pushing to any branch will make you a deployment! Yay!

