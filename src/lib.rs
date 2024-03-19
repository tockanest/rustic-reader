use std::cell::RefCell;

use neon::prelude::*;
use neon::types::buffer::TypedArray;
use pcsc;

pub use card::read;
pub use errors::ReaderError;
pub use reader::connect::*;

pub mod errors;
pub mod reader;
pub mod card;

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

    fn read_block(&self, block_number: u16) -> Vec<u8> {
        let read = read::read_block(self.context.clone(), self.reader.clone(), block_number).or_else(|e| Err(e)).unwrap();
        read
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

    fn js_read_block(mut cx: FunctionContext) -> JsResult<JsBuffer> {
        let reader = cx.argument::<BoxedReader>(0)?;
        let block_number = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;
        let reader = reader.borrow();
        let data = reader.read_ndef(block_number);
        let mut buffer = cx.buffer(data.len())?;
        buffer.as_mut_slice(&mut cx).copy_from_slice(&data);
        Ok(buffer)
    }
}



#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("connect", RusticReader::connect)?;
    cx.export_function("getReaderName", RusticReader::get_reader_name)?;
    cx.export_function("read", RusticReader::js_read_block)?;
    Ok(())
}
