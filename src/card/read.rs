use pcsc::*;

use crate::errors::ReaderError;

fn authenticate_14443_3(tx: &Transaction) -> Result<(), ReaderError> {
    let block_number = 4; // Example block number
    let key_type = 0x60; // Key A

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

fn read_14443_3(tx: &Transaction, length: u8, block_number: u16, block_size: u8, packet_size: u8, read_class: u8) -> Result<Vec<u8>, ReaderError> {
    if length > packet_size {
        let p = (length / packet_size).div_ceil(0);
        let mut commands = Vec::new();

        //for (let i = 0; i < p; i++)
        for i in 0..p {
            let block = block_number + ((i as u16 * packet_size as u16) / block_size as u16);

            let size = if (i + 1) * packet_size < length {
                packet_size
            } else {
                length - i * packet_size
            };

            commands.push(read_14443_3(tx, size, block, block_size, packet_size, read_class));
        }

        let responses = commands.into_iter().map(|c| c.map_err(|e| e)).collect::<Result<Vec<_>, _>>()?;

        let mut data = Vec::new();
        for response in responses {
            data.extend_from_slice(&response);
        };

        Ok(data)
    } else {
        let packet = [
            read_class,
            0xb0,
            ((block_number >> 8) & 0xff) as u8, // High byte of block_number
            (block_number & 0xff) as u8,        // Low byte of block_number
            length,
        ];

        //First let's authenticate the card
        authenticate_14443_3(tx)?;

        let mut response_buf = [0; 256]; // Adjust the size as needed
        let response = tx.transmit(&packet, &mut response_buf).map_err(ReaderError::PcscError)?;

        if response.len() < 2 {
            return Err(ReaderError::CardError("Invalid response.".to_string(), Error::InvalidParameter));
        }

        //Status code is of UINT16BE type
        let status_code = ((response[response.len() - 2] as u16) << 8) | response[response.len() - 1] as u16;

        if status_code != 0x9000 {
            return Err(ReaderError::CardError("Read failed.".to_string(), Error::InvalidParameter));
        }

        Ok(response[0..response.len() - 2].to_vec())
    }
}

/// When accessing a Mifare Classic 1K card blocks with this library, blocks are numbered as follows:
///
///      sector 0:
///         block 0 – manufacturer data (read only)
///          block 1 – data block
///          block 2 – data block
///          block 3 – sector trailer
///      sector 1:
///          block 4 – data block
///          block 5 – data block
///          block 6 – data block
///          block 7 – sector trailer
///      sector 2:
///          block 8 – data block
///          block 9 – data block
///          block 10 – data block
///          block 11 – sector trailer
/// And so on. Be careful when accessing the blocks, as the sector trailer contains the access bits and the key A and key B. Overwriting the sector trailer can make the card unreadable explicit
pub fn read_block(ctx: Context, reader: String, block_number: u16) -> Result<Vec<u8>, ReaderError> {
    let reader = std::ffi::CString::new(reader).map_err(|_| ReaderError::UnsupportedReader("Invalid reader name".to_string()))?;

    let mut reader_states = vec![
        ReaderState::new(reader, State::UNAWARE)
    ];

    let mut last_event_count = reader_states[0].event_count();

    loop {
        ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

        loop {
            ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

            for reader_state in &mut reader_states {
                if reader_state.event_count() != last_event_count {
                    if reader_state.event_state().contains(State::CHANGED) {
                        if reader_state.event_state().contains(State::PRESENT) {

                            let mut card = ctx.connect(reader_state.name(), ShareMode::Shared, Protocols::ANY).map_err(ReaderError::PcscError)?;

                            {
                                let tx = card.transaction().map_err(ReaderError::PcscError)?;

                                // Get the card tag type: TAG_ISO_14443_3 is Mifare, TAG_ISO_14443_4 is FeliCa
                                let status = tx.status2_owned().map_err(ReaderError::PcscError)?;
                                let atr = status.atr();
                                return if atr.starts_with(&[0x3B, 0x8F, 0x80, 0x01, 0x80, 0x4F]) {
                                    let length = 16;
                                    let block_size = 4;
                                    let packet_size = 16;
                                    let read_class = 0xff;
                                    let data = read_14443_3(
                                        &tx,
                                        length,
                                        block_number,
                                        block_size,
                                        packet_size,
                                        read_class,
                                    )?;

                                    Ok(data)
                                } else if atr.starts_with(&[0xF0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]) {
                                    Err(ReaderError::CardError("Unsupported card type.".to_string(), Error::InvalidParameter))
                                } else {
                                    Err(ReaderError::CardError("Unsupported card type.".to_string(), Error::InvalidParameter))
                                }
                            }
                        }
                        // Sync the current state to the event state after handling the event
                        reader_state.sync_current_state();
                    }

                    last_event_count = reader_state.event_count();
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::connect;

    use super::*;

    #[test]
    fn test_listen_card_events() {
        let (ctx, reader) = connect().unwrap();
        println!("Connection successful. Reader: {}", reader);

        let result = read_block(ctx, reader, 4);
        match result {
            Ok(_) => println!("Reading successful."),
            Err(e) => panic!("Unexpected error occurred: {:?}", e),
        }
    }
}