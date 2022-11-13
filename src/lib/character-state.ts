import { writable, derived } from "svelte/store";
import type { Writable } from "svelte/store";

type Hunger = "starving" | "famished" | "very hungry" | "hungry" | "little hungry" | "peckish" | "satisfied" | "full"|"stuffed";
type Thirst = "dehydrated" | "parched" | "very thirsty" | "thirsty" | "little thirsty" | "not thirsty";
type Intoxication = "";
type Day = "Ocandra" | "Terrin" | "Abid" | "Cingel" | "Nekrete" | "Waleuk" | "Yochem" | "Huegel" | "Dzeda" | "Barani" | "Detal";
type Time = "before dawn" | "dawn" | "early morning" | "late morning" | "high sun" | "early afternoon" | "late afternoon" | "dusk" | "late at night";
type Armed = "armed" | "unarmed";
type Position = "standing" | "sitting" | "resting" | "sleeping";
type Speed = "walking" | "running" | "sneaking";
type Scan = "Scan" | "";
type Listen = "Listen" | "";
type Visibility = "Vis" | "Invis";
type Flying = "flying" | "not flying";
type Stance = "riposte" | "";
type CombatQuit = "can_quit" | "cannot_quit";

// Number states
const health: Writable<number | null> = writable(null);
const maxHealth: Writable<number | null> = writable(null);
const stun: Writable<number | null> = writable(null);
const maxStun: Writable<number | null> = writable(null);
const stamina: Writable<number | null> = writable(null);
const maxStamina: Writable<number | null> = writable(null);
const focus: Writable<number | null> = writable(null);
const maxFocus: Writable<number | null> = writable(null);
const mana: Writable<number | null> = writable(null);
const maxMana: Writable<number | null> = writable(null);

// Specific string states
const hunger: Writable<Hunger | null> = writable(null);
const thirst: Writable<Thirst | null> = writable(null);
const day: Writable<Day | null> = writable(null);
const time: Writable<Time | null> = writable(null);
const armed: Writable<Armed | null> = writable(null);
const position: Writable<Position | null> = writable(null);
const speed: Writable<Speed | null> = writable(null);
const scan: Writable<Scan | null> = writable(null);
const listen: Writable<Listen | null> = writable(null);

// Nonspecific string states
const accent: Writable<string | null> = writable(null);
const language: Writable<string | null> = writable(null);
const mount: Writable<string | null> = writable(null);

// Nonspecific string states to port to specific
const intoxication: Writable<string | null> = writable(null);
const visibility: Writable<string | null> = writable(null);
const flying: Writable<string | null> = writable(null);
const stance: Writable<string | null> = writable(null);
const combatQuit: Writable<string | null> = writable(null);

export {
  health,
  maxHealth,
  stun,
  maxStun,
  stamina,
  maxStamina,
  focus,
  maxFocus,
  mana,
  maxMana,
  hunger,
  thirst,
  day,
  time,
  armed,
  position,
  speed,
  scan,
  listen,
  accent,
  language,
  mount,
  intoxication,
  visibility,
  flying,
  stance,
  combatQuit
}
