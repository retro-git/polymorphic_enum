use polymorphic_enum::polymorphic_enum;

polymorphic_enum!(
    trait Move {
        fn execute(&mut self);
        fn valid_for_state(&self, state: u8) -> bool;
    }

    enum Moves {
        Attack { card_id: u32, attack_power: u32, name: String },
        Defend,
        Test(u32, String)
    }
);

impl Move for Attack {
    fn execute(&mut self) {
        println!("Attack!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 0
    }
}

impl Move for Defend {
    fn execute(&mut self) {
        println!("Defend!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 1
    }
}

impl Move for Test {
    fn execute(&mut self) {
        println!("Test!");
    }

    fn valid_for_state(&self, state: u8) -> bool {
        state == 2
    }
}

#[test]
fn test_macro() {
    let mut attack = Moves::Attack(Attack {
        card_id: 0,
        attack_power: 0,
        name: String::from(""),
    });

    let mut defend = Moves::Defend(Defend{});

    let mut test = Moves::Test(Test(0, String::from("")));

    //put into vec with vec!
    let mut moves = vec![&mut attack, &mut defend, &mut test];

    for m in moves.iter_mut() {
        m.execute();
    }

    assert!(false);
}