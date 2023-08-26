use polymorphic_enum::polymorphic_enum;

polymorphic_enum!
(

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
        println!("{}", self.card_id);
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
        test_id: 1,
    }.into();

    // Attack card_id can range from 0 to 5. Test_id can range from 0 to 10. Generate all possible combinations in functional style.
    let moves: Vec<Moves> = (0..5).flat_map(|card_id| (0..10).map(move |test_id| Attack { card_id, test_id }.into())).collect(); // Move is necessary to capture card_id

    // Debug print all
    println!("{:#?}", moves);
    // Print array length
    println!("{}", moves.len());

    assert!(false);
}