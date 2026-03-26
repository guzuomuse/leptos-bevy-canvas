use bevy::prelude::*;

use crate::leptos_component::AWAITING_CLEANUP;
use std::sync::atomic::Ordering;

/// Clears the cleanup flag when Bevy's World is dropped during shutdown.
#[derive(Resource)]
struct CleanupFlag;

impl Drop for CleanupFlag {
    fn drop(&mut self) {
        AWAITING_CLEANUP.store(false, Ordering::Relaxed);
    }
}

pub struct LeptosBevyCanvasPlugin;
impl Plugin for LeptosBevyCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CleanupFlag);
        app.add_message::<LeptosBevyCanvasCleanup>();
        app.add_systems(First, cleanup);
    }
}

#[derive(Message, Debug)]
pub struct LeptosBevyCanvasCleanup;
fn cleanup(
    mut cleanup_messages: MessageReader<LeptosBevyCanvasCleanup>,
    mut exit: MessageWriter<AppExit>,
) {
    for _ in cleanup_messages.read() {
        exit.write(AppExit::Success);
    }
}
