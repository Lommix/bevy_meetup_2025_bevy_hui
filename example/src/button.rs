use bevy::prelude::*;
use bevy_hui::prelude::*;

mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HuiPlugin))
        .add_systems(Startup, (common::setup, load))
        .run();
}

fn load(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn(HtmlNode(assets.load("button/button.html")));
}
