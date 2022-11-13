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
        const payloadString = String.fromCharCode(...e.payload);
        if (payloadString.includes("__begin_prompt")) {
          isPrompt = true;
          console.log("prompt begins caught: " + payloadString);
        }
        if (payloadString.includes("__end_prompt")) {
          isPrompt = false;
          console.log("prompt ends caught: " + payloadString);
        }

        terminal.write(e.payload);
      }
    );

    invoke("connect_to_armageddon");

    return () => {
      terminal.dispose();
      unlisten();
    };
  });
</script>

<div id="terminal" class="terminal-height" />