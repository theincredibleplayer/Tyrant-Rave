use crate::*;
static mut FIRST_FINAL_STUB_OFFSET  : usize = 0x0; // 13.0.4 : 0x8b8e74
static mut SECOND_FINAL_STUB_OFFSET : usize = 0x0; // 13.0.4 : 0x8b9264

unsafe extern "C" fn final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    AreaModule::set_whole(fighter.module_accessor,false);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    KineticModule::clear_speed_all(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DISABLE_OUTSIDE_ENERGY);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_start"),0.0,1.0,false,0.0,false,false);
    if (fighter.global_table[0x16] != *SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(final_main_loop as *const () as _))
}

unsafe extern "C" fn final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

unsafe extern "C" fn game_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SLOW_OPPONENT(agent, 400.0, 140.0);
        macros::FT_START_CUTIN(agent);
    }

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FT_SET_FINAL_FEAR_FACE(agent, 138);
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
            if macros::is_excute(agent) {
                MotionModule::set_flip(agent.module_accessor,true,true,false);
            }
        }
        macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
    }

    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        CameraModule::zoom_out(agent.module_accessor, 30);
    }

    frame(agent.lua_state_agent, 139.0);
    if macros::is_excute(agent) {
        macros::SLOW_OPPONENT(agent, 15.0, 100.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 20.0, 90, 100, 40, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 20.0, 90, 100, 40, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }

    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

    frame(agent.lua_state_agent, 177.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 30.0, 50, 90, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 30.0, 50, 90, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, false, -1.0, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::SLOW_OPPONENT(agent, 1.0, 100.0);
    }

    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}

unsafe extern "C" fn effect_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_captain_final"), false, false, false);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.4, 0.4, 0.4, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 0.3, true);
    }
    frame(agent.lua_state_agent, 107.0);
    for _ in 0..32 {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_curry_shot"), Hash40::new("haver"), 0, 0, 0, 180.0, 0.0, 0.0, 1, true);
    }
    wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 106.0);
    if macros::is_excute(agent) {
        EffectModule::remove_screen(agent.module_accessor, Hash40::new("bg_captain_final"), 38);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("captain_fp_hold"), false, true);
    }
    frame(agent.lua_state_agent, 139.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0,0,0, true);
    }
    frame(agent.lua_state_agent, 142.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_curry_shot"), false, true);
    }
    frame(agent.lua_state_agent, 173.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 13.8, 13, 0, 0, 0, 0.9, 0, 0, 0, 0,0,0, true);
        let lr = PostureModule::lr(agent.module_accessor);
        let falcon_id = ArticleModule::get_active_num(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,Hash40::new("special_n_l"),false,-1.0);
        ArticleModule::set_rate(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,0.8);
        let falcon = ArticleModule::get_article_from_no(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,falcon_id);
        let falcon_boma = *(falcon as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        PostureModule::set_scale(falcon_boma,2.0,false);
        PostureModule::add_pos_2d(falcon_boma, &Vector2f{x:20.0*lr,y:0.0});

        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_curry_shot"), Hash40::new("haver"), 0, 0, 0, 180.0, 0.0, 0.0, 2, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    frame(agent.lua_state_agent, 200.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 30);
    }
    frame(agent.lua_state_agent, 239.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::set_flip(agent.module_accessor,false,true,false);
    }
}

unsafe extern "C" fn game_finalstart_com(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        CameraModule::zoom_out(agent.module_accessor, 30);
    }
    frame(agent.lua_state_agent, 139.0);
    if macros::is_excute(agent) {
        macros::SLOW_OPPONENT(agent, 15.0, 100.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 20.0, 90, 100, 40, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 20.0, 90, 100, 40, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 177.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 30.0, 50, 90, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 30.0, 50, 90, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::SLOW_OPPONENT(agent, 1.0, 100.0);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_finalstart_sol(agent: &mut L2CAgentBase) {
    let random_tyrant = smash::app::sv_math::rand(hash40("fighter"), 3);
    if random_tyrant == 0 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_1_1"));
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n04"));        
        }
        frame(agent.lua_state_agent, 93.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_1_2"));
        }
        frame(agent.lua_state_agent, 175.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_captain_special_n04"));
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_1_3"));
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
        }
    }else if random_tyrant == 1{
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_2_1"));
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n04"));        
        }
        frame(agent.lua_state_agent, 93.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_2_2"));
        }
        frame(agent.lua_state_agent, 175.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_captain_special_n04"));
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_2_3"));
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
        }
    }else if random_tyrant == 2 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_3_1"));
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n04"));        
        }
        frame(agent.lua_state_agent, 93.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_3_2"));
        }
        frame(agent.lua_state_agent, 175.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_captain_special_n04"));
            macros::PLAY_SE(agent, Hash40::new("vc_tyrant_3_3"));
            macros::PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
        }
    }
}

