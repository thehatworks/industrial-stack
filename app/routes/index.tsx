import type { LoaderFunction } from "@remix-run/deno";
import { json } from "@remix-run/deno";
import { useLoaderData } from "@remix-run/react";
import React from "react";

import { possibly_recur_todo } from "../../rustlib/pkg/rustlib.js";
type Todo = {
    done: boolean,
    priority: number,
    due_date: string,
    title: string,
    steps: string[],
    followers: { 
      [x: string]: {
        email: string
      }
    }
};

const todos = [{
  done: false,
  priority: 42,
  due_date: `${new Date()}`,
  title: "A Task",
  steps: ["hmm"],
  followers: {
    me: {
      email: "me@example.com"
    },
    coworker: {
      email: "coworker@example.com"
    }   
  }
}]

export const loader: LoaderFunction = () => {
  return json<ExampleRecord>(
    JSON.parse(typed_and_serialized_from_deno(JSON.stringify(some_data)))
  );
};

export default function Index() {
  const loader_data = useLoaderData<ExampleRecord>();
  return (
    <main>
      <h2>
        Remix 💿 & Deno 🧦 in a tree 🌳, along comes ferris... 🦀
      </h2>
      <section>
        <pre>
          {JSON.stringify(loader_data, null, 2)}  
        </pre>
      </section>
    </main>
  );
}
