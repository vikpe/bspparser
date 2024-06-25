use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Entity {
    Ambient {
        angle: Option<String>,
        classname: String,
        origin: String,
    },
    Ammo {
        classname: String,
        origin: String,
    },
    AirBubbles {
        origin: String,
    },
    DummyCheck {
        origin: String,
        targetname: String,
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
    FuncDoorSectret {
        angle: String,
        model: String,
        spawnflags: String,
        targetname: String,
    },
    FuncIllusionary {
        model: String,
    },
    FuncPlat {
        angle: String,
        model: String,
        origin: String,
        sounds: String,
    },
    FuncRotateEntity {
        origin: String,
        rotate: String,
        spawnflags: String,
        speed: String,
        target: String,
    },
    FuncTrain {
        dmg: String,
        model: String,
        speed: String,
        target: String,
        targetname: String,
    },
    FuncWall {
        model: String,
    },
    InfoCommand {
        origin: String,
        message: String,
    },
    InfoIntermission {
        mangle: String,
        origin: String,
    },
    InfoNotNull {
        angle: Option<String>,
        mangle: Option<String>,
        origin: String,
        spawnflags: Option<String>,
        targetname: String,
    },
    InfoNull {
        targetname: String,
    },
    InfoPlayer {
        classname: String,
        angle: String,
        origin: String,
    },
    InfoRotate {
        origin: String,
        targetname: String,
    },
    InfoTeleportDestination {
        angle: String,
        origin: String,
        targetname: String,
    },
    InfoTfgoal {
        count: String,
        goal_activation: String,
        goal_effects: String,
        origin: String,
        wait: String,
    },
    Item {
        classname: String,
        angle: Option<String>,
        origin: String,
        sounds: Option<String>,
        spawnflags: Option<String>,
    },
    Light {
        classname: String,
        light: String,
        origin: String,
        style: Option<String>,
    },
    MiscExplobox {
        origin: String,
    },
    MiscExplobox2 {
        origin: String,
    },
    MiscFireball {
        angle: Option<String>,
        light: Option<String>,
        origin: String,
        speed: String,
        style: Option<String>,
        wait: Option<String>,
    },
    Monster {
        classname: String,
        angle: Option<String>,
        origin: String,
        spawnflags: Option<String>,
        target: Option<String>,
        targetname: Option<String>,
    },
    PathCorner {
        origin: String,
        target: String,
        targetname: String,
        wait: Option<String>,
    },
    PointCheck {
        origin: String,
        light: Option<String>,
        style: Option<String>,
    },
    PointEnd {
        origin: String,
    },
    PointStart {
        origin: String,
    },
    PointZip {
        angle: Option<String>,
        origin: String,
        health: String,
        max_health: Option<String>,
    },
    RaceRouteMarker {
        angle: Option<String>,
        origin: String,
        size: Option<String>,
        target: Option<String>,
        targetname: String,
    },
    RaceRouteStart {
        model: Option<String>,
        origin: String,
        race_route_description: String,
        race_route_falsestart_mode: String,
        race_route_name: String,
        race_route_start_yaw: String,
        race_route_timeout: String,
        race_route_weapon_mode: String,
        target: String,
    },
    RotateObject {
        model: String,
        origin: String,
        target: String,
        targetname: String,
    },
    TrapSpikeshooter {
        angle: String,
        origin: String,
        targetname: String,
        wait: String,
    },
    TriggerChangeLevel {
        map: String,
        model: String,
    },
    TriggerCounter {
        count: String,
        model: String,
        target: String,
        targetname: String,
    },
    TriggerCheck {
        model: String,
        target: Option<String>,
        targetname: Option<String>,
    },
    TriggerEnd {
        model: String,
        targetname: String,
    },
    TriggerHurt {
        dmg: String,
        model: String,
    },
    TriggerMonsterjump {
        model: String,
        angle: String,
        height: Option<String>,
        speed: Option<String>,
    },
    TriggerMultiple {
        message: Option<String>,
        model: String,
        sounds: Option<String>,
        spawnflags: Option<String>,
        target: Option<String>,
        targetname: Option<String>,
        wait: Option<String>,
    },
    TriggerOnce {
        model: String,
        target: String,
    },
    TriggerPeace {
        model: String,
    },
    TriggerPush {
        angle: String,
        speed: String,
        model: String,
    },
    TriggerRelay {
        origin: String,
        targetname: String,
        killtarget: String,
    },
    TriggerSecret {
        model: String,
    },
    TriggerTeleport {
        model: String,
        origin: String,
        target: String,
    },
    Waypoint {
        origin: String,
        waypointitem: Option<String>,
        waypointnumber: String,
        waypointtype: String,
        wp0: Option<String>,
        wp1: Option<String>,
        wp2: Option<String>,
        wp3: Option<String>,
        wp4: Option<String>,
        wp5: Option<String>,
        wp6: Option<String>,
        wp7: Option<String>,
    },
    Weapon {
        classname: String,
        origin: String,
    },
    WorldSpawn {
        message: String,
        sounds: String,
        wad: String,
        worldtype: String,
    },
    Unknown {
        props: HashMap<String, String>,
    },
}

fn get_value(map: &HashMap<String, String>, key: &str) -> String {
    map.get(key).cloned().unwrap_or_default()
}

impl From<&HashMap<String, String>> for Entity {
    fn from(hmap: &HashMap<String, String>) -> Self {
        let Some(classname) = hmap.get("classname") else {
            return Entity::Unknown {
                props: hmap.clone(),
            };
        };

        let angle = get_value(hmap, "angle");
        let origin = get_value(hmap, "origin");
        let model = get_value(hmap, "model");
        let spawnflags = hmap.get("spawnflags").cloned();

        if classname.starts_with("monster_") {
            return Entity::Monster {
                classname: classname.clone(),
                angle: hmap.get("angle").cloned(),
                origin,
                spawnflags,
                target: hmap.get("target").cloned(),
                targetname: hmap.get("targetname").cloned(),
            };
        } else if classname.starts_with("item_") {
            return Entity::Item {
                classname: classname.clone(),
                angle: hmap.get("angle").cloned(),
                origin,
                sounds: hmap.get("sounds").cloned(),
                spawnflags,
            };
        } else if classname.starts_with("weapon_") {
            return Entity::Weapon {
                classname: classname.clone(),
                origin,
            };
        } else if classname.starts_with("ambient_") {
            return Entity::Ambient {
                angle: hmap.get("angle").cloned(),
                classname: classname.clone(),
                origin,
            };
        } else if classname.starts_with("light") {
            return Entity::Light {
                classname: classname.clone(),
                light: get_value(hmap, "light"),
                origin,
                style: hmap.get("style").cloned(),
            };
        } else if classname.starts_with("info_player_") {
            return Entity::InfoPlayer {
                classname: classname.clone(),
                angle,
                origin,
            };
        } else if classname.starts_with("ammo_") {
            return Entity::Ammo {
                classname: classname.clone(),
                origin,
            };
        }

        let targetname = get_value(hmap, "targetname");

        match hmap.get("classname") {
            Some(classname) => match classname.as_str() {
                "air_bubbles" => Entity::AirBubbles { origin },
                "dummy_check" => Entity::DummyCheck { origin, targetname },
                "func_button" => Entity::FuncButton {
                    angle,
                    model,
                    target: get_value(hmap, "target"),
                },
                "func_door" => Entity::FuncDoor {
                    angle,
                    model,
                    spawnflags,
                    targetname,
                    wait: get_value(hmap, "wait"),
                },
                "func_door_secret" => Entity::FuncDoorSectret {
                    angle,
                    model,
                    spawnflags: get_value(hmap, "spawnflags"),
                    targetname,
                },
                "func_illusionary" => Entity::FuncIllusionary { model },
                "func_plat" => Entity::FuncPlat {
                    angle,
                    origin,
                    model,
                    sounds: get_value(hmap, "sounds"),
                },
                "func_rotate_entity" => Entity::FuncRotateEntity {
                    origin,
                    rotate: get_value(hmap, "rotate"),
                    spawnflags: get_value(hmap, "spawnflags"),
                    speed: get_value(hmap, "speed"),
                    target: get_value(hmap, "target"),
                },
                "func_train" => Entity::FuncTrain {
                    dmg: get_value(hmap, "dmg"),
                    model,
                    speed: get_value(hmap, "speed"),
                    target: get_value(hmap, "target"),
                    targetname,
                },
                "func_wall" => Entity::FuncWall { model },
                "info_command" => Entity::InfoCommand {
                    origin,
                    message: get_value(hmap, "message"),
                },
                "info_tfgoal" => Entity::InfoTfgoal {
                    count: get_value(hmap, "count"),
                    goal_activation: get_value(hmap, "goal_activation"),
                    goal_effects: get_value(hmap, "goal_effects"),
                    origin,
                    wait: get_value(hmap, "wait"),
                },
                "info_intermission" => Entity::InfoIntermission {
                    mangle: get_value(hmap, "mangle"),
                    origin,
                },
                "info_notnull" => Entity::InfoNotNull {
                    angle: hmap.get("angle").cloned(),
                    mangle: hmap.get("mangle").cloned(),
                    origin,
                    spawnflags: hmap.get("spawnflags").cloned(),
                    targetname,
                },
                "info_null" => Entity::InfoNull { targetname },
                "info_rotate" => Entity::InfoRotate { origin, targetname },
                "info_teleport_destination" => Entity::InfoTeleportDestination {
                    angle,
                    origin,
                    targetname,
                },
                "misc_explobox" => Entity::MiscExplobox { origin },
                "misc_explobox2" => Entity::MiscExplobox2 { origin },
                "misc_fireball" => Entity::MiscFireball {
                    angle: hmap.get("angle").cloned(),
                    light: hmap.get("light").cloned(),
                    origin,
                    speed: get_value(hmap, "speed"),
                    style: hmap.get("style").cloned(),
                    wait: hmap.get("wait").cloned(),
                },
                "path_corner" => Entity::PathCorner {
                    origin,
                    target: get_value(hmap, "target"),
                    targetname,
                    wait: hmap.get("wait").cloned(),
                },
                "point_check" => Entity::PointCheck {
                    origin,
                    light: hmap.get("light").cloned(),
                    style: hmap.get("style").cloned(),
                },
                "point_end" => Entity::PointEnd { origin },
                "point_start" => Entity::PointStart { origin },
                "point_zip" => Entity::PointZip {
                    angle: hmap.get("angle").cloned(),
                    origin,
                    health: get_value(hmap, "health"),
                    max_health: hmap.get("max_health").cloned(),
                },
                "race_route_marker" => Entity::RaceRouteMarker {
                    angle: hmap.get("angle").cloned(),
                    origin,
                    size: hmap.get("size").cloned(),
                    target: hmap.get("target").cloned(),
                    targetname,
                },
                "race_route_start" => Entity::RaceRouteStart {
                    model: hmap.get("model").cloned(),
                    origin,
                    race_route_description: get_value(hmap, "race_route_description"),
                    race_route_falsestart_mode: get_value(hmap, "race_route_falsestart_mode"),
                    race_route_name: get_value(hmap, "race_route_name"),
                    race_route_start_yaw: get_value(hmap, "race_route_start_yaw"),
                    race_route_timeout: get_value(hmap, "race_route_timeout"),
                    race_route_weapon_mode: get_value(hmap, "race_route_weapon_mode"),
                    target: get_value(hmap, "target"),
                },
                "rotate_object" => Entity::RotateObject {
                    model,
                    target: get_value(hmap, "target"),
                    targetname,
                    origin,
                },
                "trap_spikeshooter" => Entity::TrapSpikeshooter {
                    angle,
                    origin,
                    targetname,
                    wait: get_value(hmap, "wait"),
                },
                "trigger_changelevel" => Entity::TriggerChangeLevel {
                    map: get_value(hmap, "map"),
                    model,
                },
                "trigger_check" => Entity::TriggerCheck {
                    model,
                    target: hmap.get("target").cloned(),
                    targetname: hmap.get("targetname").cloned(),
                },
                "trigger_counter" => Entity::TriggerCounter {
                    count: get_value(hmap, "count"),
                    model,
                    target: get_value(hmap, "target"),
                    targetname,
                },
                "trigger_end" => Entity::TriggerEnd { model, targetname },
                "trigger_hurt" => Entity::TriggerHurt {
                    dmg: get_value(hmap, "dmg"),
                    model,
                },
                "trigger_monsterjump" => Entity::TriggerMonsterjump {
                    model,
                    angle,
                    height: hmap.get("height").cloned(),
                    speed: hmap.get("speed").cloned(),
                },
                "trigger_multiple" => Entity::TriggerMultiple {
                    message: hmap.get("message").cloned(),
                    model,
                    spawnflags,
                    target: hmap.get("target").cloned(),
                    targetname: hmap.get("targetname").cloned(),
                    sounds: hmap.get("sounds").cloned(),
                    wait: hmap.get("wait").cloned(),
                },
                "trigger_once" => Entity::TriggerOnce {
                    model,
                    target: get_value(hmap, "target"),
                },
                "trigger_peace" => Entity::TriggerPeace { model },
                "trigger_push" => Entity::TriggerPush {
                    angle,
                    speed: get_value(hmap, "speed"),
                    model,
                },
                "trigger_relay" => Entity::TriggerRelay {
                    origin,
                    targetname,
                    killtarget: get_value(hmap, "killtarget"),
                },
                "trigger_secret" => Entity::TriggerSecret { model },
                "trigger_teleport" => Entity::TriggerTeleport {
                    origin,
                    model,
                    target: get_value(hmap, "target"),
                },
                "waypoint" => Entity::Waypoint {
                    origin,
                    waypointitem: hmap.get("waypointitem").cloned(),
                    waypointnumber: get_value(hmap, "waypointnumber"),
                    waypointtype: get_value(hmap, "waypointtype"),
                    wp0: hmap.get("wp0").cloned(),
                    wp1: hmap.get("wp1").cloned(),
                    wp2: hmap.get("wp2").cloned(),
                    wp3: hmap.get("wp3").cloned(),
                    wp4: hmap.get("wp4").cloned(),
                    wp5: hmap.get("wp5").cloned(),
                    wp6: hmap.get("wp6").cloned(),
                    wp7: hmap.get("wp7").cloned(),
                },
                "worldspawn" => Entity::WorldSpawn {
                    message: get_value(hmap, "message"),
                    sounds: get_value(hmap, "sounds"),
                    wad: get_value(hmap, "wad"),
                    worldtype: get_value(hmap, "worldtype"),
                },
                _ => Entity::Unknown {
                    props: hmap.clone(),
                },
            },
            _ => Entity::Unknown {
                props: hmap.clone(),
            },
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
            Entity::Ambient {
                angle: None,
                classname: "ambient_comp_hum".to_string(),
                origin: "1 2 3".to_string(),
            }
        );

        Ok(())
    }
}
