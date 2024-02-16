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
        .add_plugins(EguiPlugin)
        .add_plugins(RegistryPlugin::default())
        .add_plugins(DashboardPlugin)
        .add_systems(Startup, (describe_metrics, create_dashboard))
        .add_systems(Update, update_metrics)
        .run();
}

fn describe_metrics() {
    describe_counter!("foo::bar::baz", Unit::Count, "Lorem ipsum");
    describe_counter!("foo::bar::fizz", Unit::Count, "Lorem ipsum");
    describe_counter!("foo::foo::baz", Unit::Count, "Lorem ipsum");
    describe_counter!("fizz::bat", Unit::Count, "Lorem ipsum");

    describe_gauge!("foo::bar::baz", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo::bar::fizz", Unit::Count, "Lorem ipsum");
    describe_gauge!("foo::foo::baz", Unit::Count, "Lorem ipsum");
    describe_gauge!("fizz::bat", Unit::Count, "Lorem ipsum");

    describe_histogram!("foo::bar::baz", Unit::Count, "Lorem ipsum");
    describe_histogram!("foo::bar::fizz", Unit::Count, "Lorem ipsum");
    describe_histogram!("foo::foo::baz", Unit::Count, "Lorem ipsum");
    describe_histogram!("fizz::bat", Unit::Count, "Lorem ipsum");

    describe_counter!("fizz::partial::collapsed", Unit::Count, "Lorem ipsum");
    describe_counter!("fully::collapsed::path", Unit::Count, "Lorem ipsum");

    // Invalid paths.
    describe_counter!("edge::case::", Unit::Count, "Lorem ipsum");
    describe_counter!("edge::::case", Unit::Count, "Lorem ipsum");
    describe_counter!("edge:::case", Unit::Count, "Lorem ipsum");
    describe_counter!("::edge", Unit::Count, "Lorem ipsum");
    describe_counter!("::edge::", Unit::Count, "Lorem ipsum");
    describe_counter!(":edge", Unit::Count, "Lorem ipsum");
    describe_counter!("edge:", Unit::Count, "Lorem ipsum");
    describe_counter!(":edge:", Unit::Count, "Lorem ipsum");
    describe_counter!("::", Unit::Count, "Lorem ipsum");
    describe_counter!(":", Unit::Count, "Lorem ipsum");
    describe_counter!("", Unit::Count, "Lorem ipsum");
}

fn create_dashboard(mut commands: Commands) {
    commands.spawn(DashboardWindow::new("Dashboard"));
}

fn update_metrics() {
    let mut rng = rand::thread_rng();

    counter!("foo::bar::baz").increment(rng.gen_range(0..10));
    counter!("foo::bar::fizz").increment(rng.gen_range(0..10));
    counter!("foo::foo::baz").increment(rng.gen_range(0..10));
    counter!("fizz::bat").increment(rng.gen_range(0..10));

    gauge!("foo::bar::baz").set(rng.gen_range(0.0..10.0));
    gauge!("foo::bar::fizz").set(rng.gen_range(0.0..10.0));
    gauge!("foo::foo::baz").set(rng.gen_range(0.0..10.0));
    gauge!("fizz::bat").set(rng.gen_range(0.0..10.0));

    histogram!("foo::bar::baz").record(rng.gen_range(0.0..10.0));
    histogram!("foo::bar::fizz").record(rng.gen_range(0.0..10.0));
    histogram!("foo::foo::baz").record(rng.gen_range(0.0..10.0));
    histogram!("fizz::bat").record(rng.gen_range(0.0..10.0));

    counter!("fizz::partial::collapsed").increment(rng.gen_range(0..10));
    counter!("fully::collapsed::path").increment(rng.gen_range(0..10));

    // Invalid paths.
    counter!("edge::case::").increment(rng.gen_range(0..10));
    counter!("edge::::case").increment(rng.gen_range(0..10));
    counter!("edge:::case").increment(rng.gen_range(0..10));
    counter!("::edge").increment(rng.gen_range(0..10));
    counter!("::edge::").increment(rng.gen_range(0..10));
    counter!(":edge").increment(rng.gen_range(0..10));
    counter!("edge:").increment(rng.gen_range(0..10));
    counter!(":edge:").increment(rng.gen_range(0..10));
    counter!("::").increment(rng.gen_range(0..10));
    counter!(":").increment(rng.gen_range(0..10));
    counter!("").increment(rng.gen_range(0..10));
}
