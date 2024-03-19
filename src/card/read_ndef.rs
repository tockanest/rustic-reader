use std::ffi::CString;
use pcsc::{Context, Error, Protocols, ReaderState, ShareMode, State, Transaction};
use crate::ReaderError;

fn authenticate_14443_3(tx: &Transaction, block_number: u8) -> Result<(), ReaderError> {
    let key_type = 0x61; // Key B

    let command = [
        0xff,
        0x86,
        0x00,
        0x00,
        0x05,
        0x01,
        0x00,
        block_number,
        key_type,
        0x00,
    ];

    let mut response_buf = [0; 512]; // Adjust the size as needed

    let response = tx.transmit(&command, &mut response_buf).map_err(ReaderError::PcscError)?;

    if response.len() >= 2 {
        let sw1 = response[response.len() - 2];
        let sw2 = response[response.len() - 1];

        if sw1 == 0x90 && sw2 == 0x00 {
            Ok(())
        } else {
            //Return error
            return Err(ReaderError::CardError("Authentication Failed.".to_string(), Error::CardNotAuthenticated));
        }
    } else {
        //Return error
        return Err(ReaderError::CardError("Invalid response.".to_string(), Error::InvalidParameter));
    }
}

fn read_all_blocks(tx: &Transaction) -> Result<Vec<u8>, ReaderError> {
    let mut accessible_blocks = Vec::new();
    let mut all_data = Vec::new(); // To accumulate data from all blocks

    for sector in 0..16 {
        for block in 0..3 {
            let block_number = sector * 4 + block;
            accessible_blocks.push(block_number);
        }
    }

    accessible_blocks.retain(|&x| x != 0);

    for block in accessible_blocks {
        let command = [
            0xff,
            0xb0,
            ((block >> 8) & 0xff) as u8,
            (block & 0xff) as u8,
            16,
        ];

        authenticate_14443_3(tx, block as u8).map_err(|e| e)?;

        let mut response_buf = [0; 512]; // Adjust the size as needed

        let response = tx.transmit(&command, &mut response_buf).map_err(ReaderError::PcscError)?;

        if response.len() >= 2 {
            let status_code = ((response[response.len() - 2] as u16) << 8) | (response[response.len() - 1] as u16);

            if status_code != 0x9000 {
                return Err(ReaderError::CardError("Read failed".to_string(), Error::InvalidParameter));
            } else {
                // Accumulate the data from this block
                all_data.extend_from_slice(&response[0..response.len() - 2]);
            }
        } else {
            return Err(ReaderError::CardError("Invalid response.".to_string(), Error::InvalidParameter));
        }
    }

    Ok(all_data) // Return all accumulated data
}


fn handle_card_connection(ctx: Context, reader_name: &CString) -> Result<Vec<u8>, ReaderError> {
    let mut card = ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY).map_err(ReaderError::PcscError)?;
    let tx = card.transaction().map_err(ReaderError::PcscError)?;
    let status = tx.status2_owned().map_err(ReaderError::PcscError)?;
    let atr = status.atr();

    if atr.starts_with(&[0x3B, 0x8F, 0x80, 0x01, 0x80, 0x4F]) {

        let data = read_all_blocks(&tx).map_err(|e| e)?;
        Ok(data)

    } else {
        Err(ReaderError::CardError("Unsupported card type.".to_string(), Error::InvalidParameter))
    }
}

pub fn read_ndef(ctx: Context, reader: String) -> Result<Vec<u8>, ReaderError> {
    let new_reader = std::ffi::CString::new(reader).map_err(|_| ReaderError::UnsupportedReader("Invalid reader name".to_string()))?;

    let mut reader_states = vec![
        ReaderState::new(new_reader.clone(), State::UNAWARE)
    ];

    let mut last_event_count = reader_states[0].event_count();

    loop {
        ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

        loop {
            ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

            for reader_state in &mut reader_states {
                if reader_state.event_count() == last_event_count {
                    continue;
                }

                if !reader_state.event_state().contains(State::CHANGED) {
                    last_event_count = reader_state.event_count();
                    continue;
                }

                if !reader_state.event_state().contains(State::PRESENT) {
                    reader_state.sync_current_state();
                    last_event_count = reader_state.event_count();
                    continue;
                }

                let card_result = handle_card_connection(ctx, &new_reader).map_err(|e| e)?;
                return Ok(card_result);

            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcsc::Context;
    use crate::connect;

    #[test]
    fn test_read_ndef() {
        let (ctx, reader) = connect().unwrap();
        let result = read_ndef(ctx, reader);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}