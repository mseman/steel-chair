# Game Design

## Current Implementation

See GDI-001 v0.1.0

## Implementation History

### GDI-001 v0.1.0

1. Wrestler
    a. Each Wrestler has a Power, a number from 1 to 100 that represents the
        Wrestler's overall wrestling ability
2. Clash
    a. A Clash is a 1 vs 1 confrontation between two Wrestlers, where one
        Wrestler is determined to be the "winner"
    b. Rules for a Clash:
        i. Compare the Power of the two competing Wrestlers, and the Wrestler
            with the higher Power wins
        ii. If the two Wrestlers have the same Power, a "coin flip" will
            determine the winner

## Design Decisions

### GDD-001 Resolving Ties

In the event that there is a pure tie, the tiebreaker is decided by a "coin
flip".
