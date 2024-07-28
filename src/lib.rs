#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{
    future::next_tick,
    game_engine::unity::{get_scene_name, mono::Module, SceneManager},
    settings::{Gui, Map},
    time::Duration,
    timer, Address64, Process,
};

mod array;
mod game_data;
mod settings;

use array::CSharpArray;
use game_data::{
    AbilityManager, BetaPlayerDataManager, BossData, BossDataBinding, BossKind, Daruma,
    DarumaBinding, DarumaManager, DarumaType, EnemiesManager, GameManager, InventoryContainer,
    QuestManager,
};
use settings::{Category, Settings, ANY_PERCENT};

asr::async_main!(stable);

fn print_message(msg: &str) {
    #[cfg(debug_assertions)]
    asr::print_message(msg);
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    print_message("Bo AutoSplitter ON!!");
    let mut old_category = None;

    loop {
        // TODO: should probably move `settings.update()` into the function call `update_settings()`
        update_settings(&settings, &mut old_category);
        settings.update();

        let process = Process::wait_attach("Bo.exe").await;

        process
            .until_closes(async {
                'start_over: loop {
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
                            let name = String::from_utf8_lossy(get_scene_name(path.as_bytes()))
                                .to_string();
                            print_message(&name);
                            old_scene_name = Some(name);
                        }

                        let game_manager = GameManager::bind(&process, &module, &img).await;
                        print_message("got GameManager");
                        let game_manager_inst = game_manager
                            .class()
                            .wait_get_static_instance(&process, &module, "instance")
                            .await;
                        print_message(&format!(
                            "got GameManager instance {:?}",
                            game_manager_inst
                        ));
                        let mut old_game_manager = None;
                        if let Ok(game_manager) = game_manager.read(&process, game_manager_inst) {
                            print_message(&format!("{:#?}", game_manager));
                            old_game_manager = Some(game_manager);
                        }

                        let quest_manager = QuestManager::bind(&process, &module, &img).await;
                        print_message("got QuestManager");
                        let mut old_quest_manager = None;
                        if let Ok(quest_manager) = quest_manager
                            .read(&process, old_game_manager.unwrap().quest_pointer.into())
                        {
                            print_message(&format!("{:#?}", quest_manager));
                            old_quest_manager = Some(quest_manager);
                        }

                        let player_manager =
                            BetaPlayerDataManager::bind(&process, &module, &img).await;
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

                        let mut old_boss_list = get_boss_data_array(
                            &process,
                            old_enemies_manager.as_ref(),
                            &boss_class,
                        )
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
                        let mut old_daruma_list = get_daruma_data_array(
                            &process,
                            old_daruma_manager.as_ref(),
                            &daruma_class,
                        )
                        .await
                        .ok();
                        if let Some(dl) = &mut old_daruma_list {
                            dl.sort_by_key(|d| d.daruma_type);
                            print_message(&format!("got Daruma list: {:#?}", dl));
                        }

                        #[allow(unused_labels)]
                        'normal_game_loop: loop {
                            update_settings(&settings, &mut old_category);
                            settings.update();

                            // UPDATE first since this knows about everything
                            match game_manager.read(&process, game_manager_inst) {
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

                                    // SPLITS
                                    // First elevator up (start of pre Asahi fight)
                                    match old_game_manager.map(|gm| gm.elevator_e_up) {
                                        Some(false)
                                            if game_manager.elevator_e_up
                                                && settings.first_elevator_up =>
                                        {
                                            print_message("Split for first elevator up");
                                            timer::split();
                                        }
                                        _ => {}
                                    }

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
                                Ok(player_manager)
                                    if Some(player_manager) != old_player_manager =>
                                {
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
                                Err(err) => {
                                    print_message(&format!("Player_manager ERROR: {:?}", err))
                                }
                                _ => {}
                            }

                            if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process)
                            {
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
                                                    if quest_manager.$field && settings.$field =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
                                                    timer::split();
                                                }
                                                _ => {}
                                            }
                                        };
                                        ($field:ident, $alt_match:expr, $msg:expr) => {
                                            match old_quest_manager.map(|am| am.$field) {
                                                $alt_match
                                                    if quest_manager.$field && settings.$field =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
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
                                    check_quest!(
                                        asahi_eye_of_beast_start,
                                        "Asahi Eye of Beast start"
                                    );
                                    // Asahi Eye of Beast end
                                    check_quest!(asahi_eye_of_beast_end, "Asahi Eye of Beast end");

                                    // Shimeji Armapillos Collection quest start
                                    check_quest!(
                                        shimeji_quest_start,
                                        "Shimeji Armapillo quest start"
                                    );
                                    // Shimeji Armapillos Collection quest end
                                    //
                                    // TODO: add setting for number of armadillos collected 0-4
                                    check_quest!(shimeji_quest_end, "Shimeji Armapillo quest end");

                                    // Rozu's Requiem Quest start
                                    check_quest!(
                                        rozus_requiem_start,
                                        "Rozu's Requiem Quest start"
                                    );
                                    // Rozu's Requiem Quest end
                                    check_quest!(
                                        rozus_requiem_end,
                                        "Rozu's Requiem Quest end"
                                    );

                                    // Vermilion Stranger quest start
                                    check_quest!(
                                        vermilion_stranger_quest_start,
                                        "Vermilion Stranger quest start"
                                    );
                                    // Vermilion Stranger quest end
                                    check_quest!(
                                        vermilion_stranger_quest_end,
                                        "Vermilion Stranger quest end"
                                    );

                                    // Fox wedding quest started
                                    check_quest!(fox_wedding_start, "fox wedding start");
                                    // Fox wedding quest complete
                                    check_quest!(fox_wedding_end, "fox wedding end");

                                    // First feather key inserted
                                    check_quest!(
                                        west_feather_in_keyhole,
                                        "first feather key inserted"
                                    );
                                    // Second feather key inserted
                                    check_quest!(
                                        east_feather_in_keyhole,
                                        "second feather key inserted"
                                    );

                                    // Credits roll you did it
                                    check_quest!(credits_roll, "done");

                                    old_quest_manager = Some(quest_manager);
                                }
                                Err(err) => {
                                    print_message(&format!(
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
                                Ok(ability_manager)
                                    if old_ability_manager != Some(ability_manager) =>
                                {
                                    print_message(&format!("Update {:#?}", ability_manager));

                                    macro_rules! check_ability {
                                        ($field:ident, $msg:expr) => {
                                            match old_ability_manager.map(|am| am.$field) {
                                                Some(false)
                                                    if ability_manager.$field
                                                        && settings.$field =>
                                                {
                                                    print_message(concat!("Split for ", $msg));
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
                                Err(err) => print_message(&format!(
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
                                    print_message(&format!(
                                        "update {:#?}",
                                        inventory_container
                                    ));

                                    // First feather key
                                    match old_inventory_container.map(|ic| ic.feather_keys) {
                                        Some(0)
                                            if inventory_container.feather_keys == 1
                                                && settings.first_feather_key =>
                                        {
                                            print_message("Split for first feather");
                                            timer::split();
                                        }
                                        _ => {}
                                    }
                                    // Second feather key
                                    match old_inventory_container.map(|ic| ic.feather_keys) {
                                        Some(1)
                                            if inventory_container.feather_keys == 2
                                                && settings.second_feather_key =>
                                        {
                                            print_message("Split for second feather");
                                            timer::split();
                                        }
                                        _ => {}
                                    }

                                    old_inventory_container = Some(inventory_container);
                                }
                                Err(err) => print_message(&format!(
                                    "inventory manager ERROR: {:?} \n{:?}",
                                    err, old_game_manager
                                )),
                                _ => {}
                            }

                            match enemies_manager
                                .read(&process, old_game_manager.unwrap().enemies_pointer.into())
                            {
                                Ok(enemies_manager)
                                    if old_enemies_manager != Some(enemies_manager) =>
                                {
                                    print_message(&format!("update {:#?}", enemies_manager));

                                    old_enemies_manager = Some(enemies_manager);
                                }
                                Err(err) => print_message(&format!(
                                    "enemies manager ERROR: {:?} \n{:?}",
                                    err, old_game_manager
                                )),
                                _ => {}
                            }

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
                                    for (new_boss, old_boss) in new_list.iter().zip(old_list.iter())
                                    {
                                        if new_boss != old_boss
                                            && new_boss.boss_kind == old_boss.boss_kind
                                        {
                                            match new_boss.boss_kind {
                                                BossKind::Placeholder => {}
                                                BossKind::KiriKiriBozu => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeated_kirikiri_boss,
                                                    "KiriKiri Bozu defeated split",
                                                ),
                                                BossKind::PUA => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeated_pua_boss,
                                                    "PUA defeated split",
                                                ),
                                                BossKind::Hashihime => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_hashihime_boss,
                                                    "Hashihime defeated split",
                                                ),
                                                BossKind::Yuki => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    // TODO: which one is Yuki
                                                    false,
                                                    "Yuki defeated split",
                                                ),
                                                BossKind::Yokozuna => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_kaboto_boss,
                                                    "Yokozuna defeated split",
                                                ),
                                                BossKind::Jorogumo => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_jorogumo_boss,
                                                    "Jorojumo defeated split",
                                                ),
                                                // TODO: confirm who this is, I think it is the first tengu bird in ice palace
                                                BossKind::KarasuTengu => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_karasu_tengu_boss,
                                                    "KarasuTengu defeated split",
                                                ),
                                                // TODO: confirm who this is, I think it is the second tengu bird in ice palace
                                                BossKind::DaiTengu => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_dai_tengu_boss,
                                                    "DaiTengu defeated split",
                                                ),
                                                BossKind::Gasha => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_gash_boss,
                                                    "Gashadokuro defeated split",
                                                ),
                                                BossKind::Asahi => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_ashai_boss,
                                                    "Asahi defeated split",
                                                ),
                                                BossKind::Shogun => check_boss_defeated(
                                                    !old_boss.defeated && new_boss.defeated,
                                                    settings.defeat_sakura_boss,
                                                    "Sakura Shogun defeated split",
                                                ),
                                                // TODO: who dis
                                                BossKind::Amaterasu => {}
                                            }
                                        }
                                    }
                                }
                                old_boss_list = new_boss_list;
                            }

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
                                print_message(&format!(
                                    "update Daruma {:#?}",
                                    new_daruma_list
                                ));
                                if let (Some(new_list), Some(old_list)) =
                                    (&new_daruma_list, old_daruma_list)
                                {
                                    for (new_daruma, old_daruma) in
                                        new_list.iter().zip(old_list.iter())
                                    {
                                        if new_daruma != old_daruma
                                            && new_daruma.daruma_type == old_daruma.daruma_type
                                        {
                                            match new_daruma.daruma_type {
                                                DarumaType::Bite => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_chomper_daruma
                                                    {
                                                        print_message(
                                                            "Split for Chomper Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Parry => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_mamori_daruma
                                                    {
                                                        print_message(
                                                            "Split for Mamori Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Thorns => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_togichan_daruma
                                                    {
                                                        print_message(
                                                            "Split for Togi Chan Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Spirits => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_jingu_daruma
                                                    {
                                                        print_message(
                                                            "Split for Jingu Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Bomb => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_kaboomaru_daruma
                                                    {
                                                        print_message(
                                                            "Split for Kaboomaru Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::SpinAttack => {}
                                                DarumaType::Deprecated1 => {}
                                                DarumaType::FireWall => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_pyro_daruma
                                                    {
                                                        print_message(
                                                            "Split for PyroKun Daruma",
                                                        );
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Ice => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_yuki_daruma
                                                    {
                                                        print_message("Split for Yuki Daruma");
                                                        timer::split();
                                                    }
                                                }
                                                DarumaType::Boomerang => {
                                                    if new_daruma.available
                                                        && !old_daruma.available
                                                        && settings.got_ken_daruma
                                                    {
                                                        print_message("Split for Ken Daruma");
                                                        timer::split();
                                                    }
                                                }
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

fn update_settings(settings: &Settings, old_category: &mut Option<Category>) {
    match (settings.category, *old_category != Some(settings.category)) {
        (Category::AnyPercent, true) => {
            let map = Map::load();
            for setting in ANY_PERCENT {
                map.insert(setting, true)
            }
            map.store();
            *old_category = Some(settings.category);
        }
        (Category::HundredPercent, true) => todo!("impl HundredPercent category"),
        _ => {}
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

fn check_boss_defeated(defeated: bool, setting: bool, msg: &str) {
    if defeated && setting {
        print_message(msg);
        timer::split();
    }
}
