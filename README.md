# polymorphic_enum

## Example:
Input:

```rust
polymorphic_enum!(
    trait Move {
        fn execute(&self);
        fn valid_for_state(&self, state: u8) -> bool;
    }

    #[derive(Debug, Clone)]
    enum Moves {
        Attack { enemy_id: u32 },
        Defend,
    }
);
```

Will expand to:

```rust
trait Move {
    fn execute(&self);
    fn valid_for_state(&self, state: u8) -> bool;
}

struct Attack{ enemy_id: u32 }
struct Defend;

enum Moves {
    Attack(Attack),
    Defend(Defend)
}

impl Move for Moves {
    fn execute(&self) {
        match self {
            Moves::Attack(inner) => inner.execute(),
            Moves::Defend(inner) => inner.execute(),
        }
    }

    fn valid_for_state(&self, state: u8) -> bool {
        match self {
            Moves::Attack(inner) => inner.valid_for_state(state),
            Moves::Defend(inner) => inner.valid_for_state(state),
        }
    }
}

impl From<Attack> for Moves {
    fn from(attack: Attack) -> Self {
        Moves::Attack(attack)
    }
}
impl From<Defend> for Moves {
    fn from(defend: Defend) -> Self {
        Moves::Defend(defend)
    }
}
impl Into<Attack> for Moves {
    fn into(self) -> Attack {
        match self {
            Moves::Attack(attack) => attack,
            _ => panic!("Tried to convert a Moves into a Attack but the enum variant was not Attack"),
        }
    }
}
impl Into<Defend> for Moves {
    fn into(self) -> Defend {
        match self {
            Moves::Defend(defend) => defend,
            _ => panic!("Tried to convert a Moves into a Defend but the enum variant was not Defend"),
        }
    }
}
```