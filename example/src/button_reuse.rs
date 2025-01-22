use bevy::prelude::*;
use bevy_hui::prelude::*;

mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HuiPlugin))
        .add_systems(Startup, (common::setup, load, bind))
        .run();
}

fn load(mut cmd: Commands, assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    html_comps.register("btn", assets.load("button_reuse/button.html"));
    cmd.spawn(HtmlNode(assets.load("button_reuse/menu.html")));
}
fn bind(mut html_funcs: HtmlFunctions) {
    html_funcs.register("play", |In(entity)| {
        info!("play! {}", entity);
    });

    html_funcs.register("settings", |In(entity)| {
        info!("settings! {}", entity);
    });

    html_funcs.register("exit", |In(entity)| {
        info!("exit! {}", entity);
    });
}
