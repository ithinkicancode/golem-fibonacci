cargo_component_bindings::generate!();

use crate::bindings::exports::golem::fib::api::*;
use lib::fibonacci::{FibState, FIB_0};

struct AppState(FibState);

static mut APP_STATE: AppState = AppState(FIB_0);

#[allow(unsafe_code)]
fn with_app_state<T>(f: impl FnOnce(&mut AppState) -> T) -> T {
    unsafe { f(&mut APP_STATE) }
}

struct Component;

impl Guest for Component {
    fn next() -> Result<u64, String> {
        with_app_state(|AppState(fib)| {
            let value = fib.value();

            let number = u64::try_from(value)
                .map_err(|e| format!("ERROR converting {} to u64: {}", value, e))?;

            fib.calc_next();

            Ok(number)
        })
    }

    fn reset() {
        with_app_state(|AppState(fib)| {
            fib.reset();
        });
    }
}
