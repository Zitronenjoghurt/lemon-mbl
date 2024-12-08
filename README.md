[![Rust](https://github.com/Zitronenjoghurt/lemon-mbl/actions/workflows/rust.yml/badge.svg)](https://github.com/Zitronenjoghurt/lemon-mbl/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/lemon-mbl/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/lemon-mbl)
![](https://tokei.rs/b1/github/Zitronenjoghurt/lemon-mbl?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# LeMon - Monster Battle Library

**WORK IN PROGRESS!**

This is a rust crate which contains game logic for simulated monster battles. The goals are to be easily extendable with
new monsters, moves, abilities, etc. and to explore the structure of battle systems you would find in Pokemon or similar
creature collectors.

# Battle System

In the following sections I will document how the overall battle system will work. This is highly work-in-progress, and
therefore subject to changes, additions and removals.

## Main Battle Mechanic

The main mechanical loop in battle will be Momentum -> Energy -> Actions. Each monster will start with a bit of their
maximum energy (depending on their vigilance stat). With certain actions they will be able to build up momentum and
higher momentum will recharge their energy faster, which will allow them to spend their energy on more and more
expensive actions.

## Monster Data

An explanation of some of the base stats that every monster has.

- Vitality
    - Determines the maximum HP of a monster
- Potential
    - Determines the maximum Energy the monster can hold
- Control
    - Determines the maximum Momentum the monster can hold
- Strength
    - Primary stat for scaling physical damaging moves
- Focus
    - Primary stat for scaling elemental/magical damaging moves
- Resilience
    - Defense => not quite sure how this will be used in the damage calculations yet
- Speed
    - Initiative, who gets to move first
- Technique
    - Crit-rate => probably a raw chance
- Agility
    - Evasion => probably also a raw chance
- Vigilance
    - Ratio how much of a monsters maximum energy (potential) is ready at the start of a battle

## Monster Status Information

- Current HP
- Current Energy
    - Will be used as a currency for taking actions
- Current Momentum
    - Will recharge energy each turn
- Desperation
    - A stat which will go up the more dire the situation is for a monster, this might scale with certain actions or
      abilities