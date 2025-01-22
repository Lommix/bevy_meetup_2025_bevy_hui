## A reusable Button

---

## Let's start simple

Creating a button that does something

<br/>

```html[|3-10|12-17]
<template>
    <button
        background="#111"
        height="80px"
        width="100%"
        border_radius="5px"
        border="4px"
        border_color="#FFF"
        justify_content="center"
        align_items="center"
    >
        <text
            font_size="18px"
            font_color="#FFF"
        >
            This is a Button
        </text>
    </button>
</template>
```

![img](img/button_1.png)

---

## Adding spice

```html[9-13|18-21|]
<button
    background="#111"
    height="80px"
    width="100%"
    border_radius="5px"
    border="4px"
    border_color="#FFF"
    justify_content="center"
    id="btn"
    hover:background="#FFF"
    align_items="center"
    delay="200ms"
    ease="cubic_in"
>
    <text
        font_size="18px"
        font_color="#FFF"
        watch="btn"
        hover:font_color="#111"
        ease="cubic_in"
        delay="200ms"
    >
        This is a Button
    </text>
</button>
```

<div class="row">
    <video width="500px" src="img/button_2.mp4" autoplay=true loop />
</div>

---

## Properties

Make them reusable

---

_my_button.html_

```html[|2-5|7-8,10]
<template>
    <property name="primary">#131313</property>
    <property name="secondary">#AFAFBB</property>
    <property name="action">the_default_value</property>
    <property name="text">placeholder</property>
    <button
        background="{primary}"
        on_press="{action}"
    >
        <text text_color="{secondary}">{text}</text>
    </button>
</template>
```

---

_menu.html_

```html
<my_button
    primary="#000"
    secondary="#FFF"
    action="start_game"
    text="play"
/>
```

---

<div class="row">
    <video width="500px" src="img/reuse.mp4" autoplay=true loop />
</div>

---

## Events

Hooking up the bevy backend.

---

## Bind a function

<br/>

```rust
fn setup(mut html_funcs: HtmlFunctions) {
    html_funcs.register("start_game", start_game);
}

fn start_game(In(entity): In<Entity>, ...){
    // start your game ...
}
```

---

<br />
<br />

### Events for all components:

Node is rendered & fully compiled

- `on_spawn`

<br />
<br />

---

##### Events for interaction component:

All about buttons

- `on_press`
- `on_enter`
- `on_exit`

<br />
<br />

---

##### Event for custom logic:

For all your widgets needs.

- `on_change`

---

## Tags

Custom data for your components

<br />

```html
<button
    on_press="start_game"
    tag:difficulty="hard"
></button>
```

<br />

```rust
fn start_game(In(entity): In<Entity>, tags: Query<&Tags>){

    let tags = tags.get(entity).unwrap();
    let difficulty = tags.get("difficulty").unwrap();

    match difficulty {
        "normal" | _ => {
            // ...
        },
        "hard" => {
            // ...
        },
    }
}
```
