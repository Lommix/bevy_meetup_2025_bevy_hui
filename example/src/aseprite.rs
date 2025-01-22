use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_hui::prelude::*;

mod common;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: bevy::image::ImageSamplerDescriptor::nearest(),
            }),
            HuiPlugin,
            AsepriteUltraPlugin,
        ))
        .add_systems(Startup, (common::setup, load, bind))
        .run();
}

fn load(mut cmd: Commands, assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    html_comps.register("aseprite", assets.load("aseprite/aseprite.html"));
    cmd.spawn(HtmlNode(assets.load("aseprite/menu.html")));
}

fn bind(mut html_funcs: HtmlFunctions) {
    html_funcs.register("attach_aseprite", attach_aseprite);
}

fn attach_aseprite(
    In(entity): In<Entity>,
    tags_q: Query<&Tags>,
    mut cmd: Commands,
    server: Res<AssetServer>,
) {
    let tags = tags_q.get(entity).unwrap();
    let ase_path = tags.get("source").unwrap();

    let animation = tags
        .get("animation")
        .map(|s| Animation::tag(s))
        .unwrap_or(Animation::default());

    cmd.entity(entity).insert(AseUiAnimation {
        aseprite: server.load(ase_path),
        animation,
    });
}
