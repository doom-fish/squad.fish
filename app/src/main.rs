use app::SquadFish;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem, WindowOptions};
use theme::active_theme::ActiveTheme;
use ui::VisualContext;
mod app;
actions!(squadfish, [Quit]);

fn main() {
    com::example_main();
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Image".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        ActiveTheme::init(cx);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| {
                cx.on_release(|_2, _s, cx| {
                    cx.quit();
                })
                .detach();

                SquadFish {}
            })
        })
        .unwrap();
        cx.activate(true);
    });
}
