use std::collections::HashMap;

use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;

pub fn entities_as_hashmaps(data: &[u8]) -> Result<Vec<HashMap<String, String>>> {
    let ent_string = entities_as_string(data)?;

    let mut entities = Vec::new();
    let mut current_entity = HashMap::new();

    for line in ent_string.lines() {
        let line = line.trim();

        if line == "{" {
            current_entity = HashMap::new();
        } else if line == "}" {
            entities.push(current_entity.clone());
        } else {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let key = parts[0].trim_matches('"').to_string();
                let value = parts[1].trim_matches('"').to_string();
                current_entity.insert(key, value);
            }
        }
    }

    Ok(entities)
}

pub fn entities_as_string(data: &[u8]) -> Result<String> {
    let Some(index_worldspawn) = data.find(br#""worldspawn""#) else {
        return Err(e!("Entities not found (missing worldspawn)"));
    };

    let Some(index_from) = data[..index_worldspawn].rfind(br#"{"#) else {
        return Err(e!("Entities not found (first opening brace)"));
    };

    let Some(index_nullterm) = data[index_from..].find_byte(0).map(|i| index_from + i) else {
        return Err(e!("Entities not found (null terminator)"));
    };

    let Some(index_to) = data[..index_nullterm].rfind(b"}") else {
        return Err(e!("Entities not found (last closing brace)"));
    };

    let plain_data: Vec<u8> = data[index_from..=index_to]
        .iter()
        .map(|b| *b & 127) // strip color
        .collect();

    let Ok(ent_str) = plain_data.to_str() else {
        println!(
            "range: {}-{} ({})",
            index_from,
            index_to,
            index_to - index_from
        );
        return Err(e!("Entities not found (invalid UTF-8)"));
    };

    Ok(ent_str.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_entities_as_hashmaps() -> Result<()> {
        {
            let demo_data = fs::read("tests/files/dm3_gpl.bsp")?;
            let entities = entities_as_hashmaps(&demo_data)?;
            assert_eq!(entities.len(), 211);
            assert_eq!(
                entities.get(0),
                Some(&HashMap::from([
                    ("classname".to_string(), "worldspawn".to_string()),
                    ("message".to_string(), "The Abandoned Base".to_string()),
                    ("sounds".to_string(), "6".to_string()),
                    ("wad".to_string(), "gfx/base.wad".to_string()),
                    ("worldtype".to_string(), "2".to_string()),
                ]))
            );
            assert_eq!(
                entities.get(210),
                Some(&HashMap::from([
                    ("classname".to_string(), "info_intermission".to_string()),
                    ("origin".to_string(), "1840 256 64".to_string()),
                    ("mangle".to_string(), "20 240 0".to_string()),
                ]))
            );
        }
        Ok(())
    }

    #[test]
    fn test_entities_as_string() -> Result<()> {
        {
            let demo_data = fs::read("tests/files/dm3_gpl.bsp")?;
            let result = entities_as_string(&demo_data)?;
            let expected = fs::read_to_string("tests/files/dm3_gpl.entities")?;
            assert_eq!(result, expected);
        }
        {
            let demo_data = fs::read("tests/files/povdmm4.bsp")?;
            let result = entities_as_string(&demo_data)?;
            let expected = fs::read_to_string("tests/files/povdmm4.entities")?;
            assert_eq!(result, expected);
        }
        Ok(())
    }
}
