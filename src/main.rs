use druid::widget::{Container, Flex, Label, List, Split};
use druid::{
    AppDelegate, AppLauncher, Code, Data, Event, PlatformError, Size, Widget,
    WidgetExt, WindowDesc,
};

use yukari_lib::State;
use yukari_widgets::Footer;

// @FIXME fill entire body like footer
fn build_header<T: Data>(body: impl Widget<T> + 'static) -> impl Widget<T> {
    Flex::column()
        .with_flex_child(Label::new("menu.."), 1.)
        .with_flex_child(body, 1.)
        .must_fill_main_axis(true)
}
fn build_footer(body: impl Widget<State> + 'static) -> impl Widget<State> {
    Footer::new(
        body,
        Flex::row()
            .with_flex_child(
                Label::dynamic(|data: &State, _| data.current.to_string_lossy().into_owned()),
                2.,
            )
            .must_fill_main_axis(true),
    )
    .fill_body(true)
    .border(druid::Color::RED, 1.)
}
fn build_main() -> impl Widget<State> {
    Split::columns(
        List::new(|| Label::dynamic(|d: &String, _| d.clone()))
            .lens(State::parent_content),
        List::new(|| Label::dynamic(|d: &String, _| d.clone()))
            .scroll()
            .vertical()
            .lens(State::current_content),
    )
    .draggable(true)
    .solid_bar(true)
    .min_size(150., 300.)
}

fn build_ui() -> impl Widget<State> {
    Container::new(build_footer(build_main()))
}

fn main() -> Result<(), PlatformError> {
    fn update_title(data: &State, _: &druid::Env) -> String {
        data.current.to_string_lossy().into_owned()
    }
    AppLauncher::with_window(
        WindowDesc::new(build_ui())
            .title(update_title)
            .with_min_size(Size::new(300., 300.)),
    )
    .delegate(QExist {})
    .launch(State::cwd())
}

struct QExist {}

impl<T: Data> AppDelegate<T> for QExist {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: Event,
        _data: &mut T,
        _env: &druid::Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyUp(k) => {
                println!("{}", k.code);
                if k.code == Code::KeyQ {
                    std::process::exit(1)
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        }
    }

    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        _cmd: &druid::Command,
        _data: &mut T,
        _env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut T,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut T,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
}
