mod errors; // Declare the module
pub use errors::*; // Optionally, re-export the errors

mod reader;
mod card;

use neon::prelude::*;

fn connect_js(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Simplified example: Always return Ok (you might want to handle errors and return them to JS)
    let _ = reader::connect::connect();
    Ok(cx.undefined())
}

fn listen_card_events_js(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Starting a new thread to listen to events to avoid blocking the main JS thread
    std::thread::spawn(move || {
        let _ = card::read::listen_card_events(); // Adapt this as necessary
    });

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("connect", connect_js)?;
    cx.export_function("listenCardEvents", listen_card_events_js)?;

    Ok(())
}
