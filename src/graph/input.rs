use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(InputAction)]
#[action_output(bool)]
pub struct DeleteModifier;

#[derive(Component)]
struct GraphContext;

pub fn setup_input(mut commands: Commands) {
    commands.spawn((
        GraphContext,
        actions!(GraphContext[
            (Action::<DeleteModifier>::new(), bindings![KeyCode::ControlLeft, KeyCode::ControlRight]),
        ]),
    ));
}
