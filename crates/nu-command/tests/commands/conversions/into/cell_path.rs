use nu_protocol::ShellError;
use nu_test_support::prelude::*;

#[test]
fn into_cell_path_with_negative_number_errors_out() -> Result {
    let err = test()
        .run("(-2) | into cell-path")
        .expect_shell_error()?;

    match err {
        ShellError::CantConvert {
            to_type, from_type, ..
        } => {
            assert_eq!(to_type, "cell path");
            assert_eq!(from_type, "negative number");
            Ok(())
        }
        err => Err(err.into()),
    }
}
