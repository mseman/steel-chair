# Game Design

## Current Implementation - v0.2.0

1. Wrestler
    a. Each Wrestler has a Power, a number from 1 to 100 that represents the
        Wrestler's overall wrestling ability
2. Clash
    a. A Clash is a 1 vs 1 confrontation between two Wrestlers, where one
        Wrestler is determined to be the "winner"
    b. Rules for a Clash
        i.  By default, the odds for either Wrestler to win a Clash are 50%
        ii. The difference in Power between the two Wrestlers will increase the odds
            of winning in favor of the Wrestler with the greater Power
        iii. The minimum % chance of for any Wrestler to win a Clash will be set to 2%
        iv. The maximum % chance of for any Wrestler to win a Clash will be set to
            98%

## Implementation History

### v0.2.0

1. Update to section 2.b. Rules for a Clash
    i. By default, the odds for either Wrestler to win a Clash are 50%
    ii. The difference in Power between the two Wrestlers will increase the odds
        of winning in favor of the Wrestler with the greater Power
    iii. The minimum % chance of for any Wrestler to win a Clash will be set to
        2%
    iv. The maximum % chance of for any Wrestler to win a Clash will be set to
        98%

### v0.1.0

1. Wrestler
    a. Each Wrestler has a Power, a number from 1 to 100 that represents the
        Wrestler's overall wrestling ability
2. Clash
    a. A Clash is a 1 vs 1 confrontation between two Wrestlers, where one
        Wrestler is determined to be the "winner"
    b. Rules for a Clash
        i. Compare the Power of the two competing Wrestlers, and the Wrestler
            with the higher Power wins
        ii. If the two Wrestlers have the same Power, a "coin flip" will
            determine the winner

---

## Design Decisions

### GDD-003 Initial win probabilities

**Context** v0.2.0

As a starting point, the base win chance for each wrestler is 50%, the minimum
win chance is 2%, and thus the maximum win chance is 98%.

### GDD-002 No Guaranteed Wins

**SUPERSEDES GDD-001**

**Context:** v0.2.0

Instead of only integrating the element of chance for tiebreakers, all clashes
must have an element of chance. Regardless of a Wrestler's statistics or those
of their opponent, they must have at most a 99.9% chance of winning any Clash.

### GDD-001 Resolving Ties

**REPLACED BY GDD-002**

**Context:** v0.1.0

In the event that there is a pure tie, the tiebreaker is decided by a "coin
flip".
