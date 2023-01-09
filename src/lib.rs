//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    // Your state
    counter: i8,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    /// Your error
    OwnerError,
    IncrementError,
    DecrementError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    // Your code

    Ok(State { counter: 0 })
}

type IncrementVal = i8;
/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "counter",
    name = "increment",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn increment<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param > 0, Error::IncrementError);
    state.counter += param;
    Ok(())
}

#[receive(
    contract = "counter",
    name = "decrement",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn decrement<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param < 0, Error::DecrementError);
    state.counter += param;
    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "counter", name = "view", return_value = "i8")]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}

// #[concordium_cfg_test]
// mod tests {
//     use super::*;
//     use test_infrastructure::*;

//     type ContractResult<A> = Result<A, Error>;

//     #[concordium_test]
//     /// Test that initializing the contract succeeds with some state.
//     fn test_init() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         let state_result = init(&ctx, &mut state_builder);
//         state_result.expect_report("Contract initialization results in error");
//     }

//     #[concordium_test]
//     /// Test that invoking the `receive` endpoint with the `false` parameter
//     /// succeeds in updating the contract.
//     fn test_throw_no_error() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         // Initializing state
//         let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

//         let mut ctx = TestReceiveContext::empty();

//         let throw_error = false;
//         let parameter_bytes = to_bytes(&throw_error);
//         ctx.set_parameter(&parameter_bytes);

//         let mut host = TestHost::new(initial_state, state_builder);

//         // Call the contract function.
//         let result: ContractResult<()> = receive(&ctx, &mut host);

//         // Check the result.
//         claim!(result.is_ok(), "Results in rejection");
//     }

//     #[concordium_test]
//     /// Test that invoking the `receive` endpoint with the `true` parameter
//     /// results in the `YourError` being thrown.
//     fn test_throw_error() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         // Initializing state
//         let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

//         let mut ctx = TestReceiveContext::empty();

//         let throw_error = true;
//         let parameter_bytes = to_bytes(&throw_error);
//         ctx.set_parameter(&parameter_bytes);

//         let mut host = TestHost::new(initial_state, state_builder);

//         // Call the contract function.
//         let error: ContractResult<()> = receive(&ctx, &mut host);

//         // Check the result.
//         claim_eq!(
//             error,
//             Err(Error::YourError),
//             "Function should throw an error."
//         );
//     }
// }
