import { serve } from "https://deno.land/std@0.128.0/http/server.ts";
import { createRequestHandlerWithStaticFiles } from "@remix-run/deno";
// Import path interpreted by the Remix compiler
import * as build from "@remix-run/dev/server-build";

import init from "./rustlib/pkg/rustlib.js";

const go = async () => {
  await init(Deno.readFile('./rustlib/pkg/rustlib_bg.wasm'));
  
  const remixHandler = createRequestHandlerWithStaticFiles({
    build,
    mode: Deno.env.get("NODE_ENV"),
    getLoadContext: () => ({}),
  });
  
  const port = Number(Deno.env.get("PORT")) || 8000;
  console.log(`Listening on http://localhost:${port}`);
  serve(remixHandler, { port });
}

go();