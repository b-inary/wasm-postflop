export type Results = {
  currentPlayer: "oop" | "ip" | "chance" | "terminal";
  numActions: number;
  isEmpty: number;
  eqrBase: number[];
  weights: number[][];
  normalizer: number[][];
  equity: number[][];
  ev: number[][];
  eqr: number[][];
  strategy: number[];
  actionEv: number[];
};

export type ChanceReports = {
  currentPlayer: "oop" | "ip" | "terminal";
  numActions: number;
  status: number[];
  combos: number[][];
  equity: number[][];
  ev: number[][];
  eqr: number[][];
  strategy: number[];
};

export type SpotRoot = {
  type: "root";
  index: 0;
  player: "flop" | "turn" | "river";
  selectedIndex: -1;
  board: number[];
  pot: number;
  stack: number;
};

export type SpotChance = {
  type: "chance";
  index: number;
  player: "turn" | "river";
  selectedIndex: number;
  prevPlayer: "oop" | "ip";
  cards: {
    card: number;
    isSelected: boolean;
    isDead: boolean;
  }[];
  pot: number;
  stack: number;
};

export type SpotPlayer = {
  type: "player";
  index: number;
  player: "oop" | "ip";
  selectedIndex: number;
  actions: {
    index: number;
    name: string;
    amount: string;
    isSelected: boolean;
    color: string;
  }[];
};

export type SpotTerminal = {
  type: "terminal";
  index: number;
  player: "end";
  selectedIndex: -1;
  prevPlayer: "oop" | "ip";
  equityOop: number;
  pot: number;
};

export type Spot = SpotRoot | SpotChance | SpotPlayer | SpotTerminal;

export const displayModeList = [
  "basics",
  "graphs",
  // "scatter",
  "compare",
  "chance",
] as const;

export type DisplayMode = (typeof displayModeList)[number];

export const playerBasicsList = ["auto", "oop", "ip"] as const;
export const playerChanceList = ["auto", "oop", "ip"] as const;
export const barHeightList = ["normalized", "absolute", "full"] as const;
export const suitList = ["grouped", "individual"] as const;
export const strategyList = ["show", "none"] as const;
export const contentBasicsList = ["default", "eq", "ev", "eqr"] as const;
export const contentGraphsList = ["eq", "ev", "eqr"] as const;
export const chartChanceList = [
  "strategy-combos",
  "strategy",
  "eq",
  "ev",
  "eqr",
] as const;

export type DisplayOptions = {
  playerBasics: (typeof playerBasicsList)[number];
  playerChance: (typeof playerChanceList)[number];
  barHeight: (typeof barHeightList)[number];
  suit: (typeof suitList)[number];
  strategy: (typeof strategyList)[number];
  contentBasics: (typeof contentBasicsList)[number];
  contentGraphs: (typeof contentGraphsList)[number];
  chartChance: (typeof chartChanceList)[number];
};

export type HoverContent = {
  name: string;
  indices: number[];
};

export type TableMode = "basics" | "graphs" | "chance";
