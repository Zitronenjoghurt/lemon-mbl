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

## Physical Types

Every monster will have 1 or more physical types, which describe what kind of material they consist of. These could be
stuff like Organic, Slime, Ethereal, Construct, ... Attacking actions of certain damage types might have advantage or
disadvantage against different kind of physical types. E.g. fire-based action might have an advantage against
slime-based monsters.

## Elemental Types

Every monster will also have 1 or more elemental types, which describe what kind of power they can harness/control or
what kind of power makes up their being. Similar to the physical type, different damage types might have
advantage/disadvantage against certain elemental types. This creates an additional layer of complexity since now, if a
slime is of elemental type frost and frost has advantage against fire-based actions, the previous fire weakness of the
slime will be evened out.

# Story / Lore

Work in progress. I'll continuously extend on those ideas, it's probably a bit random but might help to give a bit of
context to this little "game". Inspirations are Persona, Pokemon and Mob Psycho.

## The Veil

The Veil is a ubiquitous field of energy that exists independent of the physical world. Because of some kind of
imbalance, veil energy infiltrates the physical world where it's able to focus into so called 'Veil Nodes'
which manifest into different kinds of monsters. Veil Nodes can pose a significant threat to humans as they are
uncontrollable and unpredictable.

## Veil Users

Veil users have the innate ability to control veil energy. Perhaps there will be some kind of way to capture and
manipulate Veil Nodes to fight for them.