use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    stream::Stream,
    task::{Context, Poll},
};
use gloo_events::{EventListener, EventListenerOptions};
use std::pin::Pin;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent, TouchEvent};

pub enum Event {
    TouchStart(TouchEvent),
    TouchMove(TouchEvent),
    TouchEnd(TouchEvent),
    TouchCancel(TouchEvent),

    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
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
    use Event::*;

    EventListener::new_with_options(
        target,
        event_type,
        EventListenerOptions::enable_prevent_default(),
        move |event| {
            // タッチ、マウスによる選択、スクロールなどを止める
            event.prevent_default();

            let event = if event_type.starts_with("touch") {
                let event = event.clone().dyn_into().unwrap();
                match event_type {
                    "touchstart" => TouchStart(event),
                    "touchmove" => TouchMove(event),
                    "touchend" => TouchEnd(event),
                    "touchcancel" => TouchCancel(event),
                    _ => unreachable!(),
                }
            } else if event_type.starts_with("mouse") {
                let event = event.clone().dyn_into().unwrap();
                match event_type {
                    "mouseenter" => MouseEnter(event),
                    "mouseleave" => MouseLeave(event),
                    "mousemove" => MouseMove(event),
                    "mousedown" => MouseDown(event),
                    "mouseup" => MouseUp(event),
                    _ => unreachable!(),
                }
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
