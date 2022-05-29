# Industrial Stack (Remix 💿 + Deno 🧦 + Rust 🦀)

## MAJOR WARNING

I don't personally think this architecture makes sense in a Remix App, and likely won't develop this particular repository/stack any furter. That said, this is an example of synchronously calling into bound wasm (Rust) code from a Remix Loader.

## What works well
- `npm run dev` should work immediately after clone & install, browse to https://localhost:8000 to see index.tsx using data passed in and out of Rust in a loader.
- It will even watch for changes on your rustlib package! check out package.json, using a dirty nodemon hack & cross platform touch script I found on npm, when you make changes to rust and npm run dev is running, it will rebuild the wasm AND touch app/routes/index.jsx for you. now you too can develop BLAZINGLY FAST!
- Add/Edit rustlib/src/lib.rs, make sure to #[wasm_bindgen] and import { } anything you want in javascript (see `app/routes/index.tsx` and `rustlib/src/lib.rs`)
- rustlib/src/lib.rs has some examples of packages and data manipulation that might be useful when serializing and deserializing information to and from JavaScript (serde_json, chrono::Datetime, etc.)

## What might work
- `Dockerfile` is close to done / worked for me many times. Most of the hard part (actually getting a Docker with both rust and node installed means we cant just inherit FROM vanilla docker images :P )
- GitHub Action definitely isn't working but, I think all the parts are there potentially for a build and deploy to Deno Deploy not based on Docker. It takes longer than 10 seconds which will make Ryan Dahl cry.
- Haven't tried to deploy to deno deploy in a few commits, but, I did test it at least once with wasm. if you make sure to deploy build, public, and rustlib i.e. with deployctl it should more or less work..

## What doesn't work yet
- Not really a "stack" yet, more like an example app. i.e. no remix.init dir, etc..
- rust only tests & fixtures in `src/lib.rs`, which are run as one of the steps in `npm run test`

## What if I want to use this for real?

Reach out to me! mike AT thehatworks.com - I'd love to hear why, how, and debate systems architecture with you! We'd probably get along!

feel that rusty metal grinding yet?

In addition to the normal dependencies for Remix, you will need to have a working version of the rust toolchain, wasm-pack and support for the wasm32-unknown-unknown target. if the following command runs as expected you should be ok:

`rustup --version && cargo --version && wasm-pack --version`

In case you are reading a README somewhere, for more, check out the actual [Repository](https://github.com/thehatworks/industrial-stack)

## Install (intended to work but not yet probably)

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

Building the Deno & Wasm App (`npm run build`) results in three main artifacts you need:

- `build/` (server bundle)
- `public/build/` (browser bundle)
- `rustlib/pkg/` (packed up, importable npm package)
  - contains wasm binary and some shims for call interface. See the "wasm-pack" project for details

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

