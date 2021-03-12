use std::sync::Arc;

use druid::lens::{Constant, InArc, Identity, Then};
use druid::widget::{Container, Flex, Label, List, Split, LensWrap};
use druid::{
    AppDelegate, AppLauncher, Code, Data, Event, Lens, PlatformError, Size, Widget, WindowDesc
};

use yukari_widgets::Footer;

#[derive(Lens, Clone, Data)]
struct Test {
    #[data(same_fn = "PartialEq::eq")]
    header: String,
    #[data(same_fn = "PartialEq::eq")]
    footer: String,
    #[data(same_fn = "PartialEq::eq")]
    title: String,
    #[data(ignore)]
    child1: Arc<Vec<String>>,
    #[data(ignore)]
    child2: Arc<Vec<String>>,
}

fn build_header(body: impl Widget<Test> +'static) -> impl Widget<Test> {
    Flex::column()
        .with_flex_child(Label::dynamic(|data: &Test, _| data.header.clone()), 2.)
        .with_flex_child(body, 1.)
        .must_fill_main_axis(true)
}
fn build_footer(body: impl Widget<Test> +'static) -> impl Widget<Test> {
    Footer::new(
        body,
        Flex::row()
            .with_flex_child(Label::dynamic(|data :&Test, _| data.footer.clone()), 2.)
            .must_fill_main_axis(true),
    )
    .fill_body(true)
    .border(druid::Color::RED, 1.)
}
fn build_main() -> impl Widget<Test> {
    Split::columns(
        LensWrap::new(
            List::new(|| Label::new("left")),
            // Then::new(Constant(vec![()]), InArc::new(Identity)),
            Test::child1
        ),
        LensWrap::new(
            List::new(|| Label::new("right")),
            Test::child2
            // Then::new(Test::child2, InArc::new(Identity)),
            // Then::new(Constant(vec![()]), InArc::new(Identity)),
        ),
    )
    .draggable(true)
    .solid_bar(true)
    .min_size(150., 300.)
}

fn build_ui() -> impl Widget<Test> {
    Container::new(build_footer(build_main()))
}

fn main() -> Result<(), PlatformError> {
    fn update_title(t: &Test, _: &druid::Env) -> String {
        t.title.clone()
    }
    AppLauncher::with_window(
        WindowDesc::new(build_ui())
            .title(update_title)
            .with_min_size(Size::new(300., 300.)),
    )
    .delegate(QExist {})
    .launch(Test {
        header: "menu..".into(),
        footer: "footer".into(),
        title: "one".into(),
        child1: Arc::new(vec![
            "first".into(),
            "second".into(),
            "third".into(),
            "fourth".into(),
        ]),
        child2: Arc::new(vec!["first".into(), "second".into()]),
    })
}

struct QExist {}

impl AppDelegate<Test> for QExist {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: Event,
        data: &mut Test,
        _env: &druid::Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyUp(k) => {
                println!("{}", k.code);
                if k.code == Code::KeyQ {
                    std::process::exit(1)
                } else if k.code == Code::KeyW {
                    data.title = if data.title == "one" {
                        "two".into()
                    } else {
                        "one".into()
                    };
                    None
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
        _data: &mut Test,
        _env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut Test,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut Test,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
}
