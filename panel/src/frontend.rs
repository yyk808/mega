mod assets;
mod window;

use crate::frontend::assets::Assets;
use common::config::Config as MegaConfig;
use gpui::{div, point, px, rgb, size, App, AppContext, Bounds, IntoElement, ParentElement, Render, SharedString, Styled, TitlebarOptions, ViewContext, VisualContext, WindowKind, WindowOptions};
use theme::{ActiveTheme, SystemAppearance};
use uuid::Uuid;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}

pub(crate) async fn init(_config: &MegaConfig) {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        settings::init(cx);
        SystemAppearance::init(cx);
        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        
        let options = build_window_options(None, cx);

        cx.open_window(
            options,
            |cx| {
            cx.new_view(|_cx| HelloWorld { text: "Test Window".into(), })
        }).unwrap();
    });
}

pub(crate) fn build_window_options(display_uuid: Option<Uuid>, cx: &mut AppContext) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    let window_decorations = gpui::WindowDecorations::Client;
    let app_id = "Dev.Mega.Panel";

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        window_background: cx.theme().window_background_appearance(),
        app_id: Some(app_id.to_owned()),
        window_decorations: Some(window_decorations),
        window_min_size: Some(gpui::Size {
            width: px(360.0),
            height: px(240.0),
        }),
    }
}
