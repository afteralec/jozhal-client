<script lang="ts">
  import { onMount, afterUpdate } from "svelte";

  let value = "";
  let history: string[] = [];
  let historyPosition = 0;
  let selectValueOnUpdate = false;

  onMount(async () => {
    const textAreaNode = document.querySelector("#armageddon-input");
    if (!textAreaNode) return;
    (textAreaNode as HTMLTextAreaElement).focus();
  });

  afterUpdate(() => {
    if (selectValueOnUpdate) {
      selectValue();
      selectValueOnUpdate = false;
    }
  });

  function handleInputChange(event: Event) {
    const target = event?.target;

    if (target) {
      value = (target as HTMLTextAreaElement).value;
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      historyUp();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      historyDown();
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (event.shiftKey) return;

    if (event.key === "Enter") {
      submitInput();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
    }
  }

  async function submitInput() {
    const { invoke } = await import("@tauri-apps/api");

    if (value === "_prompt") {
      const promptCommand =
        "prompt __begin_prompt [%h] [%H] [%m] [%M] [%v] [%V] [%t] [%T] [%x] [%X] [%l] [%L] [%n] [%i] [%s] [%S] [%f] [%o] [%a] [%k] [%w] [%A] [%O] [%e] [%E] [%d] [%p] [%P] [%c] [%R] [%u] [%U] [%q] __end_prompt";

      invoke("process_armageddon_input", { input: promptCommand });
      selectValue();
      return;
    }

    invoke("process_armageddon_input", { input: value });

    if (getLastHistory() !== value && value.length > 0) {
      history.push(value);
      historyPosition = history.length - 1;
    }

    selectValue();
  }

  function historyUp() {
    if (historyPosition > 0) {
      historyPosition = historyPosition - 1;
    } else {
      return;
    }

    setValueToHistoryAtHistoryPosition();
    selectValue();
  }

  function historyDown() {
    if (historyPosition == history.length) {
      value = "";
      return;
    }

    if (historyPosition < history.length) {
      historyPosition = historyPosition + 1;
    }

    setValueToHistoryAtHistoryPosition();
    selectValue();
  }

  function setValueToHistoryAtHistoryPosition() {
    if (history[historyPosition]) {
      value = history[historyPosition];
    } else {
      value = "";
    }
  }

  function selectValue() {
    const textAreaNode = document.querySelector("#armageddon-input");
    if (!textAreaNode) return;
    (textAreaNode as HTMLTextAreaElement).select();
  }

  function getLastHistory() {
    if (history.length === 0) return null;
    return history[history.length - 1];
  }
</script>

<div class="input-height flex items-center justify-center pt-8 pb-8">
  <textarea
    id="armageddon-input"
    {value}
    on:input={handleInputChange}
    on:keydown={handleKeyDown}
    on:keyup={handleKeyUp}
    type="text"
    rows="2"
    class="w-[80%] bg-gray-800 text-white rounded focus:ring-0 focus:border-transparent border-transparent"
  />
</div>

<style>
  textarea {
    resize: none;
  }
</style>
