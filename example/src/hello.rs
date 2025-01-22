use bevy::prelude::*;
use bevy_hui::prelude::*;

mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HuiPlugin, HuiAutoLoadPlugin::new(&["components"])))
        .add_systems(Startup, (common::setup, load))
        .run();
}

fn load(mut cmd: Commands, mut html_comps: HtmlComponents, assets: Res<AssetServer>) {
    html_comps.register("hello", assets.load("hello.html"));
    cmd.spawn(HtmlNode(assets.load("menu.html")));
}
