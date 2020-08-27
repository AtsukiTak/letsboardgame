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
}

#[pin_project::pin_project]
pub struct EventStream {
    #[pin]
    receiver: Receiver<Event>,
    listener_touch_start: EventListener,
    listener_touch_move: EventListener,
    listener_touch_end: EventListener,
    listener_touch_cancel: EventListener,
}

impl EventStream {
    pub fn listen(target: &EventTarget) -> Self {
        let (sender, receiver) = channel(1024);

        let listener_touch_start = listen_touch_event(target, "touchstart", sender.clone());
        let listener_touch_move = listen_touch_event(target, "touchmove", sender.clone());
        let listener_touch_end = listen_touch_event(target, "touchend", sender.clone());
        let listener_touch_cancel = listen_touch_event(target, "touchcancel", sender.clone());

        EventStream {
            receiver,
            listener_touch_start,
            listener_touch_move,
            listener_touch_end,
            listener_touch_cancel,
        }
    }
}

fn listen_touch_event(
    target: &EventTarget,
    event_type: &'static str,
    mut sender: Sender<Event>,
) -> EventListener {
    EventListener::new_with_options(
        target,
        event_type,
        EventListenerOptions::enable_prevent_default(),
        move |event| {
            // タッチによる選択、スクロールなどを止める
            event.prevent_default();

            let touch_event = event.clone().dyn_into::<TouchEvent>().unwrap();
            let event = match event_type {
                "touchstart" => Event::TouchStart(touch_event),
                "touchmove" => Event::TouchMove(touch_event),
                "touchend" => Event::TouchEnd(touch_event),
                "touchcancel" => Event::TouchCancel(touch_event),
                _ => unreachable!(),
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
