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

const migrateBetString1to2 = (str: string, isRaise: boolean) => {
  const { s, valid } = sanitizeBetString(str, isRaise);
  return valid ? s : "";
};

export const migrateConfig1to2 = (value: ConfigValue1): ConfigValue2 => {
  return {
    startingPot: Math.min(value.startingPot, MAX_AMOUNT),
    effectiveStack: Math.min(value.effectiveStack, MAX_AMOUNT),
    rakePercent: 0,
    rakeCap: 0,
    donkOption: 0,
    oopFlopBet: migrateBetString1to2(value.oopFlopBetStr, false),
    oopFlopRaise: migrateBetString1to2(value.oopFlopRaiseStr, true),
    oopTurnBet: migrateBetString1to2(value.oopTurnBetStr, false),
    oopTurnRaise: migrateBetString1to2(value.oopTurnRaiseStr, true),
    oopTurnDonk: "",
    oopRiverBet: migrateBetString1to2(value.oopRiverBetStr, false),
    oopRiverRaise: migrateBetString1to2(value.oopRiverRaiseStr, true),
    oopRiverDonk: "",
    ipFlopBet: migrateBetString1to2(value.ipFlopBetStr, false),
    ipFlopRaise: migrateBetString1to2(value.ipFlopRaiseStr, true),
    ipTurnBet: migrateBetString1to2(value.ipTurnBetStr, false),
    ipTurnRaise: migrateBetString1to2(value.ipTurnRaiseStr, true),
    ipRiverBet: migrateBetString1to2(value.ipRiverBetStr, false),
    ipRiverRaise: migrateBetString1to2(value.ipRiverRaiseStr, true),
    addAllInThreshold: value.addAllInThreshold,
    forceAllInThreshold: value.forceAllInThreshold,
    mergingThreshold: 0,
    expectedBoardLength: 0,
    addedLines: "",
    removedLines: "",
  };
};
