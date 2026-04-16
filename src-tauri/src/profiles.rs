use crate::types::Profile;

// ─── Profile Definitions (per ENG-3: data, not behavior) ────────────────────

fn s(val: &str) -> String {
    val.to_string()
}

fn sv(vals: &[&str]) -> Vec<String> {
    vals.iter().map(|v| s(v)).collect()
}

pub fn competitive_fps() -> Profile {
    Profile {
        id: s("competitive_fps"),
        name: s("Competitive FPS"),
        description: s("Minimum latency, maximum debloat. For CS2, Valorant, Apex Legends."),
        power_ultimate: true,
        usb_suspend_disable: true,
        nagle_disable: true,
        interrupt_mod_disable: true,
        wake_on_lan_disable: true,
        mouse_accel_disable: true,
        disable_services: sv(&["SysMain", "DiagTrack", "XblAuthManager", "WSearch"]),
        process_priority_high: true,
        background_apps_disable: true,
    }
}

pub fn casual_gaming() -> Profile {
    Profile {
        id: s("casual_gaming"),
        name: s("Casual Gaming"),
        description: s("Balanced performance for single-player and casual multiplayer."),
        power_ultimate: false,
        usb_suspend_disable: true,
        nagle_disable: true,
        interrupt_mod_disable: false,
        wake_on_lan_disable: true,
        mouse_accel_disable: true,
        disable_services: sv(&["DiagTrack", "XblAuthManager"]),
        process_priority_high: false,
        background_apps_disable: true,
    }
}

pub fn video_editing() -> Profile {
    Profile {
        id: s("video_editing"),
        name: s("Video Editing"),
        description: s("Maximum throughput for Premiere, DaVinci Resolve, After Effects."),
        power_ultimate: true,
        usb_suspend_disable: true,
        nagle_disable: false,
        interrupt_mod_disable: false,
        wake_on_lan_disable: false,
        mouse_accel_disable: false,
        disable_services: sv(&["DiagTrack"]),
        process_priority_high: false,
        background_apps_disable: false,
    }
}

pub fn streaming() -> Profile {
    Profile {
        id: s("streaming"),
        name: s("Streaming"),
        description: s("Gaming optimizations that keep OBS and streaming services alive."),
        power_ultimate: false,
        usb_suspend_disable: true,
        nagle_disable: true,
        interrupt_mod_disable: false,
        wake_on_lan_disable: true,
        mouse_accel_disable: true,
        disable_services: sv(&["DiagTrack", "XblAuthManager"]),
        process_priority_high: true,
        background_apps_disable: false,
    }
}

pub fn productivity() -> Profile {
    Profile {
        id: s("productivity"),
        name: s("Productivity"),
        description: s("Balanced mode. Keeps search, indexing, and background services."),
        power_ultimate: false,
        usb_suspend_disable: false,
        nagle_disable: false,
        interrupt_mod_disable: false,
        wake_on_lan_disable: false,
        mouse_accel_disable: false,
        disable_services: sv(&["DiagTrack"]),
        process_priority_high: false,
        background_apps_disable: false,
    }
}

/// Returns all available profiles in display order
pub fn all_profiles() -> Vec<Profile> {
    vec![
        competitive_fps(),
        casual_gaming(),
        video_editing(),
        streaming(),
        productivity(),
    ]
}
