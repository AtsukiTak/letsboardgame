use crate::Canvas;
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    stream::Stream,
    task::{Context, Poll},
};
use gloo_events::{EventListener, EventListenerOptions};
use std::pin::Pin;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, TouchEvent};

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

pub struct MouseEvent {
    internal: web_sys::MouseEvent,
    target: Canvas,
}

impl MouseEvent {
    fn new(internal: web_sys::MouseEvent, target: &Canvas) -> Self {
        MouseEvent {
            internal,
            target: target.clone(),
        }
    }
    pub fn x(&self) -> f64 {
        let target_rect = self.target.as_element().get_bounding_client_rect();
        self.internal.client_x() as f64 - target_rect.x()
    }

    pub fn y(&self) -> f64 {
        let target_rect = self.target.as_element().get_bounding_client_rect();
        self.internal.client_y() as f64 - target_rect.y()
    }
}

#[pin_project::pin_project]
pub struct EventStream {
    #[pin]
    receiver: Receiver<Event>,

    listeners: Vec<EventListener>,
}

impl EventStream {
    pub fn listen(target: &Canvas) -> Self {
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
    target: &Canvas,
    event_type: &'static str,
    mut sender: Sender<Event>,
) -> EventListener {
    use Event::*;

    let event_target = target.as_element().dyn_ref::<EventTarget>().unwrap();
    let target = target.clone();

    EventListener::new_with_options(
        event_target,
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
                let event = MouseEvent::new(event.clone().dyn_into().unwrap(), &target);
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
