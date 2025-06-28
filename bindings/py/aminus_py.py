"""
Aminus Python Bindings - Genshin Impact Damage Calculation Library

This module provides Python bindings for the Aminus Rust library,
enabling fast and accurate damage calculations for Genshin Impact.
"""

from enum import IntEnum
from . import aminus_py as _rust

# Re-export classes with cleaner names
StatTable = _rust.PyStatTable
ArtifactPiece = _rust.PyArtifactPiece
ArtifactBuilder = _rust.PyArtifactBuilder
Formulas = _rust.PyFormulas

# Re-export utility functions
stat_from_id = _rust.stat_from_id
element_from_id = _rust.element_from_id
damage_type_from_id = _rust.damage_type_from_id
scaling_from_id = _rust.scaling_from_id
amplifier_from_id = _rust.amplifier_from_id
quality_from_id = _rust.quality_from_id
stat_from_string = _rust.stat_from_string
get_stat_name = _rust.get_stat_name
is_elemental_dmg_bonus = _rust.is_elemental_dmg_bonus
get_roll_quality_multiplier = _rust.get_roll_quality_multiplier

# Enum classes using IntEnum for seamless integer compatibility
class Stat(IntEnum):
    """Character, weapon, and artifact stats"""
    BASE_HP = 0
    FLAT_HP = 1
    HP_PERCENT = 2
    BASE_ATK = 3
    FLAT_ATK = 4
    ATK_PERCENT = 5
    BASE_DEF = 6
    FLAT_DEF = 7
    DEF_PERCENT = 8
    ELEMENTAL_MASTERY = 9
    CRIT_RATE = 10
    CRIT_DMG = 11
    ENERGY_RECHARGE = 12
    DMG_BONUS = 13
    ELEMENTAL_DMG_BONUS = 14
    PYRO_DMG_BONUS = 15
    CRYO_DMG_BONUS = 16
    GEO_DMG_BONUS = 17
    DENDRO_DMG_BONUS = 18
    ELECTRO_DMG_BONUS = 19
    HYDRO_DMG_BONUS = 20
    ANEMO_DMG_BONUS = 21
    PHYSICAL_DMG_BONUS = 22
    NORMAL_ATK_DMG_BONUS = 23
    CHARGE_ATK_DMG_BONUS = 24
    PLUNGE_ATK_DMG_BONUS = 25
    SKILL_DMG_BONUS = 26
    BURST_DMG_BONUS = 27
    HEALING_BONUS = 28
    NONE = 29
    REACTION_BONUS = 30
    DEF_REDUCTION = 31
    DEF_IGNORE = 32
    PYRO_RESISTANCE_REDUCTION = 33
    HYDRO_RESISTANCE_REDUCTION = 34
    ELECTRO_RESISTANCE_REDUCTION = 35
    CRYO_RESISTANCE_REDUCTION = 36
    ANEMO_RESISTANCE_REDUCTION = 37
    GEO_RESISTANCE_REDUCTION = 38
    DENDRO_RESISTANCE_REDUCTION = 39
    PHYSICAL_RESISTANCE_REDUCTION = 40

class Element(IntEnum):
    """Elemental types"""
    PYRO = 0
    HYDRO = 1
    ELECTRO = 2
    CRYO = 3
    ANEMO = 4
    GEO = 5
    DENDRO = 6
    PHYSICAL = 7

class DamageType(IntEnum):
    """Types of damage attacks"""
    NORMAL = 0
    CHARGED = 1
    PLUNGING = 2
    SKILL = 3
    BURST = 4

class BaseScaling(IntEnum):
    """Base stats that abilities scale from"""
    ATK = 0
    DEF = 1
    HP = 2

class Amplifier(IntEnum):
    """Elemental reaction amplifiers"""
    NONE = 0
    FORWARD = 1  # Vaporize (Pyro -> Hydro), Melt (Pyro -> Cryo)
    REVERSE = 2  # Vaporize (Hydro -> Pyro), Melt (Cryo -> Pyro)

