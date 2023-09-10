use gtest::{Log, Program, System};
use tmg1_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
    let block_height_0 = sys.block_height();
    let res = program.send(2, String::from("Hello"));
    assert!(!res.main_failed());

    // test `Name`
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder().payload(TmgEvent::Name(String::from("Hello")));
    assert!(res.contains(&expected_log));

    // test `Age`
    let res = program.send(2, TmgAction::Age);
    let block_height_1 = sys.block_height();
    let age = block_height_1 - block_height_0;
    let expected_log = Log::builder().payload(TmgEvent::Age(age.into()));
    assert!(res.contains(&expected_log));
}
