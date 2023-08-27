use polymorphic_enum::polymorphic_enum;
use itertools::iproduct;

polymorphic_enum!(

    trait Move {
        fn execute(&self);
        fn valid_for_state(&self, state: u8) -> bool;
    }

    #[derive(Debug, Clone)]
    enum Moves {
        Attack { card_id: u32, test_id: u32 },
        Defend,
        Test(u32, String)
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

impl Move for Test {
    fn execute(&self) {
        println!("Test!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 2
    }
}

#[test]
fn test_macro() {
    // Create a vector of Moves
    let moves: Vec<Moves> = moves! {
        Attack { card_id: 1, test_id: 2 },
        Defend,
        Test(1, "test".to_string())
    };

    for m in moves {
        m.execute();
    }

    assert!(false);
}