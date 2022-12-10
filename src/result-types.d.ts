export type Results = {
  currentPlayer: "oop" | "ip" | "chance" | "terminal";
  numActions: number;
  isEmpty: boolean[];
  weights: Float32Array[];
  normalized: Float32Array[];
  equity: Float32Array[];
  ev: Float32Array[];
  eqr: Float32Array[];
  strategy: Float32Array;
  evDetail: Float32Array;
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
    amount: number;
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
  barHeight: "normalized" | "full" | "absolute";
  suit: "grouped" | "individual";
  strategy: "show" | "none";
  content: "default" | "eq" | "ev" | "eqr";
};
