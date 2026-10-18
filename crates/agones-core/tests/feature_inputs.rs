use std::cell::Cell;

use agones_core::*;
use cgp::prelude::*;

fn require_one_successful_child(result: std::process::Output) {
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "{stdout}\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        stdout.contains("1 passed; 0 failed; 0 ignored"),
        "empty/incorrect child selection: {stdout}"
    );
}

#[derive(HasField)]
struct CapturedInputs {
    feature_gate_defaults: &'static [(&'static str, bool)],
    feature_gate_input_options: FeatureGateInputOptions,
    environment: Option<Vec<u8>>,
    reads: Cell<usize>,
}

#[cgp_new_provider]
impl FeatureGateEnvironmentReader<CapturedInputs> for ReadCapturedEnvironment {
    fn read_feature_gate_environment(context: &CapturedInputs) -> Option<Vec<u8>> {
        context.reads.set(context.reads.get() + 1);
        context.environment.clone()
    }
}

delegate_and_check_components! {
    CapturedInputs {
        FeatureGateDefaultsGetterComponent: UseField<Symbol!("feature_gate_defaults")>,
        FeatureGateInputOptionsGetterComponent: UseField<Symbol!("feature_gate_input_options")>,
        FeatureGateEnvironmentReaderComponent: ReadCapturedEnvironment,
        FeatureGateParserComponent: ParseAgonesFeatureGates,
        FeatureGateConfigurationLoaderComponent: LoadAgonesFeatureGates,
    }
}

#[test]
fn source_input_precedence_and_empty_values_reach_the_same_production_provider() {
    type Case = (
        Option<&'static [u8]>,
        Option<&'static [u8]>,
        Option<&'static [u8]>,
        &'static str,
        usize,
    );
    let cases: &[Case] = &[
        (None, None, None, "Example=true&Test=false", 1),
        (None, None, Some(b""), "Example=true&Test=false", 1),
        (None, None, Some(b"Test=true"), "Example=true&Test=true", 1),
        (
            None,
            Some(b"Example=false"),
            Some(b"Test=true"),
            "Example=false&Test=false",
            0,
        ),
        (
            None,
            Some(b""),
            Some(b"Test=true"),
            "Example=true&Test=false",
            0,
        ),
        (
            Some(b"Test=true"),
            Some(b"Example=false"),
            Some(b"Example=false"),
            "Example=true&Test=true",
            0,
        ),
        (
            Some(b""),
            Some(b"Example=false"),
            Some(b"Test=true"),
            "Example=true&Test=false",
            0,
        ),
        (
            None,
            Some(b"Example=false&Example=\xff"),
            Some(b"Test=true"),
            "Example=false&Test=false",
            0,
        ),
    ];
    for (override_value, flag_value, environment, expected, reads) in cases {
        let context = CapturedInputs {
            feature_gate_defaults: &[("Example", true), ("Test", false)],
            feature_gate_input_options: FeatureGateInputOptions {
                override_value: override_value.map(<[u8]>::to_vec),
                flag_value: flag_value.map(<[u8]>::to_vec),
            },
            environment: environment.map(<[u8]>::to_vec),
            reads: Cell::new(0),
        };
        assert_eq!(context.load_feature_gates().unwrap().encode(), *expected);
        assert_eq!(context.reads.get(), *reads);
    }
}

#[test]
fn invalid_selected_input_rejects_instead_of_falling_through_to_environment() {
    let context = CapturedInputs {
        feature_gate_defaults: &[("Example", true), ("Test", false)],
        feature_gate_input_options: FeatureGateInputOptions {
            flag_value: Some(b"Example=\xff".to_vec()),
            ..Default::default()
        },
        environment: Some(b"Example=false".to_vec()),
        reads: Cell::new(0),
    };
    assert_eq!(
        context.load_feature_gates(),
        Err(FeatureGateError::InvalidBool {
            feature: "Example".into(),
            value: vec![0xff],
        })
    );
    assert_eq!(context.reads.get(), 0);
}

// The parent supplies a real environment to a separate process. No test changes
// the shared environment of this parallel test runner.
#[test]
fn process_environment_effect_preserves_missing_empty_and_raw_os_values() {
    use std::process::Command;
    let executable = std::env::current_exe().unwrap();
    for value in [
        None,
        Some(""),
        Some("Example=true"),
        Some("Example=invalid"),
        Some("Test=true"),
    ] {
        let mut child = Command::new(&executable);
        child
            .args([
                "--exact",
                "process_environment_child",
                "--ignored",
                "--nocapture",
            ])
            .env("AGONES_FEATURE_INPUT_CHILD", "1")
            .env("AGONES_FEATURE_INPUT_EXPECTED", value.unwrap_or("<absent>"));
        if let Some(value) = value {
            child.env(FEATURE_GATE_ENV, value);
        } else {
            child.env_remove(FEATURE_GATE_ENV);
        }
        require_one_successful_child(child.output().unwrap());
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        let result = Command::new(&executable)
            .args([
                "--exact",
                "process_environment_child",
                "--ignored",
                "--nocapture",
            ])
            .env("AGONES_FEATURE_INPUT_CHILD", "1")
            .env("AGONES_FEATURE_INPUT_EXPECTED", "<raw>")
            .env(
                FEATURE_GATE_ENV,
                std::ffi::OsString::from_vec(b"Example=true&Example=\xff".to_vec()),
            )
            .output()
            .unwrap();
        require_one_successful_child(result);
    }
}

#[test]
#[ignore = "runs only as the isolated process-effect child"]
fn process_environment_child() {
    assert_eq!(
        std::env::var("AGONES_FEATURE_INPUT_CHILD").as_deref(),
        Ok("1")
    );
    let expected = std::env::var("AGONES_FEATURE_INPUT_EXPECTED").unwrap();
    let mut context = FeatureGateInputContext::default();
    if expected == "Test=true" {
        context.feature_gate_defaults = &[("Example", true), ("Test", false)];
    }
    let raw = context.read_feature_gate_environment();
    let expected_raw = match expected.as_str() {
        "<absent>" => None,
        "<raw>" => Some(b"Example=true&Example=\xff".to_vec()),
        value => Some(value.as_bytes().to_vec()),
    };
    assert_eq!(raw, expected_raw);
    if expected == "Test=true" {
        let gates = context.load_feature_gates().unwrap();
        assert!(gates.enabled("Example"));
        assert!(gates.enabled("Test"));
    } else if expected == "Example=invalid" {
        assert!(context.load_feature_gates().is_err());
    } else {
        let gates = context.load_feature_gates().unwrap();
        assert_eq!(
            gates.enabled("Example"),
            expected == "Example=true" || expected == "<raw>"
        );
        assert!(gates.enabled("SidecarContainers"));
    }
    let explicit_empty = FeatureGateInputContext::from_parsed_flag(Some("".into()));
    assert!(
        !explicit_empty
            .load_feature_gates()
            .unwrap()
            .enabled("Example")
    );
}
