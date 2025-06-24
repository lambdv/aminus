(async () => {
  const aminus = await import('./pkg/aminus_js.js');

  function safeCall(label, fn) {
    try {
      console.log('Calling:', label);
      const result = fn();
      console.log(label + ':', result);
    } catch (e) {
      console.error('Error in', label, e);
    }
  }

  // Test main stat values
  safeCall('HP 5-star level 20', () => aminus.getMainStatValue(5, 20, 1)); // Stat.FlatHP
  safeCall('ATK 5-star level 20', () => aminus.getMainStatValue(5, 20, 4)); // Stat.FlatATK
  safeCall('Energy Recharge 5-star level 20', () => aminus.getMainStatValue(5, 20, 12)); // Stat.EnergyRecharge
  safeCall('ATK% 5-star level 20', () => aminus.getMainStatValue(5, 20, 5)); // Stat.ATKPercent

  // Test sub stat values
  safeCall('Crit Rate 5-star', () => aminus.getSubStatValue(5, 10)); // Stat.CritRate
  safeCall('ATK% 5-star', () => aminus.getSubStatValue(5, 5)); // Stat.ATKPercent

  // Test character stats
  safeCall('Diluc Base HP', () => aminus.getCharacterBaseStats("Diluc").get(0)); // Stat.BaseHP
  safeCall('Diluc Base ATK', () => aminus.getCharacterBaseStats("Diluc").get(3)); // Stat.BaseATK
  safeCall('Diluc Base DEF', () => aminus.getCharacterBaseStats("Diluc").get(6)); // Stat.BaseDEF

  // Test weapon stats
  safeCall("Wolf's Gravestone Base ATK", () => aminus.getWeaponStats("Wolf's Gravestone").get(3)); // Stat.BaseATK
})(); 