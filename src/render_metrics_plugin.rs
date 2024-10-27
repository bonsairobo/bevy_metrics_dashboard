use bevy::{prelude::*, render::view::VisibleEntities};
use metrics::{describe_gauge, gauge, Unit};

/// Provides rendering metrics like visible entities, etc.
pub struct RenderMetricsPlugin;

impl Plugin for RenderMetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, describe_render_metrics)
            .add_systems(Update, update_render_metrics);
    }
}

fn describe_render_metrics() {
    describe_gauge!(
        "visible_2d_entities",
        Unit::Count,
        "The number of entities visible to some 2D camera"
    );
    describe_gauge!(
        "visible_3d_entities",
        Unit::Count,
        "The number of entities visible to some 3D camera"
    );
}

fn update_render_metrics(
    cameras_2d: Query<(Entity, &VisibleEntities), With<Camera2d>>,
    cameras_3d: Query<(Entity, &VisibleEntities), With<Camera3d>>,
) {
    for (entity, visible) in &cameras_2d {
        let labels = [(
            "entity",
            format!("{}v{}", entity.index(), entity.generation()),
        )];
        let gauge = gauge!("visible_2d_entities", &labels);
        gauge.set(visible.len::<()>() as f64);
    }
    for (entity, visible) in &cameras_3d {
        let labels = [(
            "entity",
            format!("{}v{}", entity.index(), entity.generation()),
        )];
        let gauge = gauge!("visible_3d_entities", &labels);
        gauge.set(visible.len::<()>() as f64);
    }
}
