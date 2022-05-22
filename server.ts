import { serve } from "https://deno.land/std@0.128.0/http/server.ts";
import { createRequestHandlerWithStaticFiles } from "@remix-run/deno";
// Import path interpreted by the Remix compiler
import * as build from "@remix-run/dev/server-build";

const load_wasm = async () => {
  console.log(Deno.cwd());
  const wasmCode = await Deno.readFile("rustlib/target/wasm32-unknown-unknown/debug/rustlib.wasm");
  const wasmModule = new WebAssembly.Module(wasmCode);
  const wasmInstance = new WebAssembly.Instance(wasmModule);
  const entrypoint = wasmInstance.exports.entrypoint as CallableFunction;
  console.log(`deno: ${entrypoint("my man")}`);
};
  
const go = async () => {
  await load_wasm();
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