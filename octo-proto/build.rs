use std::io::Result;

fn main() -> Result<()> {
    // Only run full codegen if the `codegen` feature is enabled.
    // This avoids requiring protoc during development.
    if std::env::var("CARGO_FEATURE_CODGEN").is_ok() {
        let mut config = prost_build::Config::new();
        config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
        config.protoc_arg("--experimental_allow_proto3_optional");

        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile_protos_with_config(
                config,
                &[
                    "proto/octo.proto",
                    "proto/apb/api/data.proto",
                    "proto/apb/api/user.proto",
                    "proto/apb/api/quest.proto",
                    "proto/apb/api/gacha.proto",
                    "proto/apb/api/battle.proto",
                    "proto/apb/api/config.proto",
                    "proto/apb/api/tutorial.proto",
                    "proto/apb/api/gift.proto",
                    "proto/apb/api/gameplay.proto",
                    "proto/apb/api/gimmick.proto",
                    "proto/apb/api/notification.proto",
                    "proto/apb/api/cageornament.proto",
                    "proto/apb/api/deck.proto",
                    "proto/apb/api/friend.proto",
                    "proto/apb/api/loginbonus.proto",
                    "proto/apb/api/navicutin.proto",
                    "proto/apb/api/contentsstory.proto",
                    "proto/apb/api/dokan.proto",
                    "proto/apb/api/portalcage.proto",
                    "proto/apb/api/characterviewer.proto",
                    "proto/apb/api/mission.proto",
                    "proto/apb/api/shop.proto",
                    "proto/apb/api/costume.proto",
                    "proto/apb/api/movie.proto",
                    "proto/apb/api/omikuji.proto",
                    "proto/apb/api/weapon.proto",
                    "proto/apb/api/explore.proto",
                    "proto/apb/api/characterboard.proto",
                    "proto/apb/api/parts.proto",
                    "proto/apb/api/character.proto",
                    "proto/apb/api/companion.proto",
                    "proto/apb/api/material.proto",
                    "proto/apb/api/consumableitem.proto",
                    "proto/apb/api/sidestoryquest.proto",
                    "proto/apb/api/bighunt.proto",
                    "proto/apb/api/reward.proto",
                    "proto/apb/api/labyrinth.proto",
                    "proto/apb/api/banner.proto",
                    "proto/apb/api/admin.proto",
                    "proto/apb/api/characterreward.proto",
                ],
                &["proto/"],
            )?;
    }

    Ok(())
}
