import { describe, test, expect } from '@jest/globals';
import { Stat, Element, DamageType, BaseScaling, Amplifier, RollQuality } from '../src/types';
import { StatTable, statFromString, getStatName, isElementalDmgBonus, Formulas, StatFactory, Rotation, Optimizers, ArtifactPiece, ArtifactBuilder} from '../pkg/aminus_js';

// Team DPS calculation for Raiden National (Raiden, Xiangling, Xingqiu, Bennett)
function calculateRaidenNationalDPS() {

    // ============================================================================
    // Character Setup (using manual stat configuration)
    // ============================================================================

    // Xiangling (C6, Level 90, 9/12/12 talents)
    console.log("Setting up Xiangling...");
    const xiangling = new StatTable();
    // Base stats (approximated from CSV data)
    xiangling.add(Stat.BaseATK, 679); // Base ATK
    xiangling.add(Stat.BaseHP, 5288); // Base HP
    xiangling.add(Stat.BaseDEF, 39); // Base DEF
    xiangling.add(Stat.ElementalMastery, 960); // Ascension stat
    
    // Weapon: Dragon's Bane R5
    xiangling.add(Stat.BaseATK, 454);
    xiangling.add(Stat.ElementalMastery, 221);
    
    // Artifacts: 4pc Emblem of Severed Fate
    xiangling.add(Stat.FlatHP, 311); // Flower
    xiangling.add(Stat.FlatATK, 52); // Feather
    xiangling.add(Stat.EnergyRecharge, 0.47); // Sands
    xiangling.add(Stat.ElementalDMGBonus, 0.31); // Goblet
    xiangling.add(Stat.CritRate, 0.31); // Circlet
    
    // KQMC standard substats
    xiangling.add(Stat.HPPercent, 0.10);
    xiangling.add(Stat.ATKPercent, 0.10);
    xiangling.add(Stat.DEFPercent, 0.12);
    xiangling.add(Stat.FlatHP, 508);
    xiangling.add(Stat.FlatATK, 33);
    xiangling.add(Stat.FlatDEF, 39);
    xiangling.add(Stat.ElementalMastery, 158); // 79 + 79 for reactions
    xiangling.add(Stat.CritRate, 0.33);
    xiangling.add(Stat.CritDMG, 0.79);
    xiangling.add(Stat.EnergyRecharge, 0.11);

    // Xingqiu (C6, Level 90, 9/12/12 talents)
    console.log("Setting up Xingqiu...");
    const xingqiu = new StatTable();
    // Base stats
    xingqiu.add(Stat.BaseATK, 712);
    xingqiu.add(Stat.BaseHP, 5288);
    xingqiu.add(Stat.BaseDEF, 39);
    xingqiu.add(Stat.ATKPercent, 0.24); // Ascension stat
    
    // Weapon: Lion's Roar R5
    xingqiu.add(Stat.BaseATK, 510);
    xingqiu.add(Stat.ATKPercent, 0.41);
    
    // Artifacts: 4pc Emblem of Severed Fate
    xingqiu.add(Stat.FlatHP, 311);
    xingqiu.add(Stat.FlatATK, 52);
    xingqiu.add(Stat.EnergyRecharge, 0.47);
    xingqiu.add(Stat.ElementalDMGBonus, 0.31);
    xingqiu.add(Stat.CritRate, 0.31);
    
    // Substats
    xingqiu.add(Stat.HPPercent, 0.20);
    xingqiu.add(Stat.ATKPercent, 0.10);
    xingqiu.add(Stat.DEFPercent, 0.12);
    xingqiu.add(Stat.FlatHP, 508);
    xingqiu.add(Stat.FlatATK, 344);
    xingqiu.add(Stat.FlatDEF, 39);
    xingqiu.add(Stat.ElementalMastery, 40);
    xingqiu.add(Stat.CritRate, 0.33);
    xingqiu.add(Stat.CritDMG, 0.79);
    xingqiu.add(Stat.EnergyRecharge, 0.11);

    // Bennett (C6, Level 90, 9/12/12 talents)
    console.log("Setting up Bennett...");
    const bennett = new StatTable();
    // Base stats
    bennett.add(Stat.BaseATK, 756);
    bennett.add(Stat.BaseHP, 5288);
    bennett.add(Stat.BaseDEF, 39);
    bennett.add(Stat.EnergyRecharge, 0.27); // Ascension stat
    
    // Weapon: Sapwood Blade R5
    bennett.add(Stat.BaseATK, 565);
    bennett.add(Stat.EnergyRecharge, 0.31);
    
    // Artifacts: 4pc Noblesse Oblige
    bennett.add(Stat.FlatHP, 311);
    bennett.add(Stat.FlatATK, 52);
    bennett.add(Stat.EnergyRecharge, 0.47);
    bennett.add(Stat.ElementalDMGBonus, 0.31);
    bennett.add(Stat.CritRate, 0.31);
    
    // Substats
    bennett.add(Stat.HPPercent, 0.10);
    bennett.add(Stat.ATKPercent, 0.20);
    bennett.add(Stat.DEFPercent, 0.12);
    bennett.add(Stat.FlatHP, 508);
    bennett.add(Stat.FlatATK, 1342);
    bennett.add(Stat.FlatDEF, 39);
    bennett.add(Stat.ElementalMastery, 40);
    bennett.add(Stat.CritRate, 0.33);
    bennett.add(Stat.CritDMG, 0.79);

    // Raiden (C0, Level 90, 9/9/9 talents)
    console.log("Setting up Raiden...");
    const raiden = new StatTable();
    // Base stats
    raiden.add(Stat.BaseATK, 847);
    raiden.add(Stat.BaseHP, 5288);
    raiden.add(Stat.BaseDEF, 39);
    raiden.add(Stat.EnergyRecharge, 0.32); // Ascension stat
    
    // Weapon: The Catch R5
    raiden.add(Stat.BaseATK, 510);
    raiden.add(Stat.EnergyRecharge, 0.46);
    
    // Artifacts: 4pc Emblem of Severed Fate
    raiden.add(Stat.FlatHP, 311);
    raiden.add(Stat.FlatATK, 47);
    raiden.add(Stat.ATKPercent, 0.47); // ATK% sands for Raiden
    raiden.add(Stat.ElementalDMGBonus, 0.31);
    raiden.add(Stat.CritRate, 0.31);
    
    // Substats
    raiden.add(Stat.HPPercent, 0.10);
    raiden.add(Stat.ATKPercent, 0.20);
    raiden.add(Stat.DEFPercent, 0.12);
    raiden.add(Stat.FlatHP, 508);
    raiden.add(Stat.FlatATK, 1342);
    raiden.add(Stat.FlatDEF, 39);
    raiden.add(Stat.ElementalMastery, 40);
    raiden.add(Stat.CritRate, 0.33);
    raiden.add(Stat.CritDMG, 0.79);

    // ============================================================================
    // Team Buffs
    // ============================================================================

    // Bennett's ATK buff
    const bennettBuffs = new StatTable();
    bennettBuffs.add(Stat.FlatATK, 1000); // Bennett's Q flat ATK buff
    bennettBuffs.add(Stat.ATKPercent, 0.45); // Additional ATK% from artifacts
    bennettBuffs.add(Stat.ElementalDMGBonus, 0.51); // Elemental DMG bonus
    bennettBuffs.add(Stat.EnergyRecharge, 0.20); // ER bonus

    // Noblesse Oblige 4pc effect
    const noblesseBuffs = new StatTable();
    noblesseBuffs.add(Stat.ATKPercent, 0.20);

    // ============================================================================
    // Rotation Setup (22-second rotation)
    // ============================================================================

    console.log("Setting up team rotation...");

    // Xiangling's rotation
    const xianglingRotation = new Rotation();
    xianglingRotationadd("E", Element.Pyro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 4, 2.23); // Guoba
    xianglingRotationadd("N1_Vape", Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.Forward, 1, 1.44); // 1-Hit Swing + Vape
    xianglingRotationadd("N2", Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 1, 1.76); // 2-Hit Swing
    xianglingRotationadd("N3_Vape", Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.Forward, 1, 2.19); // 3-Hit Swing + Vape
    xianglingRotationadd("Q_VV", Element.Pyro, DamageType.Burst, BaseScaling.ATK, Amplifier.Forward, 8, 2.24); // Pyronado + VV + c1
    xianglingRotationadd("Q", Element.Pyro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 4, 2.24); // Pyronado hits

    // Xingqiu's rotation
    const xingqiuRotation = new Rotation();
    xingqiuRotationadd("E", Element.Hydro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 1, 7.18); // Skill
    xingqiuRotationadd("Q_c2_NO", Element.Hydro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 23, 1.09); // c2 + NO buffed Rain Swords
    xingqiuRotationadd("Q_buffed", Element.Hydro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 12, 1.09); // c2 buffed Rain Swords

    // Bennett's rotation
    const bennettRotation = new Rotation();
    bennettRotationadd("E", Element.Pyro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 1, 2.75); // E
    bennettRotationadd("E_no_buff", Element.Pyro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 1, 2.75); // E no bennett buff
    bennettRotationadd("Q", Element.Pyro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 1, 4.66); // Q

    // Raiden's rotation
    const raidenRotation = new Rotation();
    raidenRotationadd("E_cast", Element.Electro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 1, 1.99); // E cast
    raidenRotationadd("E_dot", Element.Electro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 18, 0.71); // E dot
    raidenRotationadd("Q_cast", Element.Electro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 1, 10.78); // Q cast
    raidenRotationadd("N3C_buffed", Element.Electro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 3, 6.73); // Bennett buffed n3c
    raidenRotationadd("N2_buffed", Element.Electro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 2, 2.967); // Bennett buffed n2
    raidenRotationadd("C", Element.Electro, DamageType.Charged, BaseScaling.ATK, Amplifier.None, 1, 3.763); // Charged attack
    raidenRotationadd("N1C", Element.Electro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 1, 3.763); // n1c

    // ============================================================================
    // Calculate DPS for each character
    // ============================================================================

    console.log("Calculating individual character DPS...\n");

    // Xiangling DPS
    const xianglingWithBuffs = new StatTable();
    xianglingWithBuffs.addTable(xiangling);
    xianglingWithBuffs.addTable(bennettBuffs);
    xianglingWithBuffs.addTable(noblesseBuffs);
    
    const xianglingDPR = xianglingRotation.evaluate(xianglingWithBuffs);
    const xianglingDPS = xianglingDPR / 22.0; // 22-second rotation

    // Xingqiu DPS
    const xingqiuWithBuffs = new StatTable();
    xingqiuWithBuffs.addTable(xingqiu);
    xingqiuWithBuffs.addTable(noblesseBuffs);
    
    const xingqiuDPR = xingqiuRotation.evaluate(xingqiuWithBuffs);
    const xingqiuDPS = xingqiuDPR / 22.0;

    // Bennett DPS
    const bennettWithBuffs = new StatTable();
    bennettWithBuffs.addTable(bennett);
    bennettWithBuffs.addTable(noblesseBuffs);
    
    const bennettDPR = bennettRotation.evaluate(bennettWithBuffs);
    const bennettDPS = bennettDPR / 22.0;

    // Raiden DPS
    const raidenWithBuffs = new StatTable();
    raidenWithBuffs.addTable(raiden);
    raidenWithBuffs.addTable(bennettBuffs);
    raidenWithBuffs.addTable(noblesseBuffs);
    
    const raidenDPR = raidenRotation.evaluate(raidenWithBuffs);
    const raidenDPS = raidenDPR / 22.0;

    // ============================================================================
    // Results
    // ============================================================================

    const totalDPR = xianglingDPR + xingqiuDPR + bennettDPR + raidenDPR;
    const totalDPS = totalDPR / 22.0;

    console.log("=== RESULTS ===");
    console.log(`Rotation Length: 22 seconds`);
    console.log(`\nCharacter Breakdown:`);
    console.log(`Xiangling: ${xianglingDPR.toFixed(0)} DPR, ${xianglingDPS.toFixed(0)} DPS (${((xianglingDPS/totalDPS)*100).toFixed(0)}%)`);
    console.log(`Xingqiu:   ${xingqiuDPR.toFixed(0)} DPR, ${xingqiuDPS.toFixed(0)} DPS (${((xingqiuDPS/totalDPS)*100).toFixed(0)}%)`);
    console.log(`Bennett:   ${bennettDPR.toFixed(0)} DPR, ${bennettDPS.toFixed(0)} DPS (${((bennettDPS/totalDPS)*100).toFixed(0)}%)`);
    console.log(`Raiden:    ${raidenDPR.toFixed(0)} DPR, ${raidenDPS.toFixed(0)} DPS (${((raidenDPS/totalDPS)*100).toFixed(0)}%)`);
    console.log(`\nTotal:     ${totalDPR.toFixed(0)} DPR, ${totalDPS.toFixed(0)} DPS`);

    // Compare with CSV data
    console.log(`\n=== COMPARISON WITH CSV DATA ===`);
    console.log(`CSV Total DPS: 52,634`);
    console.log(`Calculated DPS: ${totalDPS.toFixed(0)}`);
    console.log(`Difference: ${((totalDPS - 52634) / 52634 * 100).toFixed(1)}%`);

    return {
        xiangling: { dpr: xianglingDPR, dps: xianglingDPS },
        xingqiu: { dpr: xingqiuDPR, dps: xingqiuDPS },
        bennett: { dpr: bennettDPR, dps: bennettDPS },
        raiden: { dpr: raidenDPR, dps: raidenDPS },
        total: { dpr: totalDPR, dps: totalDPS }
    };
}

calculateRaidenNationalDPS()
