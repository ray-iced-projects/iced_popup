#!// popup

use iced::time::{self, milliseconds};

use iced::widget::{button, checkbox, space, text, toggler};
use iced::widget::{column, container};
use iced::{Element, Fill, Subscription, Theme};
use iced::widget::Id;

use crate::popup::Popup;

mod popup;


pub fn main() -> iced::Result {

    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .theme(Theme::Dark)
        .run()
}

#[derive(Debug, Default, Clone)]
struct App {
    opened_with_btn1: bool,
    opened_with_btn2: bool,
    opened_with_chk: bool,
    opened_with_tog: bool,
    opened_with_timer: bool,

    is_checked: bool,
    is_togged: bool,

    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    Button1Pressed,
    Button2Pressed,
    CheckboxChecked(bool),
    TogglerTogged(bool),

    PopupOpened,
    ClickedOutside(Option<Id>),

    Tick,
    Timer,
}

#[derive(Clone, Debug, Default)]
enum State {
    #[default]
    Idle,
    Ticking,
}


impl App {
    
    fn update(&mut self, message: Message) {
        match message {
            Message::Button1Pressed => {
                self.opened_with_btn1 = !self.opened_with_btn1
            },
            Message::Button2Pressed => {
                self.opened_with_btn2 = !self.opened_with_btn2
            },
            Message::CheckboxChecked(open) => {
                self.opened_with_chk = open;
                self.is_checked = open;
            },
            Message::TogglerTogged(open) => {
                self.opened_with_tog = open;
                self.is_togged = open;
            },
            Message::PopupOpened => {
                dbg!("Popup Opened");
            },
            Message::ClickedOutside(id) => {
                if id.as_ref() == Some(&Id::new("btn_popup_2")) {
                    // Only close the inner popup; leave the outer open
                    self.opened_with_btn2 = false;
                } else {
                    self.opened_with_btn1 = false;
                    self.opened_with_btn2 = false;
                    self.opened_with_chk = false;
                    self.opened_with_tog = false;
                    self.opened_with_timer = false;
                    self.is_checked = false;
                    self.is_togged = false;
                }
            },
            Message::Tick => {
                self.opened_with_timer = !self.opened_with_timer;
            }
            Message::Timer => match self.state {
                State::Idle => {
                    self.state = State::Ticking;
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {

        let tick = match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => time::every(milliseconds(1000)).map(|_| Message::Tick),
        };


        Subscription::batch(vec![tick])
    }

    fn view(&self) -> Element<'_, Message> {

        let timer_btn: Element<Message> = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Timer).into()
        };

        let popup_btn = 
            from_btn(self.opened_with_btn1, self.opened_with_btn2);

        let popup_chk = 
            from_chk(self.is_checked, self.opened_with_chk);

        let popup_tog = 
            from_tog(self.is_togged, self.opened_with_tog);

        let popup_timer = 
            from_timer(self.opened_with_timer);

        let col = 
            column([
                timer_btn, 
                popup_btn, 
                popup_chk, 
                popup_tog, 
                popup_timer])
                .spacing(20.0);
        
        container(col)
            .center(Fill)
            .into()
    }

}


// from button — demonstrates a popup nested inside another popup
fn from_btn<'a>(opened1: bool, opened2: bool) -> Element<'a, Message> {

    // Inner popup: lives inside the outer popup's content
    let inner_btn = button("Open nested popup")
        .on_press(Message::Button2Pressed);

    let inner_content =
        container(text("I'm a nested Popup!"))
            .width(180.0)
            .height(80.0)
            .style(|theme| container::rounded_box(theme));

    let inner_popup = Popup::new(inner_btn, inner_content, opened2)
        .id("btn_popup_2")
        .gap(50.0)
        .on_click_outside(Message::ClickedOutside)
        .position(popup::Position::Right);

    // Outer popup content contains the nested popup as a widget
    let col = column([
        text("I'm a Popup opened with a button").into(),
        inner_popup.into(),
    ]);

    let content =
        container(col)
            .width(220.0)
            .height(100.0)
            .style(|theme| container::rounded_box(theme));

    let btn = button("Open outer popup")
        .on_press(Message::Button1Pressed);

    Popup::new(btn, content, opened1)
        .id("btn_popup_1")
        .on_click_outside(Message::ClickedOutside)
        .on_open(|| Message::PopupOpened)
        .into()
}


// from checkbox
fn from_chk<'a>(is_checked: bool, opened: bool) -> Element<'a, Message> {
    let chk = 
            checkbox(is_checked)
                .label("Check me to open")
                .on_toggle(Message::CheckboxChecked);

    let content = 
    container(text("I'm a Popup from a Checkbox"))
        .width(200.0)
        .height(100.0)
        .style(move|theme| {
            container::rounded_box(theme)
        });

    Popup::new(chk, content, opened)
        .id("chk_popup")
        .on_click_outside(Message::ClickedOutside)
        .on_open(|| Message::PopupOpened)
        .position(popup::Position::Right)
        .gap(60.0)
        .into()
}

// from toggler
fn from_tog<'a>(is_togged: bool, opened: bool) -> Element<'a, Message> {
    let tog = 
            toggler(is_togged)
                .label("Tog me to open")
                .on_toggle(Message::TogglerTogged);

    let content = 
    container(text("I'm a Popup from a Toggler"))
        .width(200.0)
        .height(100.0)
        .style(move|theme| {
            container::rounded_box(theme)
        });

    Popup::new(tog, content, opened)
        .id("tog_popup")
        .on_click_outside(Message::ClickedOutside)
        .on_open(|| Message::PopupOpened)
        .position(popup::Position::Left)
        .gap(30.0)
        .into()
}

// from timer
fn from_timer<'a>(opened: bool) -> Element<'a, Message> {
    
    let sp = space();

    let content = 
    container(text("I'm a Popup from a Timer"))
        .width(200.0)
        .height(100.0)
        .style(move|theme| {
            container::rounded_box(theme)
        });

    Popup::new(sp, content, opened)
        .id("timer_popup")
        .on_click_outside(Message::ClickedOutside)
        .on_open(|| Message::PopupOpened)
        .position(popup::Position::Bottom)
        .gap(20.0)
        .into()
}
