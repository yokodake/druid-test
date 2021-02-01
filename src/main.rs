use druid::{
    AppDelegate, AppLauncher, Code, Command, Data, Event, LocalizedString, PlatformError, Size,
    Target, Widget, WindowDesc,
};

use druid::widget::{Container, Flex, Label, TextBox};

mod footer;
use footer::Footer;
mod tripane;

fn build_ui() -> impl Widget<()> {
    Container::new(
        Footer::new(
            Label::new("body"),
            Container::new(Label::new("footer")).background(druid::Color::BLACK),
        )
        .fill_body(true)
        .border(druid::Color::RED, 1.),
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .title("test")
            .with_min_size(Size::new(300., 300.)),
    )
    .delegate(QExist {})
    .launch(())
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
