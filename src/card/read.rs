use crate::errors::ReaderError;
use crate::reader::connect::connect;
use pcsc::*;

pub fn listen_card_events() -> Result<(), ReaderError> {
    let (ctx, reader) = connect()?;
    let reader = std::ffi::CString::new(reader).map_err(|_| ReaderError::UnsupportedReader("Invalid reader name".to_string()))?;

    let mut reader_states = vec![
        ReaderState::new(reader, State::UNAWARE)
    ];

    loop {
        ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

        loop {
            ctx.get_status_change(None, &mut reader_states).map_err(ReaderError::PcscError)?;

            for reader_state in &mut reader_states {
                if reader_state.event_state().contains(State::CHANGED) {
                    if reader_state.event_state().contains(State::EMPTY) {
                        println!("Card removed");
                    } else if reader_state.event_state().contains(State::PRESENT) {
                        println!("Card inserted");
                    }
                    // Sync the current state to the event state after handling the event
                    reader_state.sync_current_state();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listen_card_events() {
        let result = listen_card_events();
        match result {
            Ok(_) => println!("Listening successful."),
            Err(e) => match e {
                ReaderError::UnsupportedReader(_) => println!("Unsupported reader connected."),
                _ => panic!("Unexpected error occurred."),
            },
        }
    }
}