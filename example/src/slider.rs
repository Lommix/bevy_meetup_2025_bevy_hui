use bevy::prelude::*;
use bevy_hui::prelude::*;

mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HuiPlugin))
        .add_systems(Startup, (common::setup, load, bind))
        .add_systems(Update, (update_drag,))
        .add_observer(update_slider_target)
        .run();
}

fn load(mut cmd: Commands, assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    html_comps.register("slider", assets.load("slider/slider.html"));
    cmd.spawn(HtmlNode(assets.load("slider/menu.html")));
}

fn bind(mut html_funcs: HtmlFunctions) {
    html_funcs.register("init_slider", init_slider);
}

fn update_slider_target(
    trigger: Trigger<UiChangedEvent>,
    targets: Query<(&UiTarget, &Slider)>,
    mut texts: Query<&mut Text>,
) {
    let slider_entity = trigger.entity();

    let Ok((target, slider)) = targets.get(slider_entity) else {
        return;
    };

    let Ok(mut text) = texts.get_mut(**target) else {
        return;
    };

    text.0 = format!("Slider Value: {:.2}", slider.value);
}

fn init_slider(In(entity): In<Entity>, children: Query<&Children>, mut cmd: Commands) {
    let nob_entity = children
        .get(entity)
        .ok()
        .map(|children| children.first())
        .flatten()
        .unwrap();

    cmd.entity(entity).insert(Slider { value: 0. });
    cmd.entity(*nob_entity).insert(SliderNob { slider: entity });
}

#[derive(Component)]
pub struct Slider {
    pub value: f32,
}

#[derive(Component)]
pub struct SliderNob {
    slider: Entity,
}

impl Slider {
    pub fn value(&self) -> f32 {
        self.value
    }
}

fn update_drag(
    mut cmd: Commands,
    mut events: EventReader<bevy::input::mouse::MouseMotion>,
    mut nobs: Query<(Entity, &SliderNob, &mut HtmlStyle, &Interaction)>,
    mut sliders: Query<&mut Slider>,
    computed_nodes: Query<&ComputedNode>,
) {
    for event in events.read() {
        nobs.iter_mut()
            .filter(|(_, _, _, interaction)| matches!(interaction, Interaction::Pressed))
            .for_each(|(nob_entity, nob, mut style, _)| {
                let slider_computed = computed_nodes.get(nob.slider).unwrap();
                let nob_computed = computed_nodes.get(nob_entity).unwrap();

                // --------------------
                // math
                let current_pos = match style.computed.node.left {
                    Val::Px(pos) => pos,
                    _ => 0.,
                };

                let max_pos = slider_computed.unrounded_size().x
                    * slider_computed.inverse_scale_factor()
                    - nob_computed.unrounded_size().x * nob_computed.inverse_scale_factor();

                let next_pos = (current_pos
                    + event.delta.x / slider_computed.inverse_scale_factor())
                .min(max_pos)
                .max(0.);

                let slider_value = next_pos / max_pos;
                style.computed.node.left = Val::Px(next_pos);

                let mut slider = sliders.get_mut(nob.slider).unwrap();
                slider.value = slider_value;

                cmd.trigger_targets(UiChangedEvent, nob.slider);
            });
    }
}