class RollQuality(IntEnum):
    """Artifact substat roll quality"""
    LOW = 0
    MID = 1
    HIGH = 2
    MAX = 3
    AVG = 4

# Keep backward compatibility with old class-based constants
class Stats:
    """Deprecated: Use Stat enum instead"""
    BASE_HP = Stat.BASE_HP
    FLAT_HP = Stat.FLAT_HP
    HP_PERCENT = Stat.HP_PERCENT
    BASE_ATK = Stat.BASE_ATK
    FLAT_ATK = Stat.FLAT_ATK
    ATK_PERCENT = Stat.ATK_PERCENT
    BASE_DEF = Stat.BASE_DEF
    FLAT_DEF = Stat.FLAT_DEF
    DEF_PERCENT = Stat.DEF_PERCENT
    ELEMENTAL_MASTERY = Stat.ELEMENTAL_MASTERY
    CRIT_RATE = Stat.CRIT_RATE
    CRIT_DMG = Stat.CRIT_DMG
    ENERGY_RECHARGE = Stat.ENERGY_RECHARGE
    DMG_BONUS = Stat.DMG_BONUS
    ELEMENTAL_DMG_BONUS = Stat.ELEMENTAL_DMG_BONUS
    PYRO_DMG_BONUS = Stat.PYRO_DMG_BONUS
    CRYO_DMG_BONUS = Stat.CRYO_DMG_BONUS
    GEO_DMG_BONUS = Stat.GEO_DMG_BONUS
    DENDRO_DMG_BONUS = Stat.DENDRO_DMG_BONUS
    ELECTRO_DMG_BONUS = Stat.ELECTRO_DMG_BONUS
    HYDRO_DMG_BONUS = Stat.HYDRO_DMG_BONUS
    ANEMO_DMG_BONUS = Stat.ANEMO_DMG_BONUS
    PHYSICAL_DMG_BONUS = Stat.PHYSICAL_DMG_BONUS
    NORMAL_ATK_DMG_BONUS = Stat.NORMAL_ATK_DMG_BONUS
    CHARGE_ATK_DMG_BONUS = Stat.CHARGE_ATK_DMG_BONUS
    PLUNGE_ATK_DMG_BONUS = Stat.PLUNGE_ATK_DMG_BONUS
    SKILL_DMG_BONUS = Stat.SKILL_DMG_BONUS
    BURST_DMG_BONUS = Stat.BURST_DMG_BONUS
    HEALING_BONUS = Stat.HEALING_BONUS
    NONE = Stat.NONE

class Elements:
    """Deprecated: Use Element enum instead"""
    PYRO = Element.PYRO
    HYDRO = Element.HYDRO
    ELECTRO = Element.ELECTRO
    CRYO = Element.CRYO
    ANEMO = Element.ANEMO
    GEO = Element.GEO
    DENDRO = Element.DENDRO
    PHYSICAL = Element.PHYSICAL

class DamageTypes:
    """Deprecated: Use DamageType enum instead"""
    NORMAL = DamageType.NORMAL
    CHARGED = DamageType.CHARGED
    PLUNGING = DamageType.PLUNGING
    SKILL = DamageType.SKILL
    BURST = DamageType.BURST

class Amplifiers:
    """Deprecated: Use Amplifier enum instead"""
    NONE = Amplifier.NONE
    FORWARD = Amplifier.FORWARD
    REVERSE = Amplifier.REVERSE

__version__ = "0.1.0"
__all__ = [
    "StatTable",
    "ArtifactPiece", 
    "ArtifactBuilder",
    "Formulas",
    # New enum classes
    "Stat",
    "Element",
    "DamageType",
    "BaseScaling", 
    "Amplifier",
    "RollQuality",
    # Backward compatibility classes
    "Stats",
    "Elements", 
    "DamageTypes",
    "Amplifiers",
    # Utility functions
    "stat_from_id",
    "element_from_id",
    "damage_type_from_id",
    "scaling_from_id",
    "amplifier_from_id",
    "quality_from_id",
    "stat_from_string",
    "get_stat_name",
    "is_elemental_dmg_bonus",
    "get_roll_quality_multiplier",
] 