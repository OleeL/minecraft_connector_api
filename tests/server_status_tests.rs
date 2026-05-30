use minecraft_connector_api::server_status::ServerStatus;

#[test]
fn deserializes_basic_server_status_response() {
    let payload = r#"
    {
        "version": {
            "name": "1.20.4",
            "protocol": 765
        },
        "players": {
            "max": 20,
            "online": 3,
            "sample": [
                {
                    "name": "Steve",
                    "id": "00000000-0000-0000-0000-000000000000"
                }
            ]
        },
        "description": {
            "text": "A Minecraft Server"
        },
        "favicon": "data:image/png;base64,abc123"
    }
    "#;

    let status: ServerStatus = serde_json::from_str(payload).unwrap();

    assert_eq!(status.version.name, "1.20.4");
    assert_eq!(status.version.protocol, 765);
    assert_eq!(status.players.max, 20);
    assert_eq!(status.players.online, 3);
    assert_eq!(status.players.sample.unwrap()[0].name, "Steve");
    assert_eq!(status.description.text, "A Minecraft Server");
    assert_eq!(status.favicon, "data:image/png;base64,abc123");
    assert!(status.modinfo.is_none());
}

#[test]
fn deserializes_modinfo_type_field() {
    let payload = r#"
    {
        "version": {
            "name": "Forge 1.12.2",
            "protocol": 340
        },
        "players": {
            "max": 10,
            "online": 1
        },
        "description": {
            "text": "Modded server"
        },
        "favicon": "",
        "modinfo": {
            "type": "FML",
            "modList": [
                {
                    "modid": "examplemod",
                    "version": "1.0.0"
                }
            ]
        }
    }
    "#;

    let status: ServerStatus = serde_json::from_str(payload).unwrap();
    let modinfo = status.modinfo.unwrap();

    assert_eq!(modinfo.type_field, "FML");
    assert_eq!(modinfo.mod_list.len(), 1);
    assert_eq!(modinfo.mod_list[0]["modid"], "examplemod");
}
