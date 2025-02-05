//! Systems to insert components on loaded scenes.
//!
//! Please see the [`SceneHook`] documentation for detailed examples.
mod hook;
pub mod reload;

use bevy::{ecs::system::SystemParam, prelude::*};

pub use hook::{run_hooks, SceneHook, SceneHooked};

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    pub scene: SceneBundle,
}

#[derive(Bundle)]
pub struct HookedDynamicSceneBundle {
    pub hook: SceneHook,
    pub scene: DynamicSceneBundle,
}

/// Convenience parameter to query if a scene marked with `M` has been loaded.
#[derive(SystemParam)]
pub struct HookedSceneState<'w, 's, M: Component> {
    query: Query<'w, 's, (), (With<M>, With<SceneHooked>)>,
}
impl<'w, 's, T: Component> HookedSceneState<'w, 's, T> {
    pub fn is_loaded(&self) -> bool {
        self.query.iter().next().is_some()
    }
}

/// Convenience run criteria to query if a scene marked with `M` has been loaded.
pub fn is_scene_hooked<M: Component>(state: HookedSceneState<M>) -> bool {
    state.is_loaded()
}

/// Systems defined in the [`bevy_scene_hook`](crate) crate (this crate).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum Systems {
    /// System running the hooks.
    SceneHookRunner,
}

/// Plugin to run hooks associated with spawned scenes.
pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_hooks.in_set(Systems::SceneHookRunner));
    }
}
