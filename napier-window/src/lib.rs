use futures::{
    channel::mpsc::{channel, Receiver},
    stream::Stream,
    task::{Context, Poll},
};
use gloo_events::EventListener;
use std::{borrow::Cow, pin::Pin};
use web_sys::{Event, EventTarget};

#[pin_project::pin_project]
pub struct EventStream {
    #[pin]
    receiver: Receiver<Event>,
    listener: EventListener,
}

impl EventStream {
    pub fn listen<S>(target: &EventTarget, event_type: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        EventStream::listen_with_buffer(target, event_type, 64)
    }

    pub fn listen_with_buffer<S>(target: &EventTarget, event_type: S, buffer: usize) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let (mut sender, receiver) = channel(buffer);

        let listener = EventListener::new(target, event_type, move |event| {
            if let Err(_) = sender.try_send(event.clone()) {
                log::info!("EventStream buffer is full");
            }
        });

        EventStream { receiver, listener }
    }
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
