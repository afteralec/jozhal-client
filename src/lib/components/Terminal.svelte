<script lang="ts">
  import type { Event } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import "../../styles/xterm.css";

  let isPrompt = false;
  let capturedPrompt = "";

  onMount(async () => {
    let terminalNode = document.getElementById("terminal");
    if (!terminalNode) return;

    const { Terminal } = await import("xterm");
    const { FitAddon } = await import("xterm-addon-fit");

    const terminal = new Terminal({
      theme: {
        background: "#0f172a",
        cursor: "#0f172a"
      }
    });
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);

    terminal.open(terminalNode);
    fitAddon.fit();

    window.addEventListener("resize", () => {
      fitAddon.fit();
      console.log("window resized");
    });

    const { invoke } = await import("@tauri-apps/api");
    const { listen } = await import("@tauri-apps/api/event");

    const unlisten = await listen(
      "armageddon_output",
      (e: Event<Uint8Array>) => {
        processPrompt(e.payload);

        terminal.write(e.payload);
      }
    );

    invoke("connect_to_armageddon");

    return () => {
      terminal.dispose();
      unlisten();
    };
  });

  function processPrompt(bytes: Uint8Array) {
    for (let i = 0; i < bytes.length; i++) {
      if (bytes.includes(95)) {
        if (payloadContainsBeginPrompt(bytes)) {
          const payloadString = String.fromCharCode(...bytes);
          const promptMatter = payloadString.split("__begin_prompt");
          console.log("caught begin prompt: " + promptMatter);
          isPrompt = true;

          if (payloadContainsEndPrompt(bytes)) {
            console.log("caught end of prompt in same payload as begin prompt");
            isPrompt = false;
            return;
          }
        } else if (isPrompt) {
          if (payloadContainsEndPrompt(bytes)) {
            const payloadString = String.fromCharCode(...bytes);
            const promptMatter = payloadString.split("__begin_prompt");
            console.log("caught end prompt: " + promptMatter);
            isPrompt = false;
            return;
          } else {
            const payloadString = String.fromCharCode(...bytes);
            const promptMatter = payloadString.split("__begin_prompt");
            console.log("middle prompt matter: " + promptMatter);
          }
        }
      }
    }
  }

  function payloadContainsBeginPrompt(bytes: Uint8Array) {
    for (let i = 0; i < bytes.length; i++) {
      const byte = bytes[i];

      if (byte === 95) {
        if (payloadContainsBeginPromptFromStart(bytes, i)) {
          return true;
        }
      }
    }

    return false;
  }

  let beginPromptBytes = [
    95, 95, 98, 101, 103, 105, 110, 95, 112, 114, 111, 109, 112, 116
  ];
  function payloadContainsBeginPromptFromStart(
    bytes: Uint8Array,
    startIndex: number
  ) {
    let j = 1;
    for (let i = startIndex; i < startIndex + 13; i++) {
      if (!bytes[i]) return false;

      if (bytes[i] !== beginPromptBytes[j]) return false;

      j++;
    }

    return true;
  }

  function payloadContainsEndPrompt(bytes: Uint8Array) {
    for (let i = 0; i < bytes.length; i++) {
      const byte = bytes[i];

      if (byte === 95) {
        if (payloadContainsEndPromptFromStart(bytes, i)) {
          return true;
        }
      }
    }

    return false;
  }

  let endPromptBytes = [
    95, 95, 101, 110, 100, 95, 112, 114, 111, 109, 112, 116
  ];
  function payloadContainsEndPromptFromStart(
    bytes: Uint8Array,
    startIndex: number
  ) {
    let j = 1;
    for (let i = startIndex; i < startIndex + 11; i++) {
      if (!bytes[i]) return false;

      if (bytes[i] !== endPromptBytes[j]) return false;

      j++;
    }

    return true;
  }
</script>

<div id="terminal" class="terminal-height" />
