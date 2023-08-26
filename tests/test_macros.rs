use polymorphic_enum::polymorphic_enum;

polymorphic_enum!(
    trait Move {
        fn execute(&self);
        fn valid_for_state(&self, state: u8) -> bool;
    }

    #[derive(Clone)]
    enum Moves {
        #[derive(Clone)]
        Attack { card_id: u32, attack_power: u32, name: String },
        #[derive(Clone)]
        Defend,
        #[derive(Clone)]
        Test(u32, String)
    }
);

impl Move for Attack {
    fn execute(&self) {
        println!("Attack!");
        println!("{}", self.name);
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
    let attack: Moves = Attack {
        card_id: 0,
        attack_power: 0,
        name: String::from("Test"),
    }.into();

    // Convert attack back into an Attack.
    let attack2 = attack.clone();
    attack.execute();


    assert!(false);
}