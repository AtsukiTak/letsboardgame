use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    stream::Stream,
    task::{Context, Poll},
};
use gloo_events::{EventListener, EventListenerOptions};
use std::pin::Pin;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent as WebMouseEvent, TouchEvent as WebTouchEvent};

pub enum Event {
    Touch(TouchEvent),
    Mouse(MouseEvent),
}

pub enum TouchEvent {
    Start(WebTouchEvent),
    Move(WebTouchEvent),
    End(WebTouchEvent),
    Cancel(WebTouchEvent),
}

pub enum MouseEvent {
    Enter(WebMouseEvent),
    Leave(WebMouseEvent),
    Down(WebMouseEvent),
    Up(WebMouseEvent),
    Move(WebMouseEvent),
}

#[pin_project::pin_project]
pub struct EventStream {
    #[pin]
    receiver: Receiver<Event>,

    listeners: Vec<EventListener>,
}

impl EventStream {
    pub fn listen(target: &EventTarget) -> Self {
        let (sender, receiver) = channel(1024);

        let listeners = [
            "touchstart",
            "touchmove",
            "touchend",
            "touchcancel",
            "mouseenter",
            "mouseleave",
            "mousedown",
            "mouseup",
            "mousemove",
        ]
        .iter()
        .map(|event_type| listen_event(target, event_type, sender.clone()))
        .collect::<Vec<_>>();

        EventStream {
            receiver,
            listeners,
        }
    }
}

fn listen_event(
    target: &EventTarget,
    event_type: &'static str,
    mut sender: Sender<Event>,
) -> EventListener {
    EventListener::new_with_options(
        target,
        event_type,
        EventListenerOptions::enable_prevent_default(),
        move |event| {
            // タッチ、マウスによる選択、スクロールなどを止める
            event.prevent_default();

            let event = if event_type.starts_with("touch") {
                let event = event.clone().dyn_into().unwrap();
                let touch_event = match event_type {
                    "touchstart" => TouchEvent::Start(event),
                    "touchmove" => TouchEvent::Move(event),
                    "touchend" => TouchEvent::End(event),
                    "touchcancel" => TouchEvent::Cancel(event),
                    _ => unreachable!(),
                };
                Event::Touch(touch_event)
            } else if event_type.starts_with("mouse") {
                let event = event.clone().dyn_into().unwrap();
                let mouse_event = match event_type {
                    "mouseenter" => MouseEvent::Enter(event),
                    "mouseleave" => MouseEvent::Leave(event),
                    "mousemove" => MouseEvent::Move(event),
                    "mousedown" => MouseEvent::Down(event),
                    "mouseup" => MouseEvent::Up(event),
                    _ => unreachable!(),
                };
                Event::Mouse(mouse_event)
            } else {
                unreachable!();
            };

            if let Err(_) = sender.try_send(event) {
                log::info!("EventStream buffer is full. So any succeeding event will not sent until receiver consumes an event");
            }
        },
    )
}

impl Stream for EventStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.receiver.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.receiver.size_hint()
    }
}
