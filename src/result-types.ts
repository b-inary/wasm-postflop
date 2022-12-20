export type Results = {
  currentPlayer: "oop" | "ip" | "chance" | "terminal";
  numActions: number;
  isEmpty: number;
  eqrBase: number[];
  weights: Float64Array[];
  normalizer: Float64Array[];
  equity: Float64Array[];
  ev: Float64Array[];
  eqr: Float64Array[];
  strategy: Float64Array;
  actionEv: Float64Array;
};

export type ChanceReports = {
  currentPlayer: "oop" | "ip" | "terminal";
  numActions: number;
  status: Float64Array;
  combos: Float64Array[];
  equity: Float64Array[];
  ev: Float64Array[];
  eqr: Float64Array[];
  strategy: Float64Array;
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
  "compare",
  // "graphs",
  // "scatter",
  "chance",
] as const;

export type DisplayMode = typeof displayModeList[number];

export const playerBasicsList = ["auto", "oop", "ip"] as const;
export const playerChanceList = ["auto", "oop", "ip"] as const;
export const barHeightList = ["normalized", "absolute", "full"] as const;
export const suitList = ["grouped", "individual"] as const;
export const strategyList = ["show", "none"] as const;
export const contentBasicsList = ["default", "eq", "ev", "eqr"] as const;
// export const contentChanceList = [
//   "strategy",
//   "combos",
//   "eq",
//   "ev",
//   "eqr",
// ] as const;

export type DisplayOptions = {
  playerBasics: typeof playerBasicsList[number];
  playerChance: typeof playerChanceList[number];
  barHeight: typeof barHeightList[number];
  suit: typeof suitList[number];
  strategy: typeof strategyList[number];
  contentBasics: typeof contentBasicsList[number];
  // contentChance: typeof contentChanceList[number];
};

export type HoverContent = {
  name: string;
  indices: number[];
};

export type TableMode = "basics" | "chance";
