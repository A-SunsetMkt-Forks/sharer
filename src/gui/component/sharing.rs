use iced::widget::{container, row, text_input, vertical_space};
use iced::Alignment::Center;
use iced::Length::{Fill, Shrink};
use iced::{clipboard, Command};
use iced_aw::TabLabel;

use crate::capture::capturer::Capturer;
use crate::column_iced;
use crate::gui::component::Component;
use crate::gui::theme::button;
use crate::gui::theme::button::{FilledButton, IconButton};
use crate::gui::theme::icon::Icon;
use crate::gui::theme::tab::Tab;
use crate::gui::theme::text::text;
use crate::gui::theme::widget::Element;
use crate::gui::theme::widget::Tabs;
use crate::gui::{app, resource};

pub struct SharingPage {
    current_tab: usize,
    invite_tab: InviteTab,
    viewers_tab: ViewersTab,
}

impl SharingPage {
    pub fn new() -> Self {
        Self {
            current_tab: Default::default(),
            invite_tab: InviteTab {},
            viewers_tab: ViewersTab {},
        }
    }
}

pub struct InviteTab {}

pub struct ViewersTab {}

pub struct UpdateProps<'a> {
    pub capturer: &'a mut Capturer,
}

#[derive(Clone, Debug)]
pub struct ViewProps {
    pub room_id: String,
    pub room_password: String,
    pub invite_link: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    Stop,
    CopyRoomID,
    CopyPasscode,
    CopyInviteLink,
    ChangeTab(usize),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::Sharing(message)
    }
}

impl<'a> Component<'a> for SharingPage {
    type Message = Message;
    type UpdateProps = UpdateProps<'a>;
    type ViewProps = ViewProps;

    fn update(
        &mut self,
        message: Self::Message,
        props: Self::UpdateProps,
    ) -> Command<app::Message> {
        match message {
            Message::CopyInviteLink => {
                if let Some(invite_link) = props.capturer.get_invite_link() {
                    return clipboard::write(invite_link);
                }
            }
            Message::CopyRoomID => {
                if let Some(room_id) = props.capturer.get_room_id() {
                    return clipboard::write(room_id);
                }
            }
            Message::CopyPasscode => {
                if let Some(room_password) = props.capturer.get_room_password() {
                    return clipboard::write(room_password);
                }
            }
            Message::Stop => {
                props.capturer.shutdown();
            }
            Message::ChangeTab(tab) => {
                self.current_tab = tab;
            }
        }
        Command::none()
    }

    fn view(&self, props: Self::ViewProps) -> Element<'_, app::Message> {
        column_iced![
            Tabs::new(self.current_tab, move |message| app::Message::Sharing(
                Message::ChangeTab(message)
            ))
            .push(
                self.invite_tab.tab_label(),
                self.invite_tab.view(props.clone())
            )
            .push(self.viewers_tab.tab_label(), self.viewers_tab.view(props))
            .tab_bar_style(Default::default())
            .icon_font(resource::font::ICON)
            .text_font(resource::font::BARLOW)
            .tab_bar_position(iced_aw::TabBarPosition::Top)
            .height(Shrink),
            vertical_space(Fill),
            action_bar(),
        ]
        .align_items(Center)
        .width(Fill)
        .height(Fill)
        .into()
    }
}

fn action_bar<'a>() -> Element<'a, app::Message> {
    row![FilledButton::new("End")
        .icon(Icon::StopCircle)
        .style(button::Style::Danger)
        .build()
        .on_press(Message::Stop.into()),]
    .padding(16)
    .into()
}

impl InviteTab {
    fn invite_info_card<'a>(
        head: &str,
        body: &str,
        on_copy: app::Message,
        width: f32,
    ) -> Element<'a, app::Message> {
        container(
            row![
                column_iced![
                    text(head).size(14).width(iced::Length::Shrink),
                    vertical_space(6),
                    text_input("", body)
                        .style(crate::gui::theme::text_input::Style::Selectable)
                        .size(18)
                        .font(resource::font::BARLOW)
                        .on_input(move |_| { app::Message::Ignore })
                        .width(iced::Length::Fill)
                        .padding(0)
                ]
                .width(iced::Length::Fixed(width - 72.)),
                IconButton::new(Icon::ContentCopy).build().on_press(on_copy)
            ]
            .align_items(Center)
            .spacing(8)
            .padding([16, 8, 16, 16]),
        )
        .style(crate::gui::theme::container::Style::OutlinedCard)
        .width(width)
        .into()
    }
}

impl Tab for InviteTab {
    type Message = app::Message;
    type Props = ViewProps;

    fn title(&self) -> String {
        String::from("Invite")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Link.into(), self.title())
    }

    fn content(&self, props: Self::Props) -> Element<'_, app::Message> {
        column_iced![
            row![
                Self::invite_info_card(
                    "Room",
                    props.room_id.as_str(),
                    Message::CopyRoomID.into(),
                    156.,
                ),
                Self::invite_info_card(
                    "Passcode",
                    props.room_password.as_str(),
                    Message::CopyPasscode.into(),
                    156.,
                ),
            ]
            .spacing(12),
            Self::invite_info_card(
                "Invite Link",
                props.invite_link.as_str(),
                Message::CopyInviteLink.into(),
                324.,
            )
        ]
        .width(Shrink)
        .height(Fill)
        .align_items(Center)
        .spacing(12)
        .into()
    }
}

impl Tab for ViewersTab {
    type Message = app::Message;
    type Props = ViewProps;

    fn title(&self) -> String {
        String::from("Viewers")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Group.into(), self.title())
    }

    fn content(&self, _props: Self::Props) -> Element<'_, app::Message> {
        column_iced![].into()
    }
}
