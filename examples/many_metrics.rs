use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_metrics_dashboard::{DashboardPlugin, DashboardWindow};
use metrics::{
    counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram, Unit,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DashboardPlugin::default())
        .add_systems(Startup, (describe_metrics, create_dashboard))
        .add_systems(Update, update_metrics)
        .run();
}

fn describe_metrics() {
    describe_counter!("foo", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("foo", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("bar", Unit::Count, "Lorem ipsum");
    describe_gauge!("bar", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("bar", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("baz", Unit::Count, "Lorem ipsum");
    describe_gauge!("baz", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("baz", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("bop", Unit::Count, "Lorem ipsum");
    describe_gauge!("bop", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("bop", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("doowop", Unit::Count, "Lorem ipsum");
    describe_gauge!("doowop", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("doowop", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("skibby", Unit::Count, "Lorem ipsum");
    describe_gauge!("skibby", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("skibby", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("dibby", Unit::Count, "Lorem ipsum");
    describe_gauge!("dibby", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("dibby", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("zoom", Unit::Count, "Lorem ipsum");
    describe_gauge!("zoom", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("zoom", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("foo2", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("foo2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("bar2", Unit::Count, "Lorem ipsum");
    describe_gauge!("bar2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("bar2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("baz2", Unit::Count, "Lorem ipsum");
    describe_gauge!("baz2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("baz2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("bop2", Unit::Count, "Lorem ipsum");
    describe_gauge!("bop2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("bop2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("doowop2", Unit::Count, "Lorem ipsum");
    describe_gauge!("doowop2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("doowop2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("skibby2", Unit::Count, "Lorem ipsum");
    describe_gauge!("skibby2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("skibby2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("dibby2", Unit::Count, "Lorem ipsum");
    describe_gauge!("dibby2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("dibby2", Unit::Milliseconds, "Lorem ipsum");
    describe_counter!("zoom2", Unit::Count, "Lorem ipsum");
    describe_gauge!("zoom2", Unit::Milliseconds, "Lorem ipsum");
    describe_histogram!("zoom2", Unit::Milliseconds, "Lorem ipsum");
}

fn create_dashboard(mut commands: Commands) {
    commands.spawn(DashboardWindow::new("Metrics Dashboard"));
}

fn update_metrics(time: Res<Time>) {
    let sec = time.delta_seconds_f64();
    let ms = 1000.0 * sec;

    histogram!("foo").record(ms);
    gauge!("foo").set(ms);
    counter!("foo").increment(1);
    histogram!("bar").record(ms);
    gauge!("bar").set(ms);
    counter!("bar").increment(1);
    histogram!("baz").record(ms);
    gauge!("baz").set(ms);
    counter!("baz").increment(1);
    histogram!("bop").record(ms);
    gauge!("bop").set(ms);
    counter!("bop").increment(1);
    histogram!("doowop").record(ms);
    gauge!("doowop").set(ms);
    counter!("doowop").increment(1);
    histogram!("skibby").record(ms);
    gauge!("skibby").set(ms);
    counter!("skibby").increment(1);
    histogram!("dibby").record(ms);
    gauge!("dibby").set(ms);
    counter!("dibby").increment(1);
    histogram!("zoom").record(ms);
    gauge!("zoom").set(ms);
    counter!("zoom").increment(1);
    histogram!("foo2").record(ms);
    gauge!("foo2").set(ms);
    counter!("foo2").increment(1);
    histogram!("bar2").record(ms);
    gauge!("bar2").set(ms);
    counter!("bar2").increment(1);
    histogram!("baz2").record(ms);
    gauge!("baz2").set(ms);
    counter!("baz2").increment(1);
    histogram!("bop2").record(ms);
    gauge!("bop2").set(ms);
    counter!("bop2").increment(1);
    histogram!("doowop2").record(ms);
    gauge!("doowop2").set(ms);
    counter!("doowop2").increment(1);
    histogram!("skibby2").record(ms);
    gauge!("skibby2").set(ms);
    counter!("skibby2").increment(1);
    histogram!("dibby2").record(ms);
    gauge!("dibby2").set(ms);
    counter!("dibby2").increment(1);
    histogram!("zoom2").record(ms);
    gauge!("zoom2").set(ms);
    counter!("zoom2").increment(1);
}
