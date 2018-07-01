use winit::EventsLoopProxy;

use webrender::api::{DocumentId, RenderNotifier};

#[derive(Clone)]
pub struct Notifier {
    events_proxy: EventsLoopProxy,
}

impl Notifier {
    pub fn new(events_proxy: EventsLoopProxy) -> Notifier {
        Notifier { events_proxy }
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        Box::new(Notifier {
            events_proxy: self.events_proxy.clone(),
        })
    }

    fn wake_up(&self) {
        let _ = self.events_proxy.wakeup();
    }

    fn new_frame_ready(&self, _: DocumentId, _scrolled: bool, _composite_needed: bool) {
        self.wake_up();
    }
}
