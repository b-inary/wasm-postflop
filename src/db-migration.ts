import { MAX_AMOUNT } from "./utils";

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

const migrateBetString1to2 = (s: string): string => {
  if (s.includes(" ") && !s.includes(",")) {
    return s.replace(" ", ", ");
  } else {
    return s;
  }
};

export const migrateConfig1to2 = (value: ConfigValue1): ConfigValue2 => {
  return {
    startingPot: Math.min(value.startingPot, MAX_AMOUNT),
    effectiveStack: Math.min(value.effectiveStack, MAX_AMOUNT),
    rakePercent: 0,
    rakeCap: 0,
    donkOption: 0,
    oopFlopBet: migrateBetString1to2(value.oopFlopBetStr),
    oopFlopRaise: migrateBetString1to2(value.oopFlopRaiseStr),
    oopTurnBet: migrateBetString1to2(value.oopTurnBetStr),
    oopTurnRaise: migrateBetString1to2(value.oopTurnRaiseStr),
    oopTurnDonk: "",
    oopRiverBet: migrateBetString1to2(value.oopRiverBetStr),
    oopRiverRaise: migrateBetString1to2(value.oopRiverRaiseStr),
    oopRiverDonk: "",
    ipFlopBet: migrateBetString1to2(value.ipFlopBetStr),
    ipFlopRaise: migrateBetString1to2(value.ipFlopRaiseStr),
    ipTurnBet: migrateBetString1to2(value.ipTurnBetStr),
    ipTurnRaise: migrateBetString1to2(value.ipTurnRaiseStr),
    ipRiverBet: migrateBetString1to2(value.ipRiverBetStr),
    ipRiverRaise: migrateBetString1to2(value.ipRiverRaiseStr),
    addAllInThreshold: value.addAllInThreshold,
    forceAllInThreshold: value.forceAllInThreshold,
    mergingThreshold: 0,
    expectedBoardLength: 0,
    addedLines: "",
    removedLines: "",
  };
};
