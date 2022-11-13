<script lang="ts">
  import type { Event } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import type {
    Position,
    Speed,
    Armed,
    Time,
    Day,
    Scan,
    Listen,
    Hunger,
    Thirst
  } from "../character-state";
  import {
    health,
    maxHealth,
    mana,
    maxMana,
    stamina,
    maxStamina,
    stun,
    maxStun,
    focus,
    maxFocus,
    visibility,
    position,
    verbosePosition,
    flying,
    language,
    accent,
    mount,
    speed,
    armed,
    mood,
    time,
    encumbrance,
    day,
    scan,
    listen,
    stance,
    intoxication,
    hunger,
    thirst,
    combatQuit
  } from "../character-state";
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
    if (bytes.includes(95)) {
      if (payloadContainsBeginPrompt(bytes)) {
        const payloadString = String.fromCharCode(...bytes);
        const promptMatter = payloadString.split("__begin_prompt");
        capturedPrompt += promptMatter[1].replace("__end_prompt", "");
        isPrompt = true;

        if (payloadContainsEndPrompt(bytes)) {
          processPromptData();
          isPrompt = false;
          return;
        }
      } else if (isPrompt) {
        if (payloadContainsEndPrompt(bytes)) {
          const payloadString = String.fromCharCode(...bytes);
          const promptMatter = payloadString.split("__begin_prompt");
          // promptMatter[0] here should be the prompt data
          capturedPrompt += promptMatter[0].replace("__end_prompt", "");
          processPromptData();
          // on this assign we're closing the prompt and can process the data
          isPrompt = false;
          return;
        } else {
          // @TODO: Handle this corner case
          const payloadString = String.fromCharCode(...bytes);
          const promptMatter = payloadString.split("__begin_prompt");
          return;
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

  function processPromptData() {
    const promptItems = capturedPrompt
      .replace(/\[/g, "")
      .replace(/\]/g, "")
      .split(" ");
    capturedPrompt = "";

    health.set(Number(promptItems[0]));
    maxHealth.set(Number(promptItems[1]));
    mana.set(Number(promptItems[2]));
    maxMana.set(Number(promptItems[3]));
    stamina.set(Number(promptItems[4]));
    maxStamina.set(Number(promptItems[5]));
    stun.set(Number(promptItems[6]));
    maxStun.set(Number(promptItems[7]));
    focus.set(Number(promptItems[8]));
    maxFocus.set(Number(promptItems[9]));
    // longDescriptionStatus: result[11],
    // longDescription: result[12],
    // name: result[13],
    visibility.set(promptItems[14]);
    position.set(promptItems[15] as Position);
    verbosePosition.set(promptItems[16]);
    flying.set(promptItems[17]);
    language.set(promptItems[18]);
    accent.set(promptItems[19]);
    mount.set(promptItems[20]);
    speed.set(promptItems[21] as Speed);
    armed.set(promptItems[22] as Armed);
    mood.set(promptItems[23]);
    time.set(promptItems[24] as Time);
    encumbrance.set(promptItems[25]);
    day.set(promptItems[26] as Day);
    scan.set(promptItems[27] as Scan);
    listen.set(promptItems[28] as Listen);
    stance.set(promptItems[29]);
    intoxication.set(promptItems[30]);
    hunger.set(promptItems[31] as Hunger);
    thirst.set(promptItems[32] as Thirst);
    combatQuit.set(promptItems[33]);
  }
</script>

<div id="terminal" class="terminal-height" />
