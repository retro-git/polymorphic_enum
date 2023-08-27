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

You are now expected to implement the trait for each of the generated structs like so:
```rust
impl Move for Attack {
    fn execute(&self) {
        println!("Attack!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 0
    }
}

impl Move for Defend {
    fn execute(&self) {
        println!("Defend!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 1
    }
}
```

Then we can do something like this:
```rust
fn test_moves() {
    // Create a list of Moves
    let moves: Vec<Moves> = vec![Attack { enemy_id: 1 }.into(), Defend.into()];

    for m in moves {
        m.execute(); // Prints "Attack!" and "Defend!"
    }
}
```