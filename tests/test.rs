use polymorphic_enum::polymorphic_enum;

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

#[test]
fn test_macro() {
    // Create a vector of Moves
    let moves: Vec<Moves> = moves! {
        Attack { enemy_id: 1 },
        Defend
    };

    for m in moves {
        m.execute();
    }

    assert!(false);
}