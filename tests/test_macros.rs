use polymorphic_enum::polymorphic_enum;

polymorphic_enum!(
    trait Move {
        fn execute(&self);
        fn valid_for_state(&self, state: u8) -> bool;
    }

    enum Moves {
        Attack { card_id: u32, attack_power: u32, name: String },
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

#[derive(Debug)]
struct Larper;

#[test]
fn test_macro() {
    let attack = Attack {
        card_id: 0,
        attack_power: 0,
        name: String::from("Test"),
    };

    let larp1 = Larper;

    // debug print larper
    println!("{:?}", larp1);

    let test = Test(0, String::from("Test"));

    let moves: Vec<Moves> = moves![attack, Defend, test];

    for m in moves.iter() {
        m.execute();
    }

    assert!(false);
}