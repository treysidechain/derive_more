#![allow(dead_code)]

use std::any;

use super::*;

derive_display!(TestErr, T);
#[derive(Debug, Error)]
enum TestErr<T> {
    Unit,
    NamedImplicitNoBacktrace {
        field: T,
    },
    NamedImplicitBacktraceByFieldName {
        backtrace: MyBacktrace,
        field: T,
    },
    NamedImplicitBacktraceByFieldType {
        implicit_backtrace: Backtrace,
        field: T,
    },
    NamedExplicitNoBacktraceByFieldName {
        #[error(not(backtrace))]
        backtrace: MyBacktrace,
        field: T,
    },
    NamedExplicitNoBacktraceByFieldType {
        #[error(not(backtrace))]
        implicit_backtrace: Backtrace,
        field: T,
    },
    NamedExplicitBacktrace {
        #[error(backtrace)]
        explicit_backtrace: MyBacktrace,
        field: T,
    },
    NamedExplicitNoBacktraceRedundant {
        #[error(not(backtrace))]
        not_backtrace: MyBacktrace,
        #[error(not(backtrace))]
        field: T,
    },
    NamedExplicitBacktraceByFieldNameRedundant {
        #[error(backtrace)]
        backtrace: MyBacktrace,
        field: T,
    },
    NamedExplicitBacktraceByFieldTypeRedundant {
        #[error(backtrace)]
        implicit_backtrace: Backtrace,
        field: T,
    },
    NamedExplicitSupressesImplicit {
        #[error(backtrace)]
        not_backtrace: MyBacktrace,
        backtrace: Backtrace,
        field: T,
    },
    UnnamedImplicitNoBacktrace(T, T),
    UnnamedImplicitBacktrace(Backtrace, T, T),
    UnnamedExplicitNoBacktrace(#[error(not(backtrace))] Backtrace, T),
    UnnamedExplicitBacktrace(#[error(backtrace)] MyBacktrace, T, T),
    UnnamedExplicitNoBacktraceRedundant(
        #[error(not(backtrace))] MyBacktrace,
        #[error(not(backtrace))] T,
    ),
    UnnamedExplicitBacktraceRedundant(#[error(backtrace)] Backtrace, T, T),
    UnnamedExplicitSupressesImplicit(#[error(backtrace)] MyBacktrace, Backtrace, T),
}

impl<T> TestErr<T> {
    fn get_stored_backtrace(&self) -> &Backtrace {
        match self {
            Self::NamedImplicitBacktraceByFieldName { backtrace, .. } => backtrace,
            Self::NamedImplicitBacktraceByFieldType {
                implicit_backtrace, ..
            } => implicit_backtrace,
            Self::NamedExplicitBacktrace {
                explicit_backtrace, ..
            } => explicit_backtrace,
            Self::NamedExplicitBacktraceByFieldNameRedundant { backtrace, .. } => {
                backtrace
            }
            Self::NamedExplicitBacktraceByFieldTypeRedundant {
                implicit_backtrace,
                ..
            } => implicit_backtrace,
            Self::NamedExplicitSupressesImplicit { not_backtrace, .. } => not_backtrace,
            Self::UnnamedImplicitBacktrace(backtrace, _, _) => backtrace,
            Self::UnnamedExplicitBacktrace(backtrace, _, _) => backtrace,
            Self::UnnamedExplicitBacktraceRedundant(backtrace, _, _) => backtrace,
            Self::UnnamedExplicitSupressesImplicit(backtrace, _, _) => backtrace,
            _ => panic!("ERROR IN TEST IMPLEMENTATION"),
        }
    }

    fn get_unused_backtrace(&self) -> &Backtrace {
        match self {
            Self::NamedExplicitSupressesImplicit { backtrace, .. } => backtrace,
            Self::UnnamedExplicitSupressesImplicit(_, backtrace, _) => backtrace,
            _ => panic!("ERROR IN TEST IMPLEMENTATION"),
        }
    }
}

type MyBacktrace = Backtrace;

#[test]
fn unit() {
    assert!(any::request_ref::<Backtrace>(&TestErr::<i32>::Unit).is_none());
}

#[test]
fn named_implicit_no_backtrace() {
    let err = TestErr::NamedImplicitNoBacktrace { field: 0 };

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn named_implicit_backtrace_by_field_name() {
    let err = TestErr::NamedImplicitBacktraceByFieldName {
        backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn named_implicit_backtrace_by_field_type() {
    let err = TestErr::NamedImplicitBacktraceByFieldType {
        implicit_backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn named_explicit_no_backtrace_by_field_name() {
    let err = TestErr::NamedExplicitNoBacktraceByFieldName {
        backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn named_explicit_no_backtrace_by_field_type() {
    let err = TestErr::NamedExplicitNoBacktraceByFieldType {
        implicit_backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn named_explicit_backtrace() {
    let err = TestErr::NamedExplicitBacktrace {
        explicit_backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn named_explicit_no_backtrace_redundant() {
    let err = TestErr::NamedExplicitNoBacktraceRedundant {
        not_backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn named_explicit_backtrace_by_field_name_redundant() {
    let err = TestErr::NamedExplicitBacktraceByFieldNameRedundant {
        backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn named_explicit_backtrace_by_field_type_redundant() {
    let err = TestErr::NamedExplicitBacktraceByFieldTypeRedundant {
        implicit_backtrace: Backtrace::force_capture(),
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn named_explicit_supresses_implicit() {
    let err = TestErr::NamedExplicitSupressesImplicit {
        not_backtrace: Backtrace::force_capture(),
        backtrace: (|| Backtrace::force_capture())(), // ensure backtraces are different
        field: 0,
    };

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_unused_backtrace);
}

#[test]
fn unnamed_implicit_no_backtrace() {
    let err = TestErr::UnnamedImplicitNoBacktrace(0, 0);

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn unnamed_implicit_backtrace() {
    let err = TestErr::UnnamedImplicitBacktrace(Backtrace::force_capture(), 0, 0);

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn unnamed_explicit_no_backtrace() {
    let err = TestErr::UnnamedExplicitNoBacktrace(Backtrace::force_capture(), 0);

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn unnamed_explicit_backtrace() {
    let err = TestErr::UnnamedExplicitBacktrace(Backtrace::force_capture(), 0, 0);

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn unnamed_explicit_no_backtrace_redundant() {
    let err =
        TestErr::UnnamedExplicitNoBacktraceRedundant(Backtrace::force_capture(), 0);

    assert!(any::request_ref::<Backtrace>(&err).is_none());
}

#[test]
fn unnamed_explicit_backtrace_redundant() {
    let err =
        TestErr::UnnamedExplicitBacktraceRedundant(Backtrace::force_capture(), 0, 0);

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
}

#[test]
fn unnamed_explicit_supresses_implicit() {
    let err = TestErr::UnnamedExplicitSupressesImplicit(
        Backtrace::force_capture(),
        (|| Backtrace::force_capture())(), // ensure backtraces are different
        0,
    );

    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_unused_backtrace);
}

derive_display!(BoundedTestErr, T);
#[derive(Debug, Error)]
enum BoundedTestErr<T> {
    NamedImplicitNoBacktraceFromSource {
        #[error(source)]
        err: T,
    },
    NamedExplicitNoBacktraceFromSource {
        #[error(source, not(backtrace))]
        err: T,
    },
    NamedExplicitBacktraceFromSource {
        #[error(backtrace, source)]
        err: T,
    },
    NamedImplicitDifferentSourceAndBacktrace {
        #[error(source)]
        err: T,
        backtrace: Backtrace,
    },
    NamedExplicitDifferentSourceAndBacktrace {
        #[error(source)]
        err: T,
        #[error(backtrace)]
        backtrace: Backtrace,
    },
    UnnamedImplicitNoBacktraceFromSource(T),
    UnnamedExplicitNoBacktraceFromSource(#[error(not(backtrace))] T),
    UnnamedExplicitBacktraceFromSource(#[error(backtrace)] T),
    UnnamedImplicitDifferentSourceAndBacktrace(#[error(source)] T, Backtrace),
    UnnamedExplicitDifferentSourceAndBacktrace(
        #[error(source)] T,
        #[error(backtrace)] Backtrace,
    ),
}

impl<T: Error> BoundedTestErr<T> {
    fn get_stored_backtrace(&self) -> &Backtrace {
        match self {
            Self::NamedImplicitDifferentSourceAndBacktrace { backtrace, .. }
            | Self::NamedExplicitDifferentSourceAndBacktrace { backtrace, .. }
            | Self::UnnamedImplicitDifferentSourceAndBacktrace(_, backtrace)
            | Self::UnnamedExplicitDifferentSourceAndBacktrace(_, backtrace) => {
                backtrace
            }
            _ => panic!("ERROR IN TEST IMPLEMENTATION"),
        }
    }

    fn get_source_backtrace(&self) -> &Backtrace {
        any::request_ref(match self {
            Self::NamedExplicitBacktraceFromSource { err }
            | Self::NamedExplicitDifferentSourceAndBacktrace { err, .. }
            | Self::NamedImplicitDifferentSourceAndBacktrace { err, .. }
            | Self::UnnamedExplicitBacktraceFromSource(err)
            | Self::UnnamedExplicitDifferentSourceAndBacktrace(err, ..)
            | Self::UnnamedImplicitDifferentSourceAndBacktrace(err, ..) => err,
            _ => panic!("ERROR IN TEST IMPLEMENTATION"),
        })
        .unwrap()
    }
}

#[test]
fn named_implicit_no_backtrace_from_source() {
    let err = BoundedTestErr::NamedImplicitNoBacktraceFromSource {
        err: BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
    };

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_none());
    assert!(any::request_value::<i32>(&err).is_none());
}

#[test]
fn named_explicit_no_backtrace_from_source() {
    let err = BoundedTestErr::NamedExplicitNoBacktraceFromSource {
        err: BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
    };

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_none());
    assert!(any::request_value::<i32>(&err).is_none());
}

#[test]
fn named_explicit_backtrace_from_source() {
    let err = BoundedTestErr::NamedExplicitBacktraceFromSource {
        err: BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
    };

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_source_backtrace);
}

#[test]
fn named_implicit_different_source_and_backtrace() {
    let err = BoundedTestErr::NamedImplicitDifferentSourceAndBacktrace {
        err: BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
        backtrace: (|| Backtrace::force_capture())(), // ensure backtraces are different
    };

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_source_backtrace);
}

#[test]
fn named_explicit_different_source_and_backtrace() {
    let err = BoundedTestErr::NamedExplicitDifferentSourceAndBacktrace {
        err: BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
        backtrace: (|| Backtrace::force_capture())(), // ensure backtraces are different
    };

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_source_backtrace);
}

#[test]
fn unnamed_implicit_no_backtrace_from_source() {
    let err = BoundedTestErr::UnnamedImplicitNoBacktraceFromSource(BacktraceErr {
        backtrace: Backtrace::force_capture(),
    });

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_none());
    assert!(any::request_value::<i32>(&err).is_none());
}

#[test]
fn unnamed_explicit_no_backtrace_from_source() {
    let err = BoundedTestErr::UnnamedExplicitNoBacktraceFromSource(BacktraceErr {
        backtrace: Backtrace::force_capture(),
    });

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_none());
    assert!(any::request_value::<i32>(&err).is_none());
}

#[test]
fn unnamed_explicit_backtrace_from_source() {
    let err = BoundedTestErr::UnnamedExplicitBacktraceFromSource(BacktraceErr {
        backtrace: Backtrace::force_capture(),
    });

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_source_backtrace);
}

#[test]
fn unnamed_implicit_different_source_and_backtrace() {
    let err = BoundedTestErr::UnnamedImplicitDifferentSourceAndBacktrace(
        BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
        (|| Backtrace::force_capture())(), // ensure backtraces are different
    );

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_source_backtrace);
}

#[test]
fn unnamed_explicit_different_source_and_backtrace() {
    let err = BoundedTestErr::UnnamedExplicitDifferentSourceAndBacktrace(
        BacktraceErr {
            backtrace: Backtrace::force_capture(),
        },
        (|| Backtrace::force_capture())(), // ensure backtraces are different
    );

    assert!(err.source().is_some());
    assert!(any::request_ref::<Backtrace>(&err).is_some());
    assert_eq!(any::request_value::<i32>(&err), Some(42));
    assert_bt!(==, err, .get_stored_backtrace);
    assert_bt!(!=, err, .get_source_backtrace);
}
