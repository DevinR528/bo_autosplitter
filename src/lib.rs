#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{
    future::next_tick,
    game_engine::unity::{
        get_scene_name,
        mono::{Module},
        SceneManager,
    },
    settings::{Gui, Map},
    timer,  Address64,  Process,
};
use futures::future::FutureExt;

mod array;
mod game_data;
mod settings;

use array::CSharpArray;
use game_data::{
    AbilityManager, BetaPlayerDataManager, BossData, BossDataBinding, BossKind, Daruma, DarumaBinding, DarumaManager, DarumaType, EnemiesManager, GameManager, InventoryContainer, QuestManager
};
use settings::{Category, Settings, ANY_PERCENT};

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
                if let Ok(ability_manager) =
                    ability_manager.read(&process, old_game_manager.unwrap().ability_pointer.into())
                {
                    asr::print_message(&format!("{:#?}", ability_manager));
                    old_ability_manager = Some(ability_manager);
                }

                let inventory_container = InventoryContainer::bind(&process, &module, &img).await;
                asr::print_message("got InventoryContainer");
                let mut old_inventory_container = None;
                if let Ok(inventory_container) = inventory_container
                    .read(&process, old_game_manager.unwrap().inventory_pointer.into())
                {
                    asr::print_message(&format!("{:#?}", inventory_container));
                    old_inventory_container = Some(inventory_container);
                }

                let boss_class = BossData::bind(&process, &module, &img).await;
                let enemies_manager = EnemiesManager::bind(&process, &module, &img).await;
                asr::print_message("got EnemiesManager");
                let mut old_enemies_manager = None;
                if let Ok(enemies_manager) =
                    enemies_manager.read(&process, old_game_manager.unwrap().enemies_pointer.into())
                {
                    asr::print_message(&format!("{:#?}", enemies_manager));
                    old_enemies_manager = Some(enemies_manager);
                }

                let mut old_boss_list =
                    get_boss_data_array(&process, old_enemies_manager.as_ref(), &boss_class)
                        .await
                        .ok();

                let daruma_class = Daruma::bind(&process, &module, &img).await;
                let daruma_manager = DarumaManager::bind(&process, &module, &img).await;
                asr::print_message("got DarumaManager");
                let mut old_daruma_manager = None;
                if let Ok(daruma_manager) =
                    daruma_manager.read(&process, old_game_manager.unwrap().daruma_pointer.into())
                {
                    asr::print_message(&format!("{:#?}", daruma_manager));
                    old_daruma_manager = Some(daruma_manager);
                }
                let mut old_daruma_list = get_daruma_data_array(
                    &process,
                    old_daruma_manager.as_ref(),
                    &daruma_class,
                )
                .await
                .ok();
                if let Some(dl) = &old_daruma_list {
                    asr::print_message(&format!("Daruma list updated: {:#?}", dl));
                }

                let mut paused = false;
                let mut old_category = None;
                loop {
                    match (settings.category, old_category != Some(settings.category)) {
                        (Category::AnyPercent, true) => {
                            let map = Map::load();
                            for setting in ANY_PERCENT {
                                map.insert(setting, true)
                            }
                            map.store();
                            old_category = Some(settings.category);
                        }
                        (Category::HundredPercent, true) => todo!("impl HundredPercent category"),
                        _ => {}
                    }
                    settings.update();

                    match player_manager.read(
                        &process,
                        old_game_manager.unwrap().player_data_pointer.into(),
                    ) {
                        Ok(player_manager) if old_player_manager != Some(player_manager) => {
                            match (player_manager, old_player_manager) {
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

                    if let Ok(path) = scene_manager.get_current_scene_path::<128>(&process) {
                        let name =
                            String::from_utf8_lossy(get_scene_name(path.as_bytes())).to_string();

                        // Update the scene we track
                        if old_scene_name.as_deref() != Some(&name) {
                            asr::print_message(&format!("new secene {}", name));
                            // Start timer for the first time
                            if old_scene_name.as_deref() == Some("New Main Menu")
                                && name == "CBF Intro"
                                && old_player_manager
                                    < Some(BetaPlayerDataManager { time_played: 1.0 })
                            {
                                asr::print_message(&format!(
                                    "timer started!!! {:?} {:?}",
                                    old_player_manager, old_scene_name
                                ));
                                timer::start();
                            }

                            old_scene_name = Some(name);
                        }
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
                            // This forces update to the InventoryContainer object, if it moves we must read from the new address
                            match old_game_manager.map(|gm| gm.inventory_pointer) {
                                Some(old_inventory_pointer)
                                    if old_inventory_pointer != game_manager.inventory_pointer =>
                                {
                                    old_inventory_container = None;
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
                            asr::print_message(&format!("update {:#?}", quest_manager));
                            // SPLITS
                            // Here we check when each quest is updated, basically we just check when each one is completed
                            //
                            // Asahi staff question complete
                            match old_quest_manager.map(|qm| qm.asahi_staff_end) {
                                Some(false)
                                    if quest_manager.asahi_staff_end
                                        && settings.asahi_staff_end =>
                                {
                                    asr::print_message("Split for Asahi staff end");
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
                                    asr::print_message("Split for Asahi Eye of Beast end");
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
                                    asr::print_message("Split for Shimeji armadillos collected");
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
                                    asr::print_message("Split for PUA");
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
                                    asr::print_message("Split for Hashihime");
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
                                    asr::print_message("Split for VS quest");
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
                                    asr::print_message("Split for Kaboto");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Jorogumo (spider boss)
                            match old_quest_manager.map(|qm| qm.defeat_spider_boss) {
                                Some(false)
                                    if quest_manager.defeat_spider_boss
                                        && settings.defeat_jorogumo_boss =>
                                {
                                    asr::print_message("Split for jorogumo");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Gashadokuro (Skeleton)
                            match old_quest_manager.map(|qm| qm.defeat_gash_boss) {
                                Some(false)
                                    if quest_manager.defeat_gash_boss
                                        && settings.defeat_gash_boss =>
                                {
                                    asr::print_message("Split for gashadokuro");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Sakura Shogun (final boss)
                            match old_quest_manager.map(|qm| qm.defeat_sakura_boss) {
                                Some(false)
                                    if quest_manager.defeat_sakura_boss
                                        && settings.defeat_sakura_boss =>
                                {
                                    asr::print_message("Split for Sakura Shogun");
                                    timer::split();
                                }
                                _ => {}
                            }
                            // Fox wedding quest complete
                            match old_quest_manager.map(|qm| qm.fox_wedding_end) {
                                Some(false)
                                    if quest_manager.fox_wedding_end
                                        && settings.fox_wedding_end =>
                                {
                                    asr::print_message("Split for fox wedding end");
                                    timer::split();
                                }
                                _ => {}
                            }

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
                                        && (settings.defeated_kirikiri_boss
                                            || settings.can_bat) =>
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
                        Err(err) => asr::print_message(&format!(
                            "quest manager ERROR: {:?} \n{:?}",
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
                            asr::print_message(&format!("{:#?}", inventory_container));

                            // First feather key
                            match old_inventory_container.map(|ic| ic.feather_keys) {
                                Some(0)
                                    if inventory_container.feather_keys == 1
                                        && settings.first_feather_key =>
                                {
                                    asr::print_message("Split for first feather");
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
                                    asr::print_message("Split for second feather");
                                    timer::split();
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
                            asr::print_message(&format!("{:#?}", enemies_manager));

                            old_enemies_manager = Some(enemies_manager);
                        }
                        Err(err) => asr::print_message(&format!(
                            "enemies manager ERROR: {:?} \n{:?}",
                            err, old_game_manager
                        )),
                        _ => {}
                    }

                    let new_boss_list =
                        get_boss_data_array(&process, old_enemies_manager.as_ref(), &boss_class)
                            .await
                            .ok();
                    if new_boss_list != old_boss_list {
                        asr::print_message(&format!("update Boss {:#?}", new_boss_list));

                        if let (Some(new_list), Some(old_list)) = (&new_boss_list, &old_boss_list) {
                            for (new_boss, old_boss) in new_list.iter().zip(old_list.iter()) {
                                if new_boss != old_boss && new_boss.boss_kind == old_boss.boss_kind
                                {
                                    match new_boss.boss_kind {
                                        BossKind::Placeholder => {}
                                        BossKind::KiriKiriBozu => {
                                            if !old_boss.defeated
                                                && new_boss.defeated
                                                && settings.defeated_kirikiri_boss
                                            {
                                                asr::print_message("KiriKiri Bozu defeated split");
                                                timer::split();
                                            }
                                        }
                                        BossKind::PUA => {}
                                        BossKind::Hashihime => {}
                                        BossKind::Yuki => {}
                                        BossKind::Yokozuna => {}
                                        BossKind::Jorogumo => {}
                                        BossKind::KarasuTengu => {}
                                        BossKind::DaiTengu => {}
                                        BossKind::Gasha => {}
                                        BossKind::Asahi => {}
                                        BossKind::Shogun => {}
                                        BossKind::Amaterasu => {}
                                    }
                                }
                            }

                            old_boss_list = new_boss_list;
                        }
                    }

                    let new_daruma_list =
                        get_daruma_data_array(&process, old_daruma_manager.as_ref(), &daruma_class)
                            .await
                            .ok();
                    if new_daruma_list != old_daruma_list {
                        asr::print_message(&format!("update Daruma {:#?}", new_daruma_list));

                        if let (Some(new_list), Some(old_list)) =
                            (&new_daruma_list, &old_daruma_list)
                        {
                            for (new_daruma, old_daruma) in new_list.iter().zip(old_list.iter()) {
                                if new_daruma != old_daruma
                                    && new_daruma.daruma_type == old_daruma.daruma_type
                                {
                                    match new_daruma.daruma_type {
                                        DarumaType::Bite => {}
                                        DarumaType::Parry => {}
                                        DarumaType::Thorns => {}
                                        DarumaType::Spirits => {}
                                        DarumaType::Bomb => {}
                                        DarumaType::SpinAttack => {}
                                        DarumaType::Deprecated1 => {}
                                        DarumaType::FireWall => {}
                                        DarumaType::Ice => {}
                                        DarumaType::Boomerang => {}
                                    }
                                }
                            }

                            old_daruma_list = new_daruma_list;
                        }
                    }

                    // TODO: Do something on every tick.
                    next_tick().await;
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
