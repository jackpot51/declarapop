use cosmic::{Action, Element, iced::Length, prelude::*, widget};

use crate::Config;

pub struct App {
    core: cosmic::Core,
    config: Option<Result<Config, String>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Config(Result<Config, String>),
}

impl cosmic::Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "dev.soller.declarapop";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, Task<Action<Message>>) {
        let app = App { core, config: None };
        let task = app.reload();
        (app, task)
    }

    fn view(&self) -> Element<'_, Message> {
        let element: Element<Message> = match &self.config {
            Some(Ok(config)) => {
                let mut apt_col = widget::column::with_capacity(config.apt_packages.len() + 1)
                    .width(Length::Fill);
                apt_col = apt_col.push(widget::text::title4("Manual Packages"));
                for p in config.apt_packages.iter() {
                    // Skip packages included in language packs
                    //TODO: change config format to make this easier?
                    if config
                        .languages
                        .iter()
                        .any(|(_, packages)| packages.iter().any(|x| x.0 == p.0))
                    {
                        continue;
                    }

                    apt_col = apt_col.push(widget::checkbox(true).label(&p.0));
                }

                let mut langs_col =
                    widget::column::with_capacity(config.languages.len() + 1).width(Length::Fill);
                langs_col = langs_col.push(widget::text::title4("Language Packages"));
                for (lang, packages) in config.languages.iter() {
                    let mut lang_col =
                        widget::column::with_capacity(packages.len() + 1).width(Length::Fill);
                    lang_col = lang_col.push(widget::text::heading(lang));
                    for p in packages.iter() {
                        lang_col = lang_col.push(widget::checkbox(true).label(&p.0));
                    }
                    langs_col = langs_col.push(lang_col);
                }

                let mut flat_col = widget::column::with_capacity(config.flatpak_packages.len() + 1)
                    .width(Length::Fill);
                flat_col = flat_col.push(widget::text::title4("Flatpak Apps"));
                for p in config.flatpak_packages.iter() {
                    flat_col = flat_col.push(widget::checkbox(true).label(&p.0));
                }
                widget::row!(apt_col, langs_col, flat_col).into()
            }
            Some(Err(err)) => widget::text(err).into(),
            None => widget::text("Loading...").into(),
        };
        widget::scrollable(widget::container(element).width(Length::Fill)).into()
    }

    fn update(&mut self, message: Message) -> Task<Action<Message>> {
        match message {
            Message::Config(config) => self.config = Some(config),
        }
        Task::none()
    }
}

impl App {
    fn reload(&self) -> Task<Action<Message>> {
        Task::perform(
            async {
                tokio::task::spawn_blocking(|| {
                    Action::App(Message::Config(
                        Config::current().map_err(|err| format!("Failed to load config: {}", err)),
                    ))
                })
                .await
                .unwrap()
            },
            |x| x,
        )
    }
}
