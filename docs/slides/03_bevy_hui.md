# Overview

<br/>
<br/>

- Stay close to bevy_ui <!-- .element: class="fragment" -->
- Hot reload at any time <!-- .element: class="fragment" -->
- Separate design from logic <!-- .element: class="fragment" -->
- Simple API to wire up logic <!-- .element: class="fragment" -->
- Some utility (Animations & Transitions) <!-- .element: class="fragment" -->

---

# Prepare

_main.rs_

```rust [|3|10|]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HuiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn(Camera2d);
    cmd.spawn(HtmlNode(assets.load("template.html")));
}
```

_template.html_

```html
<template>
    <node
        background="#222"
        padding="50px"
        border="5px"
        border_radius="5px"
        border_color="#FFF"
    >
        <text font_size="64">Hello World!</text>
    </node>
</template>
```

---

![img](img/hello.png)

---

# Components

_main.rs_

```rust[|1|4]
fn setup(mut cmd: Commands, mut html_comps: HtmlComponents, assets: Res<AssetServer>) {
    cmd.spawn(Camera2d);

    html_comps.register("hello", assets.load("hello.html"));
    cmd.spawn(HtmlNode(assets.load("menu.html")));
}
```

_menu.html_

```html
<template>
    <node
        margin="100px"
        display="grid"
        row_gap="20px"
    >
        <hello />
        <hello />
        <hello />
        <hello />
    </node>
</template>
```

---

![img](img/components.png)

---

# Autoloading

<br/>
<br/>
<br/>

Auto register any template in folder as component using the filename.

<br/>

```rust[|6]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HuiPlugin,
            HuiAutoLoadPlugin::new(&["components"])
        ))
        .add_systems(Startup, (common::setup, load))
        .run();
}
```

<br/>

Useful for components, which do not require custom spawn logic.

<!-- .element: class="fragment" -->

---

## Errors

Context rich error handling

<div class="row" style="max-height:55rem">
    <video src="img/error.mp4" loop autoplay="true" />
</div>
