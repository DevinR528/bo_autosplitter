#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{
    future::next_tick,
    game_engine::unity::{get_scene_name, mono::Module, SceneManager},
    settings::Gui,
    timer, Process,
};

mod game_data;
mod settings;

use game_data::{AbilityManager, BetaPlayerDataManager, GameManager, QuestManager};
use settings::Settings;

asr::async_main!(stable);

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    asr::print_message("Bo AutoSplitter ON!!");

    loop {
        let process = Process::wait_attach("Bo.exe").await;

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let module = Module::wait_attach_auto_detect(&process).await;
                asr::print_message("Found mono");
                let img = module.wait_get_default_image(&process).await;
                asr::print_message("Found Assembly-CSharp");

                let scene_manager = SceneManager::wait_attach(&process).await;
                asr::print_message("Attached SceneManager");

                let mut old_scene_name = None;
                if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process) {
                    let name = String::from_utf8_lossy(get_scene_name(path.as_bytes())).to_string();
                    asr::print_message(&name);
                    old_scene_name = Some(name);
                }

                let game_manager = GameManager::bind(&process, &module, &img).await;
                asr::print_message("got GameManager");
                let game_manager_inst = game_manager
                    .class()
                    .wait_get_static_instance(&process, &module, "instance")
                    .await;
                asr::print_message(&format!("got GameManager instance {:?}", game_manager_inst));
                let mut old_game_manager = None;
                if let Ok(game_manager) = game_manager.read(&process, game_manager_inst) {
                    asr::print_message(&format!("{:#?}", game_manager));
                    old_game_manager = Some(game_manager);
                }

                let quest_manager = QuestManager::bind(&process, &module, &img).await;
                asr::print_message("got QuestManager");
                let mut old_quest_manager = None;
                if let Ok(quest_manager) =
                    quest_manager.read(&process, old_game_manager.unwrap().quest_pointer.into())
                {
                    asr::print_message(&format!("{:#?}", quest_manager));
                    old_quest_manager = Some(quest_manager);
                }

                let player_manager = BetaPlayerDataManager::bind(&process, &module, &img).await;
                asr::print_message("got BetaPlayerDataManager");
                let mut old_player_manager = None;
                if let Ok(player_manager) = player_manager.read(
                    &process,
                    old_game_manager.unwrap().player_data_pointer.into(),
                ) {
                    asr::print_message(&format!("{:#?}", player_manager));
                    old_player_manager = Some(player_manager);
                }

                let ability_manager = AbilityManager::bind(&process, &module, &img).await;
                asr::print_message("got AbilityManager");
                let mut old_ability_manager = None;
                if let Ok(ability_manager) = ability_manager.read(
                    &process,
                    old_game_manager.unwrap().ability_pointer.into(),
                ) {
                    asr::print_message(&format!("{:#?}", ability_manager));
                    old_ability_manager = Some(ability_manager);
                }

                let mut paused = false;
                loop {
                    settings.update();

                    // Need to update timer first (before scene) so starting the timer
                    // happens correctly
                    match player_manager.read(
                        &process,
                        old_game_manager.unwrap().player_data_pointer.into(),
                    ) {
                        Ok(player_manager) if old_player_manager != Some(player_manager) => {
                            match (player_manager, old_player_manager) {
                                // Start timer for the first time
                                (
                                    BetaPlayerDataManager { time_played },
                                    Some(BetaPlayerDataManager {
                                        time_played: old_time,
                                    }),
                                ) if (time_played > 0.0 && old_time == 0.0)
                                    && old_scene_name.as_deref() != Some("New Main Menu") =>
                                {
                                    asr::print_message(&format!(
                                        "timer started!!! {:?} {:?}",
                                        player_manager, old_player_manager
                                    ));
                                    timer::start();
                                }
                                // Timer is paused because of loading screen
                                (
                                    BetaPlayerDataManager { time_played },
                                    Some(BetaPlayerDataManager {
                                        time_played: old_time,
                                    }),
                                ) if time_played == old_time => {
                                    asr::print_message(&format!(
                                        "PAUSE {:?} {:?}",
                                        player_manager, old_player_manager
                                    ));
                                    timer::pause_game_time();
                                    paused = true;
                                }
                                // Restart timer from load screen
                                (
                                    BetaPlayerDataManager { time_played },
                                    Some(BetaPlayerDataManager {
                                        time_played: old_time,
                                    }),
                                ) if paused && time_played != old_time => {
                                    asr::print_message(&format!(
                                        "RESUME {:?} {:?}",
                                        player_manager, old_player_manager
                                    ));
                                    timer::resume_game_time();
                                    paused = false;
                                }
                                (_, _) => {}
                            }

                            old_player_manager = Some(player_manager);
                        }
                        Err(err) => asr::print_message(&format!("Player_manager ERROR: {:?}", err)),
                        _ => {}
                    }

                    let mut scene_name = None;
                    if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process) {
                        let name =
                            String::from_utf8_lossy(get_scene_name(path.as_bytes())).to_string();
                        if name == "New Main Menu" {
                            old_player_manager = Some(BetaPlayerDataManager { time_played: 0.0 });
                        }

                        // Update the scene we track
                        scene_name = Some(name);
                    }
                    if old_scene_name != scene_name {
                        asr::print_message(&format!("new secene {:#?}", scene_name));
                        old_scene_name = scene_name
                    }

                    match game_manager.read(&process, game_manager_inst) {
                        Ok(game_manager) if old_game_manager != Some(game_manager) => {
                            asr::print_message(&format!("{:#?}", game_manager));
                            // This forces update to the QuestManager object, if it moves we must read from the new address
                            match old_game_manager.map(|gm| gm.quest_pointer) {
                                Some(old_quest_ptr)
                                    if old_quest_ptr != game_manager.quest_pointer =>
                                {
                                    old_quest_manager = None;
                                }
                                _ => {}
                            }
                            // This forces update to the AbilityManager object, if it moves we must read from the new address
                            match old_game_manager.map(|gm| gm.ability_pointer) {
                                Some(old_ability_ptr)
                                    if old_ability_ptr != game_manager.ability_pointer =>
                                {
                                    old_ability_manager = None;
                                }
                                _ => {}
                            }
                            asr::print_message(&format!("{:?} {:?}", old_ability_manager.unwrap(), game_manager));

                            old_game_manager = Some(game_manager);
                        }
                        Err(err) => asr::print_message(&format!("game manager ERROR: {:?}", err)),
                        _ => {}
                    }

                    match quest_manager
                        .read(&process, old_game_manager.unwrap().quest_pointer.into())
                    {
                        Ok(quest_manager) if old_quest_manager != Some(quest_manager) => {
                            asr::print_message(&format!("{:#?}", quest_manager));
                            // SPLITS
                            // Here we check when each quest is updated, basically we just check when each one is completed
                            //
                            // Asahi staff question complete
                            match old_quest_manager.map(|qm| qm.asahi_staff_end) {
                                Some(false)
                                    if quest_manager.asahi_staff_end
                                        && settings.asahi_staff_end =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Asahi Eye of Beast end
                            match old_quest_manager.map(|qm| qm.asahi_eye_of_beast_end) {
                                Some(false)
                                    if quest_manager.asahi_eye_of_beast_end
                                        && settings.asahi_eye_of_beast_end =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Shimeji Armapillos Collection quest complete
                            match old_quest_manager.map(|qm| qm.shimeji_armapillos_collect) {
                                Some(3)
                                    if quest_manager.shimeji_armapillos_collect == 4
                                        && settings.shimeji_quest_end =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Particularly Unmanageable Armadillo
                            match old_quest_manager.map(|qm| qm.defeat_pua_boss) {
                                Some(false)
                                    if quest_manager.defeat_pua_boss
                                        && settings.defeated_pua_boss =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Hashihime
                            match old_quest_manager.map(|qm| qm.defeat_hashihime_boss) {
                                Some(false)
                                    if quest_manager.defeat_hashihime_boss
                                        && settings.defeat_hashihime_boss =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Vermilion Stranger quest end
                            match old_quest_manager.map(|qm| qm.vermilion_stranger_quest_end) {
                                Some(false)
                                    if quest_manager.vermilion_stranger_quest_end
                                        && settings.vermilion_stranger_quest_end =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Kaboto Yokozuma
                            match old_quest_manager.map(|qm| qm.defeat_kaboto_boss) {
                                Some(false)
                                    if quest_manager.defeat_kaboto_boss
                                        && settings.defeat_kaboto_boss =>
                                {
                                    timer::split();
                                }
                                _ => {}
                            }

                            old_quest_manager = Some(quest_manager);
                        }
                        Err(err) => asr::print_message(&format!("quest manager ERROR: {:?}\n{:?}", err, old_game_manager)),
                        _ => {}
                    }

                    match ability_manager
                        .read(&process, old_game_manager.unwrap().ability_pointer.into())
                    {
                        Ok(ability_manager) if old_ability_manager != Some(ability_manager) => {
                            asr::print_message(&format!("{:#?}", ability_manager));
                            // MORE SPLITS
                            // Here we check when each quest is updated, basically we just check when each one is completed
                            //
                            // Attack (Once the staff is given)
                            match old_ability_manager.map(|am| am.can_attack) {
                                Some(false)
                                    if ability_manager.can_attack && settings.can_attack =>
                                {
                                    asr::print_message("Split for attack ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // KiriKiri Bozu (Once the bat ability is given)
                            match old_ability_manager.map(|am| am.can_bat) {
                                Some(false)
                                    if ability_manager.can_bat
                                        && settings.defeated_kirikiri_boss
                                        || settings.can_bat =>
                                {
                                    asr::print_message("Split for bat ability (KiriKiri Bozu)");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once dash is given
                            match old_ability_manager.map(|am| am.can_dash) {
                                Some(false) if ability_manager.can_dash && settings.can_dash => {
                                    asr::print_message("Split for dash ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once idash is given (shade cloak)
                            match old_ability_manager.map(|am| am.can_idash) {
                                Some(false) if ability_manager.can_idash && settings.can_idash => {
                                    asr::print_message("Split for idash ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once grapple is given
                            match old_ability_manager.map(|am| am.can_grapple) {
                                Some(false)
                                    if ability_manager.can_grapple && settings.can_grapple =>
                                {
                                    asr::print_message("Split for grapple ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once hammer dash is given
                            match old_ability_manager.map(|am| am.can_hammer_dash) {
                                Some(false)
                                    if ability_manager.can_hammer_dash
                                        && settings.can_hammer_dash =>
                                {
                                    asr::print_message("Split for hammer dash ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once hover is given
                            match old_ability_manager.map(|am| am.can_hover) {
                                Some(false) if ability_manager.can_hover && settings.can_hover => {
                                    asr::print_message("Split for hover ability");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Once wall jump is given
                            match old_ability_manager.map(|am| am.can_wall_jump) {
                                Some(false)
                                    if ability_manager.can_wall_jump && settings.can_wall_jump =>
                                {
                                    asr::print_message("Split for wall jump ability");
                                    timer::split();
                                }
                                _ => {}
                            }

                            old_ability_manager = Some(ability_manager);
                        }
                        Err(err) => asr::print_message(&format!("quest manager ERROR: {:?} \n{:?}", err, old_game_manager)),
                        _ => {}
                    }
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
