pub mod errors;
pub mod reader;
pub mod card;

use std::cell::RefCell;
use neon::prelude::*;
pub use errors::ReaderError;
pub use reader::connect::*;
pub use card::read;
use pcsc;

type BoxedReader = JsBox<RefCell<RusticReader>>;

pub struct RusticReader {
    context: pcsc::Context,
    reader: String,
}

impl Finalize for RusticReader {}

impl RusticReader {
    fn new() -> Self {
        let (context, reader) = connect().or_else(|e| Err(e)).unwrap();
        RusticReader {
            context,
            reader,
        }
    }
}

impl RusticReader {
    fn connect(mut cx: FunctionContext) -> JsResult<BoxedReader> {
        let reader = RefCell::new(RusticReader::new());
        Ok(cx.boxed(reader))
    }

    fn get_reader_name(mut cx: FunctionContext) -> JsResult<JsString> {
        let reader = cx.argument::<BoxedReader>(0)?;
        let reader = reader.borrow();
        let info = format!("Reader: {:?}", reader.reader);
        Ok(cx.string(info))
    }
}



#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("connect", RusticReader::connect)?;
    cx.export_function("getReaderName", RusticReader::get_reader_name)?;

    Ok(())
}
