#![feature(type_alias_impl_trait, const_async_blocks)]

use std::fmt::{self, Debug};

use asr::{
    future::next_tick,
    game_engine::unity::{
        get_scene_name,
        mono::{Class, Module},
        SceneManager,
    },
    settings::Gui,
    timer, Address64, Process,
};

asr::async_main!(stable);

#[derive(Gui)]
struct Settings {
    /// My Setting
    ///
    /// Tool tip for my setting...
    #[default = true]
    my_setting: bool,
}

#[derive(Debug, Class, Copy, Clone, PartialEq)]
struct BetaPlayerDataManager {
    #[rename = "<TimePlayed>k__BackingField"]
    time_played: f32,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, Eq)]
struct QuestManager {
    #[rename = "<AsahiBambooStaffQuestStarted>k__BackingField"]
    asahi_staff_start: bool,
    #[rename = "<AsahiBambooStaffQuestCompleted>k__BackingField"]
    asahi_staff_end: bool,
    #[rename = "<AsahiEyeOfTheBeastQuestStarted>k__BackingField"]
    asahi_eye_of_beast_start: bool,
    #[rename = "<AsahiEyeOfTheBeastQuestCompleted>k__BackingField"]
    asahi_eye_of_beast_end: bool,
    #[rename = "<AsahiAfterArmapilloBoss>k__BackingField"]
    asahi_post_armapillo_boss: bool,
    #[rename = "<ToriBumpProphecyTold>k__BackingField"]
    tori_bump_told: bool,
    #[rename = "<ToriFulfilledBumpProphecy>k__BackingField"]
    tori_bump_end: bool,
    #[rename = "<ToriBatProphecyTold>k__BackingField"]
    tori_bat_told: bool,
    #[rename = "<ToriFulfilledBatProphecy>k__BackingField"]
    tori_bat_end: bool,
    #[rename = "<ToriDashProphecyTold>k__BackingField"]
    tori_dash_told: bool,
    #[rename = "<ToriFulfilledDashProphecy>k__BackingField"]
    tori_dash_end: bool,
    #[rename = "<ShimejiArmapillosCollected>k__BackingField"]
    shimeji_armapillos_collect: i32,
    #[rename = "<ShimejiQuestStarted>k__BackingField"]
    shimeji_quest_start: bool,
    #[rename = "<ShimejiQuestCompleted>k__BackingField"]
    shimeji_quest_end: bool,
    // TODO: more of these...
}

#[derive(Class, Copy, Clone, PartialEq, Eq)]
struct GameManager {
    #[rename = "<ElevatorEntranceUp>k__BackingField"]
    elevator_e_up: bool,
    #[rename = "<ElevatorFloor1Up>k__BackingField"]
    elevator_1_up: bool,
    #[rename = "<ElevatorFloor1Down>k__BackingField"]
    elevator_1_down: bool,
    #[rename = "<ElevatorFloor2Up>k__BackingField"]
    elevator_2_up: bool,
    #[rename = "<ElevatorFloor2Down>k__BackingField"]
    elevator_2_down: bool,
    #[rename = "<ElevatorFloor3Up>k__BackingField"]
    elevator_3_up: bool,
    #[rename = "<ElevatorFloor3Down>k__BackingField"]
    elevator_3_down: bool,
    #[rename = "<VerticalChaseStarted>k__BackingField"]
    vertical_chase_start: bool,
    #[rename = "<loadGame>k__BackingField"]
    load_game: bool,
    #[rename = "<fromInGame>k__BackingField"]
    from_in_game: bool,
    #[rename = "<QuestManager>k__BackingField"]
    quest_pointer: Address64,
    #[rename = "abilityManager"]
    ability_pointer: Address64,
    #[rename = "betaDataManager"]
    player_data_pointer: Address64,
}

impl Debug for GameManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameManager")
            .field("elevator_e_up", &self.elevator_e_up)
            .field("elevator_1_up", &self.elevator_1_up)
            .field("elevator_1_down", &self.elevator_1_down)
            .field("elevator_2_up", &self.elevator_2_up)
            .field("elevator_2_down", &self.elevator_2_down)
            .field("elevator_3_up", &self.elevator_3_up)
            .field("elevator_3_down", &self.elevator_3_down)
            .field("vertical_chase_start", &self.vertical_chase_start)
            .field("load_game", &self.load_game)
            .field("from_in_game", &self.from_in_game)
            .field("quest_pointer", &self.quest_pointer)
            .field("ability_pointer", &self.ability_pointer)
            .field("player_data_pointer", &self.player_data_pointer)
            .finish()
    }
}

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
                                    old_player_manager = Some(player_manager);
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
                                    if old_quest_ptr != game_manager.quest_pointer => {
                                        old_quest_manager = None;
                                    }
                                _ => {}
                            }
                            // This forces update to the AbilityManager object, if it moves we must read from the new address
                            match old_game_manager.map(|gm| gm.ability_pointer) {
                                Some(old_ability_ptr)
                                    if old_ability_ptr != game_manager.ability_pointer => {
                                        // old_ability_manager = None;
                                    }
                                _ => {}
                            }
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
                            match old_quest_manager.map(|qm| qm.asahi_staff_end) {
                                Some(false) if quest_manager.asahi_staff_end => {
                                    timer::split();
                                }
                                _ => {}
                            }

                            old_quest_manager = Some(quest_manager);
                        }
                        Err(err) => asr::print_message(&format!("quest manager ERROR: {:?}", err)),
                        _ => {}
                    }

                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
