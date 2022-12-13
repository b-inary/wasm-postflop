export type Results = {
  currentPlayer: "oop" | "ip" | "chance" | "terminal";
  numActions: number;
  isEmpty: boolean;
  weights: Float64Array[];
  normalizer: Float64Array[];
  equity: Float64Array[];
  ev: Float64Array[];
  eqr: Float64Array[];
  strategy: Float64Array;
  evDetail: Float64Array;
};

export type ChanceReports = {
  currentPlayer: "oop" | "ip" | "terminal";
  numActions: number;
  isValid: Float64Array;
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

export type DisplayMode = "basics" | "graphs" | "scatter" | "compare" | "stats";

export type DisplayOptions = {
  player: "auto" | "oop" | "ip";
  barHeight: "normalized" | "absolute" | "full";
  suit: "grouped" | "individual";
  strategy: "show" | "none";
  content: "default" | "eq" | "ev" | "eqr";
};