unsafe extern "C" fn sound_finalstart_captain(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_captain_special_n04"));        
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_captain_005"));

    }
    frame(agent.lua_state_agent, 115.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_captain_001"));
    }
    frame(agent.lua_state_agent, 175.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        macros::PLAY_SE(agent, Hash40::new("vc_captain_002"));
        macros::PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}


#[skyline::hook(offset = FIRST_FINAL_STUB_OFFSET, inline)]
unsafe extern "C" fn first_final_stub(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[19].x() as *mut BattleObjectModuleAccessor;
    if is_tyrant(module_accessor){
        ctx.registers[0].set_w(0);
    }
}

#[skyline::hook(offset = SECOND_FINAL_STUB_OFFSET, inline)]
unsafe extern "C" fn second_final_stub(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[22].x() as *mut BattleObjectModuleAccessor;
    if is_tyrant(module_accessor){
        ctx.registers[0].set_w(0);
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_FIRST_FINAL_STUB_OFFSET: &[u8] = &[
    0x1f, 0x80, 0x07, 0x71, 0x01, 0xfb, 0xff, 0x54, 0xf4, 0xfa, 0xff, 0xb4, 0x80, 0x26, 0x40, 0xb9, 0xb7, 0xcd, 0xeb, 0x97, 0x80, 0x0f, 0x00, 0xb4, 0x08, 0xe8, 0x40, 0x39, 0x1f, 0x0d, 0x00, 0x71, 0x29, 0x0f, 0x00, 0x54
];

static OFFSET_SEARCH_SECOND_FINAL_STUB_OFFSET: &[u8] = &[
    0x1f, 0x80, 0x07, 0x71, 0x01, 0x08, 0x00, 0x54, 0xd3, 0x2a, 0x40, 0xf9, 0x68, 0x02, 0x40, 0xf9, 0x08, 0x85, 0x40, 0xf9, 0x94, 0x01, 0x80, 0x52, 0x14, 0x20, 0xa4, 0x72, 0xe0, 0x03, 0x13, 0xaa, 0xe1, 0x03, 0x14, 0x2a, 0x00, 0x01, 0x3f, 0xd6
];


pub fn install() {
    unsafe {
        unsafe {
            let text_ptr = getRegionAddress(Region::Text) as *const u8;
            let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
            let text = std::slice::from_raw_parts(text_ptr, text_size);
            if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_FIRST_FINAL_STUB_OFFSET) {
                FIRST_FINAL_STUB_OFFSET = offset;
                println!("first final stub = {:x}",offset);
            }
            if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_SECOND_FINAL_STUB_OFFSET) {
                SECOND_FINAL_STUB_OFFSET = offset;
                println!("second final stub = {:x}",offset);
            }
        }
        

        let mut costume_sol = &mut Vec::new();
        unsafe {
            for i in 0..crate::SOL_COLORS.len() {
                if crate::SOL_COLORS[i] {
                    costume_sol.push(i);
                }
            }
        }
        let mut costume_captain = &mut Vec::new();
        unsafe {
            for i in 0..crate::CAPTAIN_COLORS.len() {
                if crate::CAPTAIN_COLORS[i] {
                    costume_captain.push(i);
                }
            }
        }


        Agent::new("captain").set_costume(costume_sol.to_vec())
        .status (Main                       , *FIGHTER_STATUS_KIND_FINAL    ,final_main)
        .acmd   ("game_finalstart"          , game_finalstart           ,Priority::Default)
        .acmd   ("game_finalairstart"       , game_finalstart           ,Priority::Default)
        .acmd   ("game_finalstart_com"      , game_finalstart_com       ,Priority::Default)
        .acmd   ("effect_finalstart"        , effect_finalstart         ,Priority::Default)
        .acmd   ("effect_finalairstart"     , effect_finalstart         ,Priority::Default)
        .acmd   ("sound_finalstart"         , sound_finalstart_sol      ,Priority::Default)
        .acmd   ("sound_finalairstart"      , sound_finalstart_sol      ,Priority::Default)
        .install();

        Agent::new("captain").set_costume(costume_captain.to_vec())
        .status (Main                       , *FIGHTER_STATUS_KIND_FINAL    ,final_main)
        .acmd   ("game_finalstart"          , game_finalstart           ,Priority::Default)
        .acmd   ("game_finalairstart"       , game_finalstart           ,Priority::Default)
        .acmd   ("game_finalstart_com"      , game_finalstart_com       ,Priority::Default)
        .acmd   ("effect_finalstart"        , effect_finalstart         ,Priority::Default)
        .acmd   ("effect_finalairstart"     , effect_finalstart         ,Priority::Default)
        .acmd   ("sound_finalstart"         , sound_finalstart_captain  ,Priority::Default)
        .acmd   ("sound_finalairstart"      , sound_finalstart_captain  ,Priority::Default)
        .install();

        skyline::install_hooks!(
            first_final_stub,
            second_final_stub,
        );
    }
}
