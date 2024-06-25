use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Entity {
    AmbientCompHum {
        origin: String,
    },
    FuncButton {
        angle: String,
        model: String,
        target: String,
    },
    FuncDoor {
        angle: String,
        model: String,
        spawnflags: Option<String>,
        targetname: String,
        wait: String,
    },
    FuncPlat {
        angle: String,
        model: String,
        origin: String,
        sounds: String,
    },
    FuncWall {
        model: String,
    },
    InfoIntermission {
        mangle: String,
        origin: String,
    },
    InfoPlayerDeathmatch {
        angle: String,
        origin: String,
    },
    InfoPlayerStart {
        angle: String,
        origin: String,
    },
    InfoTeleportDestination {
        angle: String,
        origin: String,
        targetname: String,
    },
    ItemArmor1 {
        origin: String,
    },
    ItemArmor2 {
        origin: String,
    },
    ItemArmorInv {
        origin: String,
    },
    ItemArtifactInvisibility {
        origin: String,
    },
    ItemArtifactInvulnerability {
        origin: String,
    },
    ItemArtifactSuperDamage {
        origin: String,
    },
    ItemCells {
        origin: String,
        spawnflags: Option<String>,
    },
    ItemHealth {
        origin: String,
        spawnflags: Option<String>,
    },
    ItemRockets {
        origin: String,
        spawnflags: Option<String>,
    },
    ItemShells {
        origin: String,
        spawnflags: Option<String>,
    },
    ItemSpikes {
        origin: String,
        spawnflags: Option<String>,
    },
    Light {
        light: String,
        origin: String,
    },
    LightFluoro {
        origin: String,
    },
    TriggerChangeLevel {
        map: String,
        model: String,
    },
    TriggerTeleport {
        model: String,
        origin: String,
        target: String,
    },
    WeaponGrenadelauncher {
        origin: String,
    },
    WeaponLightning {
        origin: String,
    },
    WeaponNailgun {
        origin: String,
    },
    WeaponRocketlauncher {
        origin: String,
    },
    WeaponSupernailgun {
        origin: String,
    },
    WeaponSupershotgun {
        origin: String,
    },
    WorldSpawn {
        message: String,
        sounds: String,
        wad: String,
        worldtype: String,
    },
    Unknown {
        classname: Option<String>,
    },
}

fn get_value(map: &HashMap<String, String>, key: &str) -> String {
    map.get(key).cloned().unwrap_or_default()
}

impl From<&HashMap<String, String>> for Entity {
    fn from(hmap: &HashMap<String, String>) -> Self {
        let angle = get_value(hmap, "angle");
        let origin = get_value(hmap, "origin");
        let model = get_value(hmap, "model");
        let spawnflags = hmap.get("spawnflags").cloned();

        match hmap.get("classname") {
            Some(classname) => match classname.as_str() {
                "ambient_comp_hum" => Entity::AmbientCompHum { origin },
                "func_button" => Entity::FuncButton {
                    angle,
                    model,
                    target: get_value(hmap, "target"),
                },
                "func_door" => Entity::FuncDoor {
                    angle,
                    model,
                    spawnflags,
                    targetname: get_value(hmap, "targetname"),
                    wait: get_value(hmap, "wait"),
                },
                "func_plat" => Entity::FuncPlat {
                    angle,
                    origin,
                    model,
                    sounds: get_value(hmap, "sounds"),
                },
                "func_wall" => Entity::FuncWall { model },
                "info_intermission" => Entity::InfoIntermission {
                    mangle: get_value(hmap, "mangle"),
                    origin,
                },
                "info_player_deathmatch" => Entity::InfoPlayerDeathmatch { origin, angle },
                "info_player_start" => Entity::InfoPlayerStart { origin, angle },
                "info_teleport_destination" => Entity::InfoTeleportDestination {
                    angle,
                    origin,
                    targetname: get_value(hmap, "targetname"),
                },
                "item_armor1" => Entity::ItemArmor1 { origin },
                "item_armor2" => Entity::ItemArmor2 { origin },
                "item_armorInv" => Entity::ItemArmorInv { origin },
                "item_artifact_invisibility" => Entity::ItemArtifactInvisibility { origin },
                "item_artifact_invulnerability" => Entity::ItemArtifactInvulnerability { origin },
                "item_artifact_super_damage" => Entity::ItemArtifactSuperDamage { origin },
                "item_cells" => Entity::ItemCells { origin, spawnflags },
                "item_health" => Entity::ItemHealth { origin, spawnflags },
                "item_rockets" => Entity::ItemRockets { origin, spawnflags },
                "item_shells" => Entity::ItemShells { origin, spawnflags },
                "item_spikes" => Entity::ItemSpikes { origin, spawnflags },
                "light" => Entity::Light {
                    light: get_value(hmap, "light"),
                    origin,
                },
                "light_fluoro" => Entity::LightFluoro { origin },
                "trigger_changelevel" => Entity::TriggerChangeLevel {
                    map: get_value(hmap, "map"),
                    model,
                },
                "trigger_teleport" => Entity::TriggerTeleport {
                    origin,
                    model,
                    target: get_value(hmap, "target"),
                },
                "weapon_grenadelauncher" => Entity::WeaponGrenadelauncher { origin },
                "weapon_lightning" => Entity::WeaponLightning { origin },
                "weapon_nailgun" => Entity::WeaponNailgun { origin },
                "weapon_rocketlauncher" => Entity::WeaponRocketlauncher { origin },
                "weapon_supernailgun" => Entity::WeaponSupernailgun { origin },
                "weapon_supershotgun" => Entity::WeaponSupershotgun { origin },
                "worldspawn" => Entity::WorldSpawn {
                    message: get_value(hmap, "message"),
                    sounds: get_value(hmap, "sounds"),
                    wad: get_value(hmap, "wad"),
                    worldtype: get_value(hmap, "worldtype"),
                },
                _ => Entity::Unknown {
                    classname: Some(classname.to_string()),
                },
            },
            _ => Entity::Unknown { classname: None },
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_from_hashmap() -> Result<()> {
        assert_eq!(
            Entity::from(&HashMap::from([
                ("classname".to_string(), "ambient_comp_hum".to_string()),
                ("origin".to_string(), "1 2 3".to_string()),
            ])),
            Entity::AmbientCompHum {
                origin: "1 2 3".to_string(),
            }
        );

        Ok(())
    }
}
