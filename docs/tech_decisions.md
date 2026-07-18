# Tech Decisions

## TD-001 Rust

I enjoy using Rust.

## TD-002 dependency: claims

Provides helpful testing assertion macros, for example assert_ok and assert_err.

## TD-003 `clash`

`match` is a reserved word in Rust, so instead a wrestling match will henceforth
be referred to as a `clash`.

## TD-004 `Wrestler.id` field

In order to compare two different wrestlers that have the same statistics, there
must be some unique identifier for each wrestler. I would use `name` but there
might be two unique wrestlers with the same name. Not a big deal if I change my
mind later.

For non-secure unique IDs, I go for UUID v7 by default.

## TD-005 clash argument `rng`

A clash may need a random number generator in order to determine its outcome. In
order to facilitate automated testing, the random number generator must be
provided to the clash.

## TD-006 Xoshiro128PlusPlus

We need to use a seedable Rng that are deterministic and portable, i.e. the same
seed will give you the same result regardless of package version. `StdRng` and
`SmallRng`, while deterministic, are not portable. Ultimately,
`Xoshiro128PlusPlus` was chosen due to meeting the deterministic and portability
criteria.

## TD-007 FakeRng

It's annoying to find a seed for a specific sequence of numbers with a real Rng.
To facilitate easier testing, I created `FakeRng` which will continuously loop
over a sequence of numbers provided on construction.
