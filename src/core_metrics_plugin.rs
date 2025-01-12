use bevy::{ecs::entity::Entities, prelude::*};
use metrics::{describe_gauge, describe_histogram, gauge, histogram, Unit};

/// Provides core metrics like frame time, entity count, etc.
pub struct CoreMetricsPlugin;

impl Plugin for CoreMetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, describe_core_metrics)
            .add_systems(Update, update_core_metrics);
    }
}

fn describe_core_metrics() {
    describe_gauge!("frame_time", Unit::Milliseconds, "Frame time delta");
    describe_histogram!("frame_time", Unit::Milliseconds, "Frame time delta");
    describe_gauge!(
        "frames_per_second",
        Unit::CountPerSecond,
        "Frames per second"
    );
    describe_gauge!(
        "entities",
        Unit::Count,
        "The number of entities in the world"
    );
}

fn update_core_metrics(entities: &Entities, time: Res<Time>) {
    gauge!("entities").set(entities.len() as f64);

    let sec = time.delta_secs_f64();
    let ms = 1000.0 * sec;
    let fps = 1.0 / sec;
    histogram!("frame_time").record(ms);
    gauge!("frame_time").set(ms);
    gauge!("frames_per_second").set(fps);
}
