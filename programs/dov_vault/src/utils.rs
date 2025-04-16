use anchor_lang::prelude::*;

pub fn now_ts() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

