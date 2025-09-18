import { createEffect, createResource, createSignal, Show, Suspense } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import { LazyStore } from '@tauri-apps/plugin-store';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

import { TextField, TextFieldInput } from "~/ui/text-field";
import { Button } from "~/ui/button";

export default function Page() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const store = new LazyStore("settings.json")
  const [savedName, { refetch: refetchName }] = createResource(async () => store.get<string>("name"))
  const [onboarded, { refetch }] = createResource(async () => store.get<boolean>("onboarded"))
  const [name, setName] = createSignal("")

  createEffect(() => {
    const saved = savedName()
    if (saved) {
        setName(saved)
    }

  })

  async function greet() {
    info("Greeting from Solid")
    setGreetMsg(await invoke("greet", { name: name() }));
    await store.set("onboarded", !onboarded())
    await store.set("name", name())
  }

  return (
    <>
        <h1 class="typo-h1">Welcome to Tauri + Solid</h1>

        <p>Are you onboarded yet? {onboarded() ? "Yes" : "No"}</p>

        <div>{onboarded() ? <p>Hello, {savedName()}</p> : <p>Hello, anonymous</p>}</div>

        <form
          class="flex flex-row gap-2"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
        <TextField>
          <TextFieldInput
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          </TextField>
          <Button class="border" type="submit">Greet</Button>
        </form>
        <p>{greetMsg()}</p>
    </>
  );
}
