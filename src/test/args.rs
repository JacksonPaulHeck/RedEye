#[test]
fn test_args() {
    use crate::args::Args;

    // Construct Args
    let mut args = Args::create(None, false, false, false, false, false);

    // Test Getters
    assert_eq!(args.get_input_file(), &None);
    assert_eq!(args.get_dot(), &false);
    assert_eq!(args.get_compile(), &false);
    assert_eq!(args.get_interpret(), &false);
    assert_eq!(args.get_verbose(), &false);
    assert_eq!(args.get_run(), &false);

    // Test Setters
    args.set_input_file(Some(String::from("input.txt")));
    args.set_dot(true);
    args.set_compile(true);
    args.set_interpret(true);
    args.set_verbose(true);
    args.set_run(true);

    assert_eq!(args.get_input_file(), &Some(String::from("input.txt")));
    assert_eq!(args.get_dot(), &true);
    assert_eq!(args.get_compile(), &true);
    assert_eq!(args.get_interpret(), &true);
    assert_eq!(args.get_verbose(), &true);
    assert_eq!(args.get_run(), &true);
}
