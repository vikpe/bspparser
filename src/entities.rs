use std::collections::HashMap;

use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;

use crate::entity::Entity;

pub fn get_entities(data: &[u8]) -> Result<Vec<Entity>> {
    let entities = get_entities_maps(data)?
        .iter()
        .map(|em| em.into())
        .collect();
    Ok(entities)
}

fn get_entities_maps(data: &[u8]) -> Result<Vec<HashMap<String, String>>> {
    let ent_string = get_entities_string(data)?;

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

fn get_entities_string(data: &[u8]) -> Result<String> {
    let needle_from: &[u8] = &[123, 10, 34];
    let Some(index_from) = data.find(needle_from) else {
        return Err(e!("Entities not found (start needle)"));
    };

    let needle_to = &[10, 125, 10, 0];
    let Some(index_to) = data[index_from..]
        .find(needle_to)
        .map(|i| index_from + i + 2)
    else {
        return Err(e!("Entities not found (end needle)"));
    };

    Ok(data[index_from..index_to].to_str()?.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_get_entities() -> Result<()> {
        let demo_data = fs::read("test_files/dm3_gpl.bsp")?;
        let entities = get_entities(&demo_data)?;
        assert_eq!(entities.len(), 211);

        assert_eq!(
            entities[0],
            Entity::WorldSpawn {
                message: "The Abandoned Base".to_string(),
                sounds: "6".to_string(),
                wad: "gfx/base.wad".to_string(),
                worldtype: "2".to_string(),
            }
        );

        assert_eq!(
            entities[210],
            Entity::InfoIntermission {
                mangle: "20 240 0".to_string(),
                origin: "1840 256 64".to_string(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_entities_maps() -> Result<()> {
        {
            let demo_data = fs::read("test_files/dm3_gpl.bsp")?;
            let entities = get_entities_maps(&demo_data)?;
            assert_eq!(entities.len(), 211);
            assert_eq!(
                entities.get(0),
                Some(&HashMap::from([
                    ("wad".to_string(), "gfx/base.wad".to_string()),
                    ("classname".to_string(), "worldspawn".to_string()),
                    ("worldtype".to_string(), "2".to_string()),
                    ("sounds".to_string(), "6".to_string()),
                    ("message".to_string(), "The Abandoned Base".to_string()),
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
    fn test_entities_text() -> Result<()> {
        {
            let demo_data = fs::read("test_files/dm3_gpl.bsp")?;
            let result = get_entities_string(&demo_data)?;
            let expected = fs::read_to_string("test_files/dm3_gpl.entities")?;
            assert_eq!(result, expected);
        }
        {
            let demo_data = fs::read("test_files/povdmm4.bsp")?;
            let result = get_entities_string(&demo_data)?;
            let expected = fs::read_to_string("test_files/povdmm4.entities")?;
            assert_eq!(result, expected);
        }
        Ok(())
    }
}
