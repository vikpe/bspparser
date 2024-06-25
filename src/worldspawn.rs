use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;

pub fn get_worldspawn_message(data: &[u8]) -> Result<String> {
    const MSG_NEEDLE: &[u8; 11] = br#""message" ""#;

    let Some(index_from) = data.find(MSG_NEEDLE).map(|index| index + MSG_NEEDLE.len()) else {
        return Err(e!("WorldSpawn message not found"));
    };
    let Some(index_to) = data[index_from..]
        .find_byte(b'"')
        .map(|index| index_from + index)
    else {
        return Err(e!("WorldSpawn message not found"));
    };

    Ok(data[index_from..index_to].to_str()?.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_get_worldspawn_message() -> Result<()> {
        assert_eq!(
            get_worldspawn_message(&[0; 32]).unwrap_err().to_string(),
            "WorldSpawn message not found".to_string()
        );

        assert_eq!(
            get_worldspawn_message(&read("tests/files/dm3_gpl.bsp")?)?,
            "The Abandoned Base".to_string()
        );

        assert_eq!(
            get_worldspawn_message(&read("tests/files/povdmm4.bsp")?)?,
            "DMM4 Arena\\nBy Povo-Hat (http://povo-hat.besmella-quake.com)\\n".to_string()
        );

        Ok(())
    }
}
