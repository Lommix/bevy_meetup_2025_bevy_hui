## Your first widget

<br />
<br />

Load and display an Aseprite animation from a asset path.

(Using the AsepriteUltraPlugin)

<br />

![img](img/ultra.jpeg)

[https://github.com/Lommix/bevy_aseprite_ultra](https://github.com/Lommix/bevy_aseprite_ultra)

---

## Create a new binding

```rust[|3|7,8|10-13|15-17]
fn attach_aseprite(
    In(entity): In<Entity>,
    tags_q: Query<&Tags>,
    mut cmd: Commands,
    server: Res<AssetServer>,
) {
    let tags = tags_q.get(entity).unwrap();
    let path = tags.get("source").unwrap();

    let animation = tags
        .get("animation")
        .map(|s| Animation::tag(s))
        .unwrap_or(Animation::default());

    cmd.entity(entity).insert(AseUiAnimation {
        aseprite: server.load(path),
        animation,
    });
}
```

---

_aseprite.html_

```html[|5-7|]
<template>
    <property name="source">default</property>
    <property name="animation"></property>
    <node
        on_spawn="attach_aseprite"
        tag:source="{source}"
        tag:animation="{animation}"
    ></node>
</template>
```

_using it_

```html
<aseprite source="coin.aseprite" />
```

---

### Result:

<div class="row">
    <video width="500px" src="img/aseprite.mp4" autoplay=true loop />
</div>

---

## Widgets with state

---

## Simple slider

A absolute Button inside a relative Container.

```html[|2-9|10-16]
<template>
    <node
        on_spawn="init_slider"
        height="42px"
        width="100%"
        background="#222"
        border_color="#FFF"
        border="1px"
    >
        <button
            background="#FFF"
            pressed:background="#A03"
            position="absolute"
            width="40px"
            height="40px"
        ></button>
    </node>
</template>
```

<!-- .element class="fragment" -->

---

```rust[|9|10]
fn init_slider(In(entity): In<Entity>, children: Query<&Children>, mut cmd: Commands) {
    let btn_entity = children
        .get(entity)
        .ok()
        .map(|children| children.first())
        .flatten()
        .unwrap();

    cmd.entity(entity).insert(Slider { value: 0. });
    cmd.entity(*btn_entity).insert(SliderNob { slider: entity });
}

```

---

## Calculate the slider value

```rust[|3-4|10|12-13|15-18|20-21|23]
fn update_drag(
    mut cmd: Commands,
    mut events: EventReader<MouseMotion>,
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

                // % left from container calculation
                let slider_value = ...
                // mouse delta * scale factor
                style.computed.node.left = Val::Px(next_pos);

                let mut slider = sliders.get_mut(nob.slider).unwrap();
                slider.value = slider_value;

                cmd.trigger_targets(UiChangedEvent, nob.slider);
            });
    }
}
```

---

## Listing to our new event

Using target resolves to the entity behind the template

<br />

```xml
<text id="slider_value" font_size="32" > Slider Value: 0.00 </text>
<slider on_change="update_text" target="slider_value" />
```

<br />

```rust[|2-4|6-16]
fn update_slider_target(
    trigger: Trigger<UiChangedEvent>,
    sliders: Query<(&UiTarget, &Slider)>,
    mut texts: Query<&mut Text>,
) {
    let slider_entity = trigger.entity();

    let Ok((target, slider)) = sliders.get(slider_entity) else {
        return;
    };

    let Ok(mut text) = texts.get_mut(**target) else {
        return;
    };

    text.0 = format!("Slider Value: {:.2}", slider.value);
}
```

---

### Result:

<div class="row">
    <video width="500px" src="img/slider.mp4" autoplay=true loop />
</div>
