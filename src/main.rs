use druid::{
    AppDelegate, AppLauncher, Code, Data, Event,
    PlatformError, Size, Widget, WindowDesc,
};
use druid::widget::{Container, Flex, Label, Split};

use yukari_widgets::Footer;

fn build_ui<T: Data>() -> impl Widget<T> {
    Container::new (
        Footer::new
            ( Split::columns(Label::new("body1"), Label::new("body2"))
                .draggable(true)
                .solid_bar(true)
                .min_size(150., 300.)
            , Flex::row()
                .with_flex_child
                    ( Label::new("footer asdkfjklasdjf aslf asfjaklsdjf a;sdj fasdjflkasj flas")
                    , 2.
                    )
                .must_fill_main_axis(false),
            )
            .fill_body(true)
            .border(druid::Color::RED, 1.),
        )
}

trait OptionExt<T> {
    unsafe fn unwrap_unchecked(self) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    unsafe fn unwrap_unchecked(self) -> T {
        self.unwrap_or_else(|| std::hint::unreachable_unchecked())
    }
}
fn main() -> Result<(), PlatformError> {
    fn update_title(t: &String, _: &druid::Env) -> String {
        t.clone()
    }
    AppLauncher::with_window
        ( WindowDesc::new(build_ui())
            .title(update_title)
            .with_min_size(Size::new(300., 300.))
        )
        .delegate(QExist {})
        .launch("one".into())
}

struct QExist {}

impl AppDelegate<String> for QExist {
    fn event ( &mut self
             , _ctx: &mut druid::DelegateCtx
             , _window_id: druid::WindowId
             , event: Event
             , data: &mut String
             , _env: &druid::Env
             ) -> Option<Event> {
        match &event {
            Event::KeyUp(k) => {
                println!("{}", k.code);
                if k.code == Code::KeyQ {
                    std::process::exit(1)
                } else if k.code == Code::KeyW {
                    *data = String::from("two");
                    None
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        }
    }

    fn command ( &mut self
               , _ctx: &mut druid::DelegateCtx
               , _target: druid::Target
               , _cmd: &druid::Command
               , _data: &mut String
               , _env: &druid::Env
               ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added ( &mut self
                    , _id: druid::WindowId
                    , _data: &mut String
                    , _env: &druid::Env
                    , _ctx: &mut druid::DelegateCtx
                    ) {}

    fn window_removed ( &mut self
                      , _id: druid::WindowId
                      , _data: &mut String
                      , _env: &druid::Env
                      , _ctx: &mut druid::DelegateCtx
                      ) {}
}
