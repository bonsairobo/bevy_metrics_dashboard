use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_metrics_dashboard::{DashboardPlugin, DashboardWindow, RegistryPlugin};
use metrics::{
    counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram, Unit,
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(RegistryPlugin::default())
        .add_plugins(DashboardPlugin)
        .add_systems(Startup, (describe_metrics, create_dashboard))
        .add_systems(Update, update_metrics)
        .run();
}

fn describe_metrics() {
    describe_counter!("foo", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo", Unit::Count, "Lorem ipsum");
    describe_histogram!("foo", Unit::Count, "Lorem ipsum");
    describe_counter!("bar", Unit::Count, "Lorem ipsum");
    describe_gauge!("bar", Unit::Count, "Lorem ipsum");
    describe_histogram!("bar", Unit::Count, "Lorem ipsum");
    describe_counter!("baz", Unit::Count, "Lorem ipsum");
    describe_gauge!("baz", Unit::Count, "Lorem ipsum");
    describe_histogram!("baz", Unit::Count, "Lorem ipsum");
    describe_counter!("bop", Unit::Count, "Lorem ipsum");
    describe_gauge!("bop", Unit::Count, "Lorem ipsum");
    describe_histogram!("bop", Unit::Count, "Lorem ipsum");
    describe_counter!("doowop", Unit::Count, "Lorem ipsum");
    describe_gauge!("doowop", Unit::Count, "Lorem ipsum");
    describe_histogram!("doowop", Unit::Count, "Lorem ipsum");
    describe_counter!("skibby", Unit::Count, "Lorem ipsum");
    describe_gauge!("skibby", Unit::Count, "Lorem ipsum");
    describe_histogram!("skibby", Unit::Count, "Lorem ipsum");
    describe_counter!("dibby", Unit::Count, "Lorem ipsum");
    describe_gauge!("dibby", Unit::Count, "Lorem ipsum");
    describe_histogram!("dibby", Unit::Count, "Lorem ipsum");
    describe_counter!("doop", Unit::Count, "Lorem ipsum");
    describe_gauge!("doop", Unit::Count, "Lorem ipsum");
    describe_histogram!("doop", Unit::Count, "Lorem ipsum");
    describe_counter!("foo2", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo2", Unit::Count, "Lorem ipsum");
    describe_histogram!("foo2", Unit::Count, "Lorem ipsum");
    describe_counter!("bar2", Unit::Count, "Lorem ipsum");
    describe_gauge!("bar2", Unit::Count, "Lorem ipsum");
    describe_histogram!("bar2", Unit::Count, "Lorem ipsum");
    describe_counter!("baz2", Unit::Count, "Lorem ipsum");
    describe_gauge!("baz2", Unit::Count, "Lorem ipsum");
    describe_histogram!("baz2", Unit::Count, "Lorem ipsum");
    describe_counter!("bop2", Unit::Count, "Lorem ipsum");
    describe_gauge!("bop2", Unit::Count, "Lorem ipsum");
    describe_histogram!("bop2", Unit::Count, "Lorem ipsum");
    describe_counter!("doowop2", Unit::Count, "Lorem ipsum");
    describe_gauge!("doowop2", Unit::Count, "Lorem ipsum");
    describe_histogram!("doowop2", Unit::Count, "Lorem ipsum");
    describe_counter!("skibby2", Unit::Count, "Lorem ipsum");
    describe_gauge!("skibby2", Unit::Count, "Lorem ipsum");
    describe_histogram!("skibby2", Unit::Count, "Lorem ipsum");
    describe_counter!("dibby2", Unit::Count, "Lorem ipsum");
    describe_gauge!("dibby2", Unit::Count, "Lorem ipsum");
    describe_histogram!("dibby2", Unit::Count, "Lorem ipsum");
    describe_counter!("doop2", Unit::Count, "Lorem ipsum");
    describe_gauge!("doop2", Unit::Count, "Lorem ipsum");
    describe_histogram!("doop2", Unit::Count, "Lorem ipsum");
}

fn create_dashboard(mut commands: Commands) {
    commands.spawn(DashboardWindow::new("Metrics Dashboard"));
}

fn update_metrics() {
    let mut rng = rand::thread_rng();

    histogram!("foo").record(rng.gen_range(0.0..10.0));
    gauge!("foo").set(rng.gen_range(0.0..10.0));
    counter!("foo").increment(rng.gen_range(0..10));
    histogram!("bar").record(rng.gen_range(0.0..10.0));
    gauge!("bar").set(rng.gen_range(0.0..10.0));
    counter!("bar").increment(rng.gen_range(0..10));
    histogram!("baz").record(rng.gen_range(0.0..10.0));
    gauge!("baz").set(rng.gen_range(0.0..10.0));
    counter!("baz").increment(rng.gen_range(0..10));
    histogram!("bop").record(rng.gen_range(0.0..10.0));
    gauge!("bop").set(rng.gen_range(0.0..10.0));
    counter!("bop").increment(rng.gen_range(0..10));
    histogram!("doowop").record(rng.gen_range(0.0..10.0));
    gauge!("doowop").set(rng.gen_range(0.0..10.0));
    counter!("doowop").increment(rng.gen_range(0..10));
    histogram!("skibby").record(rng.gen_range(0.0..10.0));
    gauge!("skibby").set(rng.gen_range(0.0..10.0));
    counter!("skibby").increment(rng.gen_range(0..10));
    histogram!("dibby").record(rng.gen_range(0.0..10.0));
    gauge!("dibby").set(rng.gen_range(0.0..10.0));
    counter!("dibby").increment(rng.gen_range(0..10));
    histogram!("doop").record(rng.gen_range(0.0..10.0));
    gauge!("doop").set(rng.gen_range(0.0..10.0));
    counter!("doop").increment(rng.gen_range(0..10));
    histogram!("foo2").record(rng.gen_range(0.0..10.0));
    gauge!("foo2").set(rng.gen_range(0.0..10.0));
    counter!("foo2").increment(rng.gen_range(0..10));
    histogram!("bar2").record(rng.gen_range(0.0..10.0));
    gauge!("bar2").set(rng.gen_range(0.0..10.0));
    counter!("bar2").increment(rng.gen_range(0..10));
    histogram!("baz2").record(rng.gen_range(0.0..10.0));
    gauge!("baz2").set(rng.gen_range(0.0..10.0));
    counter!("baz2").increment(rng.gen_range(0..10));
    histogram!("bop2").record(rng.gen_range(0.0..10.0));
    gauge!("bop2").set(rng.gen_range(0.0..10.0));
    counter!("bop2").increment(rng.gen_range(0..10));
    histogram!("doowop2").record(rng.gen_range(0.0..10.0));
    gauge!("doowop2").set(rng.gen_range(0.0..10.0));
    counter!("doowop2").increment(rng.gen_range(0..10));
    histogram!("skibby2").record(rng.gen_range(0.0..10.0));
    gauge!("skibby2").set(rng.gen_range(0.0..10.0));
    counter!("skibby2").increment(rng.gen_range(0..10));
    histogram!("dibby2").record(rng.gen_range(0.0..10.0));
    gauge!("dibby2").set(rng.gen_range(0.0..10.0));
    counter!("dibby2").increment(rng.gen_range(0..10));
    histogram!("doop2").record(rng.gen_range(0.0..10.0));
    gauge!("doop2").set(rng.gen_range(0.0..10.0));
    counter!("doop2").increment(rng.gen_range(0..10));
}
