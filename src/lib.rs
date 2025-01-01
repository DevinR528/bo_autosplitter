#![feature(type_alias_impl_trait, const_async_blocks, wasi_ext)]

use std::{collections::HashMap, fs::File, io::BufReader};

use asr::{
    future::next_tick,
    game_engine::unity::{get_scene_name, mono::Module, SceneManager},
    settings::{Gui, Map},
    time::Duration,
    timer::{self, TimerState},
    Address64, Process,
};
use xml::{reader::XmlEvent as ReaderEvent, EventReader};

mod array;
mod game_data;
mod settings;

use array::CSharpArray;
use game_data::{
    AbilityManager, BetaPlayerDataManager, BossData, BossDataBinding, BossKind, Daruma,
    DarumaBinding, DarumaManager, DarumaType, EnemiesManager, GameManager, InventoryContainer,
    QuestManager,
};
use settings::{NumberOfKodamas, Settings};

asr::async_main!(stable);

fn print_message(msg: &str) {
    // #[cfg(debug_assertions)]
    asr::print_message(msg);
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    print_message("Bo AutoSplitter ON!!");
    // let mut old_setting_file = None;
    // let mut completed_splits = HashMap::new();
    // update_settings(&mut settings, &mut old_setting_file, &mut completed_splits);

    loop {
        let process = Process::wait_attach("Bo.exe").await;
        process
            .until_closes(async {
                'start_over: loop {
                    let mut old_setting_file = None;
                    let mut completed_splits = HashMap::new();
                    update_settings(&mut settings, &mut old_setting_file, &mut completed_splits);
                    let module = Module::wait_attach_auto_detect(&process).await;
                    print_message("Found mono");
                    let img = module.wait_get_default_image(&process).await;
                    print_message("Found Assembly-CSharp");

                    let mut paused = false;
                    'reset_all_class_pointers: loop {
                        let scene_manager = SceneManager::wait_attach(&process).await;
                        print_message("Attached SceneManager");

                        let mut old_scene_name = None;
                        if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process) {
                            let name =
                                String::from_utf8_lossy(get_scene_name(path.as_bytes())).to_string();
                            print_message(&name);
                            old_scene_name = Some(name);
                        }

                        let game_manager_class = GameManager::bind(&process, &module, &img).await;
                        print_message("got GameManager");
                        let game_manager_inst = game_manager_class
                            .class()
                            .wait_get_static_instance(&process, &module, "instance")
                            .await;
                        print_message(&format!("got GameManager instance {:?}", game_manager_inst));
                        let mut old_game_manager = None;
                        if let Ok(game_manager) = game_manager_class.read(&process, game_manager_inst) {
                            print_message(&format!("{:#?}", game_manager));

                            let qa_offset = game_manager_class.class().wait_get_field_offset(
                                &process,
                                &module,
                                "<IsQABuild>k__BackingField",
                            ).await;
                            print_message(&format!("IsQABuild field addr: {:#?}", game_manager_inst + qa_offset));

                            old_game_manager = Some(game_manager);
                        }

                        let quest_manager = QuestManager::bind(&process, &module, &img).await;
                        print_message("got QuestManager");
                        let mut old_quest_manager = None;
                        if let Ok(quest_manager) =
                            quest_manager.read(&process, old_game_manager.unwrap().quest_pointer.into())
                        {
                            print_message(&format!("{:#?}", quest_manager));
                            old_quest_manager = Some(quest_manager);
                        }

                        let player_manager = BetaPlayerDataManager::bind(&process, &module, &img).await;
                        print_message("got BetaPlayerDataManager");
                        let mut old_player_manager = None;
                        if let Ok(player_manager) = player_manager.read(
                            &process,
                            old_game_manager.unwrap().player_data_pointer.into(),
                        ) {
                            print_message(&format!("{:#?}", player_manager));
                            old_player_manager = Some(player_manager);
                        }

                        let ability_manager = AbilityManager::bind(&process, &module, &img).await;
                        print_message("got AbilityManager");
                        let mut old_ability_manager = None;
                        if let Ok(ability_manager) = ability_manager
                            .read(&process, old_game_manager.unwrap().ability_pointer.into())
                        {
                            print_message(&format!("{:#?}", ability_manager));
                            old_ability_manager = Some(ability_manager);
                        }

                        let inventory_container =
                            InventoryContainer::bind(&process, &module, &img).await;
                        print_message("got InventoryContainer");
                        let mut old_inventory_container = None;
                        if let Ok(inventory_container) = inventory_container
                            .read(&process, old_game_manager.unwrap().inventory_pointer.into())
                        {
                            print_message(&format!("{:#?}", inventory_container));
                            old_inventory_container = Some(inventory_container);
                        }

                        let boss_class = BossData::bind(&process, &module, &img).await;
                        let enemies_manager = EnemiesManager::bind(&process, &module, &img).await;
                        print_message("got EnemiesManager");
                        let mut old_enemies_manager = None;
                        if let Ok(enemies_manager) = enemies_manager
                            .read(&process, old_game_manager.unwrap().enemies_pointer.into())
                        {
                            print_message(&format!("{:#?}", enemies_manager));
                            old_enemies_manager = Some(enemies_manager);
                        }

                        let mut old_boss_list =
                            get_boss_data_array(&process, old_enemies_manager.as_ref(), &boss_class)
                                .await
                                .ok();
                        if let Some(bl) = &mut old_boss_list {
                            bl.sort_by_key(|b| b.boss_kind);
                            print_message(&format!("got Boss list: {:#?}", bl));
                        }

                        let daruma_class = Daruma::bind(&process, &module, &img).await;
                        let daruma_manager = DarumaManager::bind(&process, &module, &img).await;
                        print_message("got DarumaManager");
                        let mut old_daruma_manager = None;
                        if let Ok(daruma_manager) = daruma_manager
                            .read(&process, old_game_manager.unwrap().daruma_pointer.into())
                        {
                            print_message(&format!("{:#?}", daruma_manager));
                            old_daruma_manager = Some(daruma_manager);
                        }
                        let mut old_daruma_list =
                            get_daruma_data_array(&process, old_daruma_manager.as_ref(), &daruma_class)
                                .await
                                .ok();
                        if let Some(dl) = &mut old_daruma_list {
                            dl.sort_by_key(|d| d.daruma_type);
                            print_message(&format!("got Daruma list: {:#?}", dl));
                        }

                        #[allow(unused_labels)]
                        'normal_game_loop: loop {
                            // Check for a complete run to reset the completed splits map
                            match timer::state() {
                                | TimerState::Running
                                | TimerState::Paused => {},
                                | TimerState::Ended
                                | TimerState::NotRunning => {
                                    // reset completed splits and any other settings
                                    completed_splits.clear();
                                    asr::print_message("clearing completed splits");
                                    paused = false;
                                },
                                TimerState::Unknown => {
                                    asr::print_message("TimerState::Unknown...");
                                }, 
                                // non_exhaustive
                                _ => {}
                            }
                            // This checks for on the fly updates to the settings (you could add a split mid run)
                            update_settings(&mut settings, &mut old_setting_file, &mut completed_splits);

                            // UPDATE first since this knows about everything
                            match game_manager_class.read(&process, game_manager_inst) {
                                Ok(game_manager) if old_game_manager != Some(game_manager) => {
                                    print_message(&format!("{:#?}", game_manager));

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
                                    // This forces update to the InventoryContainer object, if it moves we must read from the new address
                                    match old_game_manager.map(|gm| gm.inventory_pointer) {
                                        Some(old_inventory_pointer)
                                            if old_inventory_pointer
                                                != game_manager.inventory_pointer =>
                                        {
                                            old_inventory_container = None;
                                        }
                                        _ => {}
                                    }

                                    macro_rules! check_game_manager {
                                        ($field:ident, $msg:expr) => {
                                            match old_game_manager.map(|am| am.$field) {
                                                Some(false)
                                                    if game_manager.$field
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true) =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    *completed_splits
                                                        .entry(stringify!($field).to_string())
                                                        .or_insert(true) = true;

                                                    timer::split();
                                                }
                                                _ => {}
                                            }
                                        };
                                    }

                                    // SPLITS
                                    // First elevator up (start of pre Asahi fight)
                                    check_game_manager!(elevator_e_up, "first elevator up");
                                    // First floor elevator up
                                    check_game_manager!(elevator_1_up, "first floor elevator up");
                                    // Second floor elevator up
                                    check_game_manager!(elevator_2_up, "second floor elevator up");
                                    // Third floor elevator up
                                    check_game_manager!(elevator_3_up, "third floor elevator up");

                                    old_game_manager = Some(game_manager);
                                }
                                Err(err) => {
                                    print_message(&format!("game manager ERROR: {:?}", err));
                                    continue 'start_over;
                                }
                                _ => {}
                            }

                            match player_manager.read(
                                &process,
                                old_game_manager.unwrap().player_data_pointer.into(),
                            ) {
                                Ok(player_manager) if Some(player_manager) != old_player_manager => {
                                    match (player_manager, old_player_manager) {
                                        // Timer is paused because of loading screen
                                        (
                                            BetaPlayerDataManager { time_played },
                                            Some(BetaPlayerDataManager {
                                                time_played: old_time,
                                            }),
                                        ) if time_played == old_time => {
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
                                            timer::resume_game_time();
                                            paused = false;
                                        }
                                        // This should be every tick we are in game and playing
                                        (player, _) => {
                                            timer::set_game_time(Duration::seconds_f32(
                                                player.time_played,
                                            ));
                                        }
                                    }

                                    old_player_manager = Some(player_manager);
                                }
                                Ok(player_manager) if Some(player_manager) == old_player_manager => {
                                    timer::pause_game_time();
                                    paused = true;
                                }
                                Err(err) => print_message(&format!("Player_manager ERROR: {:?}", err)),
                                _ => {}
                            }

                            if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process) {
                                let name = String::from_utf8_lossy(get_scene_name(path.as_bytes()))
                                    .to_string();

                                // Update the scene we track
                                if old_scene_name.as_deref() != Some(&name) {
                                    print_message(&format!("new secene {}", name));
                                    // Start timer for the first time
                                    if old_scene_name.as_deref() == Some("New Main Menu")
                                        && name == "CBF Intro"
                                        && old_player_manager
                                            < Some(BetaPlayerDataManager { time_played: 1.0 })
                                    {
                                        print_message(&format!(
                                            "timer started!!! {:?} {:?}",
                                            old_player_manager, old_scene_name
                                        ));
                                        timer::start();
                                    }

                                    if name == "New Main Menu" {
                                        print_message("start over");
                                        paused = true;
                                        timer::pause_game_time();
                                        continue 'reset_all_class_pointers;
                                    }

                                    old_scene_name = Some(name);
                                }
                            }

                            match quest_manager
                                .read(&process, old_game_manager.unwrap().quest_pointer.into())
                            {
                                Ok(quest_manager) if old_quest_manager != Some(quest_manager) => {
                                    print_message(&format!("update {:#?}", quest_manager));

                                    macro_rules! check_quest {
                                        ($field:ident, $msg:expr) => {
                                            match old_quest_manager.map(|am| am.$field) {
                                                Some(false)
                                                    if quest_manager.$field
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true) =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    *completed_splits
                                                        .entry(stringify!($field).to_string())
                                                        .or_insert(true) = true;
                                                    timer::split();
                                                }
                                                _ => {}
                                            }
                                        };
                                    }

                                    // SPLITS
                                    // Here we check when each quest is updated, basically we just check when each one is completed
                                    //
                                    // Asahi staff question start
                                    check_quest!(asahi_staff_start, "Asahi staff start");
                                    // Asahi staff question complete
                                    check_quest!(asahi_staff_end, "Asahi staff end");

                                    // Asahi Eye of Beast start
                                    check_quest!(asahi_eye_of_beast_start, "Asahi Eye of Beast start");
                                    // Asahi Eye of Beast end
                                    check_quest!(asahi_eye_of_beast_end, "Asahi Eye of Beast end");

                                    // Shimeji Armapillos Collection quest start
                                    check_quest!(shimeji_quest_start, "Shimeji Armapillo quest start");
                                    // Shimeji Armapillos Collection quest end
                                    //
                                    // TODO: add setting for number of armadillos collected 0-4
                                    check_quest!(shimeji_quest_end, "Shimeji Armapillo quest end");

                                    // Rozu's Requiem Quest start
                                    check_quest!(rozus_requiem_start, "Rozu's Requiem Quest start");
                                    // Rozu's Requiem Quest end
                                    check_quest!(rozus_requiem_end, "Rozu's Requiem Quest end");

                                    // Vermilion Stranger quest start
                                    check_quest!(vermilion_stranger_quest_start, "Vermilion Stranger quest start");
                                    // Vermilion Stranger quest end
                                    check_quest!(vermilion_stranger_quest_end, "Vermilion Stranger quest end");

                                    // Fox wedding quest started
                                    check_quest!(fox_wedding_start, "fox wedding start");
                                    // Fox wedding save groom
                                    check_quest!(fox_wedding_save_groom, "fox wedding save groom");
                                    // Fox wedding quest complete
                                    check_quest!(fox_wedding_end, "fox wedding end");

                                    // The Kitsune Kifuda quest start (DaiTangu and Gashadoku)
                                    check_quest!(kitsune_kifuda_start, "Kitsune Kifuda quest start");
                                    // The Kitsune Kifuda quest end (DaiTangu and Gashadoku)
                                    check_quest!(kitsune_kifuda_end, "Kitsune Kifuda quest end");

                                    // The Infinite Tea Kettle quest
                                    check_quest!(infinite_kettle_start, "Infinite Tea Kettle quest quest start");
                                    // The Infinite Tea Kettle quest
                                    check_quest!(infinite_kettle_end, "Infinite Tea Kettle quest end");

                                    // First feather key inserted
                                    check_quest!(west_feather_in_keyhole, "first feather key inserted");
                                    // Second feather key inserted
                                    check_quest!(east_feather_in_keyhole, "second feather key inserted");

                                    // Defeated Kaboto (beetle)
                                    check_quest!(defeat_kaboto_boss, "Kaboto defeated");
                                    // Defeated Gashadokuro
                                    check_quest!(defeat_gash_boss, "Gashadokuro defeated");
                                    // Credits roll you did it
                                    check_quest!(credits_roll, "done");

                                    old_quest_manager = Some(quest_manager);
                                }
                                Err(err) => {
                                    asr::print_message(&format!(
                                        "quest manager ERROR: {:?}\n{:?}",
                                        err, old_game_manager
                                    ));
                                    old_quest_manager = None;
                                }
                                _ => {}
                            }

                            match ability_manager
                                .read(&process, old_game_manager.unwrap().ability_pointer.into())
                            {
                                Ok(ability_manager) if old_ability_manager != Some(ability_manager) => {
                                    print_message(&format!("Update {:#?}", ability_manager));

                                    macro_rules! check_ability {
                                        ($field:ident, $msg:expr) => {
                                            match old_ability_manager.map(|am| am.$field) {
                                                Some(false)
                                                    if ability_manager.$field
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true) =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    *completed_splits
                                                        .entry(stringify!($field).to_string())
                                                        .or_insert(true) = true;
                                                    timer::split();
                                                }
                                                _ => {}
                                            }
                                        };
                                    }

                                    // MORE SPLITS
                                    // Here we check when each quest is updated, basically we just check when each one is completed
                                    //
                                    // Attack (Once the staff is given)
                                    check_ability!(can_attack, "attack ability");
                                    // Bat ability
                                    check_ability!(can_bat, "bat ability");
                                    // Once dash is given
                                    check_ability!(can_dash, "dash ability");
                                    // Once idash is given (shade cloak)
                                    check_ability!(can_idash, "I-dash ability");
                                    // Once grapple is given
                                    check_ability!(can_grapple, "grapple ability");
                                    // Once hammer dash is given
                                    check_ability!(can_hammer_dash, "hammer dash ability");
                                    // Once hover is given
                                    check_ability!(can_hover, "hover ability");
                                    // Once wall jump is given
                                    check_ability!(can_wall_jump, "wall jump ability");

                                    old_ability_manager = Some(ability_manager);
                                }
                                Err(err) => asr::print_message(&format!(
                                    "ability manager ERROR: {:?} \n{:?}",
                                    err, old_game_manager
                                )),
                                _ => {}
                            }

                            match inventory_container
                                .read(&process, old_game_manager.unwrap().inventory_pointer.into())
                            {
                                Ok(inventory_container)
                                    if old_inventory_container != Some(inventory_container) =>
                                {
                                    print_message(&format!("update {:#?}", inventory_container));

                                    macro_rules! check_inventory {
                                        (Some($case:tt), $cond_field:ident == $val:expr, $setting:ident, $msg:expr) => {
                                            match old_inventory_container.map(|am| am.$cond_field) {
                                                Some($case)
                                                    if inventory_container.$cond_field == $val
                                                        && settings.$setting
                                                        && !completed_splits
                                                            .get(stringify!($setting))
                                                            .copied()
                                                            .unwrap_or(true) =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    *completed_splits
                                                        .entry(stringify!($setting).to_string())
                                                        .or_insert(true) = true;
                                                    timer::split();
                                                }
                                                _ => {}
                                            }
                                        };
                                        (Some($case:tt), $field:ident, $setting:ident, $msg:expr) => {
                                            match old_inventory_container.map(|am| am.$field) {   
                                                Some($case)
                                                    if inventory_container.$field
                                                        && settings.$setting
                                                        && !completed_splits
                                                            .get(stringify!($setting))
                                                            .copied()
                                                            .unwrap_or(true) =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    *completed_splits
                                                        .entry(stringify!($setting).to_string())
                                                        .or_insert(true) = true;
                                                    timer::split();
                                                }
                                                    _ => {}
                                            }
                                        };
                                    }
                                    //macro_rules! check_lost_inventory {
                                    //    ($setting:ident, $cond_field:ident, $msg:expr) => {
                                    //        if old_inventory_container.$cond_field
                                    //            && !inventory_container.$cond_field
                                    //            && settings.$setting
                                    //            && !completed_splits
                                    //                .get(stringify!($setting))
                                    //                .copied()
                                    //                .unwrap_or(true)
                                    //        {
                                    //            print_message(concat!("Split for losing ", $msg));
                                    //            *completed_splits
                                    //                .entry(stringify!($cond_field).to_string())
                                    //                .or_insert(true) = true;
                                    //            timer::split();
                                    //        }
                                    //    };
                                    //}

                                    check_inventory!(Some(0), feather_keys == 1, first_feather_key, "first feather");
                                    check_inventory!(Some(1), feather_keys == 2, second_feather_key, "second feather");

                                    check_inventory!(Some(0), tablets == 1, one_vs_tablet, "first VS tablet");
                                    check_inventory!(Some(1), tablets == 2, two_vs_tablet, "second VS tablet");
                                    check_inventory!(Some(2), tablets == 3, three_vs_tablet, "third VS tablet");
                                    check_inventory!(Some(3), tablets == 4, four_vs_tablet, "fourth VS tablet");
                                    check_inventory!(Some(4), tablets == 5, five_vs_tablet, "fifth VS tablet");
                                    check_inventory!(Some(_has_scuffed_gunbai), has_scuffed_gunbai, has_scuffed_gunbai, "scuffed gunbai");

                                    match old_inventory_container.map(|am| am.number_of_kodamas) {
                                        Some(old_number)
                                            if old_number < inventory_container.number_of_kodamas
                                                && settings.number_of_kodamas != NumberOfKodamas::NoSplit =>
                                        {
                                            print_message(&format!("KODAMAS: {} {} {:?} {:#?}", old_number, inventory_container.number_of_kodamas, settings.number_of_kodamas, completed_splits));

                                            match settings.number_of_kodamas {
                                                NumberOfKodamas::EveryOne => {
                                                    if !completed_splits
                                                        .get(&format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                        .copied()
                                                        .unwrap_or(false)
                                                    {
                                                        print_message(&format!("Split for number of Kodama's {}", inventory_container.number_of_kodamas));
                                                        *completed_splits
                                                            .entry(format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                },
                                                NumberOfKodamas::EveryFive if inventory_container.number_of_kodamas % 5 == 0 => {
                                                    if !completed_splits
                                                        .get(&format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                        .copied()
                                                        .unwrap_or(false)
                                                    {
                                                        print_message(&format!("Split for number of Kodama's {}", inventory_container.number_of_kodamas));
                                                        *completed_splits
                                                            .entry(format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                },
                                                NumberOfKodamas::EveryTen if (inventory_container.number_of_kodamas % 10 == 0)
                                                    || (inventory_container.number_of_kodamas == 35) =>
                                                {
                                                    if !completed_splits
                                                        .get(&format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                        .copied()
                                                        .unwrap_or(false)
                                                    {
                                                        print_message(&format!("Split for number of Kodama's {}", inventory_container.number_of_kodamas));
                                                        *completed_splits
                                                            .entry(format!("number_of_kodamas__{}", inventory_container.number_of_kodamas))
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }

                                    old_inventory_container = Some(inventory_container);
                                }
                                Err(err) => asr::print_message(&format!(
                                    "inventory manager ERROR: {:?} \n{:?}",
                                    err, old_game_manager
                                )),
                                _ => {}
                            }

                            match enemies_manager
                                .read(&process, old_game_manager.unwrap().enemies_pointer.into())
                            {
                                Ok(enemies_manager) if old_enemies_manager != Some(enemies_manager) => {
                                    print_message(&format!("update {:#?}", enemies_manager));

                                    old_enemies_manager = Some(enemies_manager);
                                }
                                Err(err) => asr::print_message(&format!(
                                    "enemies manager ERROR: {:?} \n{:?}",
                                    err, old_game_manager
                                )),
                                _ => {}
                            }

                            // SPLITS for bosses
                            let mut new_boss_list = get_boss_data_array(
                                &process,
                                old_enemies_manager.as_ref(),
                                &boss_class,
                            )
                            .await
                            .ok();
                            if let Some(list) = &mut new_boss_list {
                                list.sort_by_key(|b| b.boss_kind);
                            }
                            if new_boss_list != old_boss_list {
                                print_message(&format!("update Boss {:#?}", new_boss_list));
                                if let (Some(new_list), Some(old_list)) =
                                    (&new_boss_list, old_boss_list)
                                {
                                    for (new_boss, old_boss) in new_list.iter().zip(old_list.iter()) {
                                        if new_boss != old_boss
                                            && new_boss.boss_kind == old_boss.boss_kind
                                        {
                                            macro_rules! check_boss {
                                                ($field:ident, $msg:expr) => {
                                                    if !old_boss.defeated
                                                        && new_boss.defeated
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true)
                                                    {
                                                        print_message(concat!("Split for ", $msg));
                                                        *completed_splits
                                                            .entry(stringify!($field).to_string())
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                };
                                                ($field:ident, $extra:ident == $val:expr, $msg:expr) => {
                                                    if !old_boss.defeated
                                                        && new_boss.defeated
                                                        && new_boss.$extra == $val
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true)
                                                    {
                                                        print_message(concat!("Split for ", $msg));
                                                        *completed_splits
                                                            .entry(stringify!($field).to_string())
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                };
                                            }

                                            match new_boss.boss_kind {
                                                BossKind::Placeholder => {}
                                                BossKind::KiriKiriBozu => check_boss!(defeated_kirikiri_boss, "KiriKiri Bozu defeated"),
                                                BossKind::PUA => check_boss!(defeated_pua_boss, "PUA defeated split"),
                                                BossKind::Hashihime => check_boss!(defeat_hashihime_boss, "Hashihime defeated"),
                                                BossKind::Yuki => check_boss!(defeat_yuki_boss, "Kitsura defeated"),
                                                BossKind::Yokozuna => {}, // check_boss!(defeat_kaboto_boss, "Yokozuna defeated split"),
                                                BossKind::Jorogumo => check_boss!(defeat_jorogumo_boss, "Jorojumo defeated"),
                                                BossKind::KarasuTengu => {
                                                    check_boss!(defeat_karasu_tengu_one_boss, total_health == 133.0, "KarasuTengu single defeated");
                                                    check_boss!(defeat_karasu_tengu_two_boss, total_health == 225.0, "KarasuTengu duo defeated");
                                                }
                                                BossKind::DaiTengu => check_boss!(defeat_dai_tengu_boss, "DaiTengu (Tengu Trio) defeated"),
                                                BossKind::Gasha => {}, // check_boss!(defeat_gash_boss, "Gashadokuro defeated"),
                                                BossKind::Asahi => check_boss!(defeat_ashai_boss, "Asahi defeated"),
                                                BossKind::Shogun => check_boss!(defeat_sakura_boss, "Sakura Shogun defeated"),
                                                // TODO: who dis...
                                                BossKind::Amaterasu => {
                                                    print_message(&format!("Amaterasu boss matched: {:#?}\n{:#?}", new_boss, old_boss));
                                                }
                                            }
                                        }
                                    }
                                }

                                old_boss_list = new_boss_list;
                            }

                            // SPLITS for Daruma collecting
                            let mut new_daruma_list = get_daruma_data_array(
                                &process,
                                old_daruma_manager.as_ref(),
                                &daruma_class,
                            )
                            .await
                            .ok();
                            if let Some(list) = &mut new_daruma_list {
                                list.sort_by_key(|d| d.daruma_type);
                            }
                            if new_daruma_list != old_daruma_list {
                                print_message(&format!("update Daruma {:#?}", new_daruma_list));
                                if let (Some(new_list), Some(old_list)) =
                                    (&new_daruma_list, old_daruma_list)
                                {
                                    for (new_daruma, old_daruma) in new_list.iter().zip(old_list.iter())
                                    {
                                        if new_daruma != old_daruma
                                            && new_daruma.daruma_type == old_daruma.daruma_type
                                        {
                                            macro_rules! check_daruma {
                                                ($field:ident, $msg:expr) => {
                                                    if !old_daruma.available
                                                        && new_daruma.available
                                                        && settings.$field
                                                        && !completed_splits
                                                            .get(stringify!($field))
                                                            .copied()
                                                            .unwrap_or(true)
                                                    {
                                                        print_message(concat!("Split for ", $msg));
                                                        *completed_splits
                                                            .entry(stringify!($field).to_string())
                                                            .or_insert(true) = true;
                                                        timer::split();
                                                    }
                                                };
                                            }

                                            match new_daruma.daruma_type {
                                                DarumaType::Bite => check_daruma!(got_chomper_daruma, "Chomper Daruma"),
                                                DarumaType::Parry => check_daruma!(got_mamori_daruma, "Mamori Daruma"),
                                                DarumaType::Thorns => check_daruma!(got_togichan_daruma, "TogiChan Daruma"),
                                                DarumaType::Spirits => check_daruma!(got_jingu_daruma, "Jingu Daruma"),
                                                DarumaType::Bomb => check_daruma!(got_kaboomaru_daruma, "Kaboomaru Daruma"),
                                                DarumaType::SpinAttack => {}
                                                DarumaType::Deprecated1 => {}
                                                DarumaType::FireWall => check_daruma!(got_pyro_daruma, "PyroKun Daruma"),
                                                DarumaType::Ice => check_daruma!(got_yuki_daruma, "Yuki Daruma"),
                                                DarumaType::Boomerang => check_daruma!(got_ken_daruma, "Ken Daruma"),
                                            }
                                        }
                                    }
                                }

                                old_daruma_list = new_daruma_list;
                            }

                            next_tick().await;
                        }
                    }
                }
            })
            .await;
    }
}

async fn get_boss_data_array(
    process: &Process,
    old_em: Option<&EnemiesManager>,
    boss_class: &BossDataBinding,
) -> Result<Vec<BossData>, ()> {
    let mut boss_datas = vec![];
    if let Some(em) = old_em {
        let arr = CSharpArray::<Address64>::new(em.bosses);
        boss_datas = arr.read_class(process, |p, a| boss_class.read(p, a))?;
    }
    Ok(boss_datas)
}

async fn get_daruma_data_array(
    process: &Process,
    old_dm: Option<&DarumaManager>,
    daruma_class: &DarumaBinding,
) -> Result<Vec<Daruma>, ()> {
    let mut all_darumas = vec![];
    if let Some(dm) = old_dm {
        let arr = CSharpArray::<Address64>::new(dm.all_darumas);
        all_darumas = arr.read_class(process, |p, a| daruma_class.read(p, a))?;
    }
    Ok(all_darumas)
}

fn update_settings(
    settings: &mut Settings,
    old_lss_file: &mut Option<String>,
    completed: &mut HashMap<String, bool>,
) {
    settings.update();

    if (!settings.lss_file.path.is_empty() && old_lss_file.is_none())
        || (old_lss_file.is_some() && old_lss_file.as_ref() != Some(&settings.lss_file.path))
    {
        if read_settings_xml(settings).is_err() {
            asr::print_message(&format!(
                "Error: reading xml settings file '{}'",
                settings.lss_file.path
            ));
        } else {
            *old_lss_file = Some(settings.lss_file.path.clone());

            print_message(&format!("Updated map (read file) {:#?}", Map::load()));
        }
    }

    let map = Map::load();
    let finished = completed
        .keys()
        .filter(|name| !name.starts_with("number_of_kodamas__"))
        .count();
    if (finished as u64) != map.len() {
        for k in map.keys() {
            completed.entry(k.to_string()).or_insert(false);
        }

        print_message(&format!("Updated map {:#?}\n{:#?}", map, completed));
    }
}

fn read_settings_xml(settings: &Settings) -> Result<(), ()> {
    match File::open(&settings.lss_file.path) {
        Ok(f) => {
            let map = Map::load();

            let file = BufReader::new(f); // Buffering is important for performance
            let parser = EventReader::new(file);

            let mut in_autosplitter = false;
            let mut in_splits = false;
            let mut current_name = None;
            for ev_result in parser {
                match ev_result {
                    Ok(ev) => {
                        match_xml(
                            ev,
                            &map,
                            &mut current_name,
                            &mut in_splits,
                            &mut in_autosplitter,
                        );
                    }
                    Err(err) => asr::print_message(&format!("Error in read: {}", err)),
                }
            }
            map.store();
        }
        Err(e) => asr::print_message(&format!("Error in read: {}", e)),
    }

    Ok(())
}

fn match_xml(
    e: ReaderEvent,
    map: &Map,
    current_name: &mut Option<String>,
    in_splits: &mut bool,
    in_autosplitter: &mut bool,
) {
    match &e {
        ReaderEvent::StartElement { name, .. } if name.local_name == "CustomSettings" => {
            *in_autosplitter = true;
        }
        ReaderEvent::StartElement {
            name, attributes, ..
        } if name.local_name == "Setting" => {
            print_message(&format!("{:?}", attributes));

            // This is the normal key is the Setting tag's id
            if let Some(attr) = attributes.first().filter(|attr| {
                attr.name.local_name == "id" && !attr.value.is_empty() && attr.value != "lss_file"
            }) {
                *current_name = Some(attr.value.clone());
                *in_splits = true;
            }

            // Here we have a Setting tag where the value is in one of the attributes
            //
            // This is for the lss_file setting
            if let Some(attr) = attributes
                .first()
                // The first attribute has the key (we know it to be lss_file)
                .filter(|attr| attr.name.local_name == "id" && attr.value == "lss_file")
                .and_then(|_| attributes.last())
                // Last attribute has the name of the file
                .filter(|attr| attr.name.local_name == "value" && !attr.value.is_empty())
            {
                map.insert("lss_file", attr.value.as_str());
                // We know this has no Characters() event next so skip it
                *in_splits = false;
                *current_name = None;
            }

            // This is for the `number_of_kodamas` setting
            if let Some(attr) = attributes
                .first()
                // The first attribute has the key (we know it to be lss_file)
                .filter(|attr| attr.name.local_name == "id" && attr.value == "number_of_kodamas")
                .and_then(|_| attributes.last())
                // Last attribute has the name of the file
                .filter(|attr| attr.name.local_name == "value" && !attr.value.is_empty())
            {
                map.insert("number_of_kodamas", attr.value.as_str());
                // We know this has no Characters() event next so skip it
                *in_splits = false;
                *current_name = None;
            }
        }
        ReaderEvent::Characters(val)
            if *in_autosplitter && *in_splits && current_name.is_some() =>
        {
            map.insert(current_name.as_ref().unwrap(), val == "True");
            *current_name = None;
        }
        ReaderEvent::EndElement { name } if name.local_name == "CustomSettings" => {
            *in_autosplitter = false;
        }
        ReaderEvent::EndElement { name } if name.local_name == "Setting" => {
            *in_splits = false;
        }
        _ => {}
    }
}
