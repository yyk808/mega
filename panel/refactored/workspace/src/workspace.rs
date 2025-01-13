mod dock;
mod pane;

use std::sync::Arc;
use gpui::{Action, AppContext, Div, EventEmitter, FocusHandle, FocusableView, InteractiveElement, IntoElement, Render, View, ViewContext, WeakView}
use theme::ActiveTheme;
use crate::pane::Pane;

/// Collects everything a local Mega client needed.
/// This crate will replace functions in zed crate `workspace`.
/// So we can reuse some other crates in zed without taking the whole repo.
pub struct Workspace {
    weak_self: WeakView<Self>,
    workspace_actions: Vec<Box<dyn Fn(Div, &mut ViewContext<Self>) -> Div>>,
    panes: Vec<View<Pane>>,
    active_pane: View<Pane>,
}

impl EventEmitter<Event> for Workspace {}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(cx);

        let theme = cx.theme().clone();
        let colors = theme.colors();
        
        
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

impl Workspace {
    pub fn register_action<A: Action>(
        &mut self,
        callback: impl Fn(&mut Self, &A, &mut ViewContext<Self>) + 'static,
    ) -> &mut Self {
        let callback = Arc::new(callback);

        self.workspace_actions.push(Box::new(move |div, cx| {
            let callback = callback.clone();
            div.on_action(
                cx.listener(move |workspace, event, cx| (callback.clone())(workspace, event, cx)),
            )
        }));
        self
    }

    fn add_workspace_actions_listeners(&self, mut div: Div, cx: &mut ViewContext<Self>) -> Div {
        for action in self.workspace_actions.iter() {
            div = (action)(div, cx)
        }
        div
    }
}