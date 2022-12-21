import { MAX_AMOUNT, sanitizeBetString } from "./utils";

export type ConfigValue1 = {
  startingPot: number;
  effectiveStack: number;
  oopFlopBetStr: string;
  oopFlopRaiseStr: string;
  oopTurnBetStr: string;
  oopTurnRaiseStr: string;
  oopRiverBetStr: string;
  oopRiverRaiseStr: string;
  ipFlopBetStr: string;
  ipFlopRaiseStr: string;
  ipTurnBetStr: string;
  ipTurnRaiseStr: string;
  ipRiverBetStr: string;
  ipRiverRaiseStr: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  adjustLastTwoBetSizes: number;
};

type ConfigValue2 = {
  startingPot: number;
  effectiveStack: number;
  rakePercent: number;
  rakeCap: number;
  donkOption: number;
  oopFlopBet: string;
  oopFlopRaise: string;
  oopTurnBet: string;
  oopTurnRaise: string;
  oopTurnDonk: string;
  oopRiverBet: string;
  oopRiverRaise: string;
  oopRiverDonk: string;
  ipFlopBet: string;
  ipFlopRaise: string;
  ipTurnBet: string;
  ipTurnRaise: string;
  ipRiverBet: string;
  ipRiverRaise: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  mergingThreshold: number;
  expectedBoardLength: number;
  addedLines: string;
  removedLines: string;
};

export const migrateConfig1to2 = (value: ConfigValue1): ConfigValue2 => {
  return {
    startingPot: Math.min(value.startingPot, MAX_AMOUNT),
    effectiveStack: Math.min(value.effectiveStack, MAX_AMOUNT),
    rakePercent: 0,
    rakeCap: 0,
    donkOption: 0,
    oopFlopBet: sanitizeBetString(value.oopFlopBetStr, false).s,
    oopFlopRaise: sanitizeBetString(value.oopFlopRaiseStr, true).s,
    oopTurnBet: sanitizeBetString(value.oopTurnBetStr, false).s,
    oopTurnRaise: sanitizeBetString(value.oopTurnRaiseStr, true).s,
    oopTurnDonk: "",
    oopRiverBet: sanitizeBetString(value.oopRiverBetStr, false).s,
    oopRiverRaise: sanitizeBetString(value.oopRiverRaiseStr, true).s,
    oopRiverDonk: "",
    ipFlopBet: sanitizeBetString(value.ipFlopBetStr, false).s,
    ipFlopRaise: sanitizeBetString(value.ipFlopRaiseStr, true).s,
    ipTurnBet: sanitizeBetString(value.ipTurnBetStr, false).s,
    ipTurnRaise: sanitizeBetString(value.ipTurnRaiseStr, true).s,
    ipRiverBet: sanitizeBetString(value.ipRiverBetStr, false).s,
    ipRiverRaise: sanitizeBetString(value.ipRiverRaiseStr, true).s,
    addAllInThreshold: value.addAllInThreshold,
    forceAllInThreshold: value.forceAllInThreshold,
    mergingThreshold: 0,
    expectedBoardLength: 0,
    addedLines: "",
    removedLines: "",
  };
};
