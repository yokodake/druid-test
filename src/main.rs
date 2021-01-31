use druid::{
    AppDelegate, AppLauncher, Code, Command, Data, Event, LocalizedString, PlatformError, Size,
    Target, Widget, WindowDesc,
};

use druid::widget::{Container, Flex, Label, TextBox};

mod footer;
use footer::FooterView;

fn build_ui() -> impl Widget<()> {
    Container::new(
        FooterView::new(Label::new("body"), Label::new("footer"))
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
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: Event,
        data: &mut T,
        env: &druid::Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyUp(k) => {
                println!("{}", k.code);
                if k.code == Code::KeyQ {
                    Some(Event::Command(Command::new(
                        druid::commands::CLOSE_ALL_WINDOWS,
                        (),
                        Target::Auto,
                    )))
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        }
    }

    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut T,
        env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &druid::Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &druid::Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }
}
