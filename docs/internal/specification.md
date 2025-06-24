 # Specification
 
 ## What is Aminus
 Aminus is a genshin impact damage and stat calculation library and framework.
 
## Why use Aminus
 Aminus is a **programmatic** alternative to using spread-sheets for genshin impact metagaming and theorycrafting use cases. 

 You can use Aminus for any use-case where you want to model stats or calculate damage. Such as for doing teamdps calculations, gearing/upgrade (eg: weapon, artifact, constelation, talents) comparison, estimating energy-recharge requirements ect.

## Front End
 Aminus provides a set of **primatives** and **abstractions** for damage-stat calculation.

 ### Primatives
  **Attribute:**  
  enum that represents a stat type. 
  
  eg: FlatATK, ATK%, Crit-Rate, Dmg%

  **StatValue:**  
  a pair between an attribute and a given value
  
  eg: (CritRate, 60%)

  some entity (character, weapon artifact) may have crit rate as a stat attribute but a StatValue specifies *how much* of a attribute it has.

 **StatTable:**
 a collection/data structure of multiple stat to value mappings.
 
 eg: {(ATK%, 20%), (CritRate, 60%), (CritDMG, 120%)}
 
 this can be used to model any in-game "stat table", from the representation of a character's total stats to total amount of stats given from a weapon or artifact piece


**Operation**

a computation that takes StatTable(s) and computes it to resulting StatTable or number

ops can unary: (StatTable)->StatTable, turnary: (StatTable,StatTable)->StatTable

most common use for operations is to apply some formula to a stattable to compute damage. or to merge 2 stat tables without mutation

**Rotation**

specififes a sequence of actions (normal attack, skill, burst) a character performs to deal damage.

a rotation is just a collection of (StatTable)->number operations where the StatTable represents a character's total stats used for this action while then number is the damage output of the action.



 ### Abstractions

 