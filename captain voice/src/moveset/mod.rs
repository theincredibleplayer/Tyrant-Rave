use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::macros::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::{Vector2f,Vector3f};
use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash_script::macros;
use std::arch::asm;
use std::f32::consts::PI;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use smash::lua2cpp::L2CFighterBase;
use crate::FIGHTER_MANAGER;


pub unsafe extern "C" fn final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1); 
    AreaModule::set_whole(fighter.module_accessor,false);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    KineticModule::clear_speed_all(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DISABLE_OUTSIDE_ENERGY);
    if ((fighter.global_table[0x16] == *SITUATION_KIND_GROUND) == false) {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0xb9e61061f),0.0,1.0,false,0.0,false,false); //final 0xf9b673ae9
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
      }
      else {
        MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0xb9e61061f),0.0,1.0,false,0.0,false,false); //finalair is 0xb9e61061f
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(final_main_loop as *const () as _))
}

pub unsafe extern "C" fn final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

unsafe extern "C" fn game_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 140.0);
        macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        macros::FT_START_CUTIN(agent);
    }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 138);
        }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
        if macros::is_excute(agent) {
            MotionModule::set_flip(agent.module_accessor,true,true,false);
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        }
        else{
        if macros::is_excute(agent) {
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        }
    }
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
            macros::SLOW_OPPONENT(agent, 1.0, 100.0);
        }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_finalairstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 140.0);
        macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        macros::FT_START_CUTIN(agent);
    }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 138);
        }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
        if macros::is_excute(agent) {
            MotionModule::set_flip(agent.module_accessor,true,true,false);
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        }
        else{
        if macros::is_excute(agent) {
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
        }
    }
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
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
    frame(agent.lua_state_agent, 176.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 13.8, 13, 0, 0, 0, 0.9, 0, 0, 0, 0,0,0, true);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,Hash40::new("special_n_l"),false,-1.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_curry_shot"), Hash40::new("haver"), 0, 0, 0, 180.0, 0.0, 0.0, 2, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    frame(agent.lua_state_agent, 220.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_curry_shot"), false, true);
    }
    frame(agent.lua_state_agent, 239.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::set_flip(agent.module_accessor,false,true,false);
    }
}

unsafe extern "C" fn effect_finalairstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_captain_final"), false, false, false);
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
    frame(agent.lua_state_agent, 176.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0, 13.8, 13, 0, 0, 0, 0.9, 0, 0, 0, 0,0,0, true);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH,Hash40::new("special_n_l"),false,-1.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_curry_shot"), Hash40::new("haver"), 0, 0, 0, 180.0, 0.0, 0.0, 2, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EffectModule::set_rate_last(agent.module_accessor, 0.5);
    }
    frame(agent.lua_state_agent, 220.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_curry_shot"), false, true);
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
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
            AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
            macros::SLOW_OPPONENT(agent, 1.0, 100.0);
        }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_finalstart(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_finalairstart(agent: &mut L2CAgentBase) {
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


pub fn install() {
    unsafe {
        Agent::new("captain")
        .status(Main, *FIGHTER_STATUS_KIND_FINAL, final_main)
        .game_acmd("game_finalstart", game_finalstart)
        .game_acmd("game_finalairstart", game_finalairstart)
        .game_acmd("game_finalstart_com", game_finalstart_com)
        .effect_acmd("effect_finalstart", effect_finalstart)
        .effect_acmd("effect_finalairstart", effect_finalairstart)
        .sound_acmd("sound_finalstart", sound_finalstart)
        .sound_acmd("sound_finalairstart", sound_finalairstart)
        .install();
        skyline::patching::Patch::in_text(0x8b8e58).data(0x17FFFFD8); // Prevents on attack events related to Final Smash (big thanks to wuboy)
        skyline::patching::Patch::in_text(0x8b9248).data(0x14000040); // Prevents on attack events related to Final Smash (big thanks to wuboy)
    }
}
