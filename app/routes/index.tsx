import type { LoaderFunction } from "@remix-run/deno";
import { json } from "@remix-run/deno";
import { useLoaderData } from "@remix-run/react";

import * as React from "react";

import { typed_and_serialized_from_deno } from "../../rustlib/pkg/rustlib.js";
type ExampleRecord = {
  truthy: boolean;
  mathsy: number;
  spelly: string;
  county: ExampleRecord[];
  mappy: {
    [x: string]: ExampleRecord
  };
};


const leaf: ExampleRecord = {
  truthy: false,
  mathsy: 10,
  spelly: "hello rust, it's me deno 🧦",
  county: [],
  mappy: {}
};

const some_data: ExampleRecord = {
  ...leaf,
  county: [ leaf, leaf, leaf ],
  mappy: { leaf, leaf2: leaf, leaf3: leaf }
};

export const loader: LoaderFunction = () => {
  return json<ExampleRecord>(
    JSON.parse(typed_and_serialized_from_deno(JSON.stringify(some_data)))
  );
};

export default function Index() {
  const loader_data = useLoaderData<Value>();
  return (
    <main>
      <h2>
        hawt new remix stack comin' at ya 💿 🧦 🦀
      </h2>
      <section>
        <pre>
          {JSON.stringify(loader_data, null, 2)}  
        </pre>
      </section>
    </main>
  );
}
