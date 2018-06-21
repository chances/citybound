use kay::{ActorSystem, External, World, Actor};
use monet::{Display};
use monet::glium::glutin::{Event, WindowEvent, MouseScrollDelta, ElementState, MouseButton};
use monet::glium::glutin::EventsLoopProxy;

use webrender;
use webrender::api::*;
use euclid::TypedScale;

struct Notifier {
    events_proxy: EventsLoopProxy,
}

impl Notifier {
    fn new(events_proxy: EventsLoopProxy) -> Notifier {
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

#[derive(Compact, Clone)]
pub struct Gizmo {
    id: GizmoID,
    inner: External<GizmoInner>,
}

pub struct GizmoInner {
    window: Display,
    events_proxy: EventsLoopProxy,
    parked_frame: Option<Box<::monet::glium::Frame>>,
    renderer: Option<webrender::Renderer>,
    api: Option<RenderApi>,
    device_pixel_ratio: u32,
    epoch: Epoch,
}

impl ::std::ops::Deref for Gizmo {
    type Target = GizmoInner;

    fn deref(&self) -> &GizmoInner {
        &self.inner
    }
}

impl ::std::ops::DerefMut for Gizmo {
    fn deref_mut(&mut self) -> &mut GizmoInner {
        &mut self.inner
    }
}

impl Gizmo {
    pub fn spawn(
        id: GizmoID,
        window: &External<Display>,
        events_loop: &External<EventsLoopProxy>,
        world: &mut World,
    ) -> Gizmo {
        Gizmo {
            id,
            inner: External::new(GizmoInner {
                window: *window.steal().into_box(),
                events_loop: *events_loop.steal().into_box(),
                parked_frame: None,
                api: None,
                device_pixel_ratio: 0,
                epoch: Epoch(0),
            }),
        }
    }

    fn init(&self, &mut frame: ::monet::glium::Frame) {
        println!("Loading webrender shaders...");
        let pixel_ratio = self.device_pixel_ratio;
        let opts = webrender::RendererOptions {
            resource_override_path: None,
            precache_shaders: false,
            pixel_ratio,
            clear_color: Some(ColorF::new(0.0, 0.0, 0.0, 0.0)),
            //scatter_gpu_cache_updates: false,
            debug_flags: webrender::DebugFlags::ECHO_DRIVER_MESSAGES,
            ..webrender::RendererOptions::default()
        };

        let framebuffer_size = {
            let (width, height) = frame.dimensions;
            DeviceUintSize::new(width, height)
        };
        let notifier = Box::new(Notifier::new(self.events_proxy));
        let context = frame.context.clone();
        let (mut renderer, sender) = webrender::Renderer::new(context.gl, notifier, opts).unwrap();
        let api = sender.create_api();
        let document_id = api.add_document(framebuffer_size, 0);

        let pipeline_id = PipelineId(0, 0);
        let layout_size = framebuffer_size.to_f32() / TypedScale::new(pixel_ratio);
        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
        let mut txn = Transaction::new();

        // TODO: Put some geometry in the DisplayList builder

        txn.set_display_list(
            self.epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.set_root_pipeline(pipeline_id);
        txn.generate_frame();
        api.send_transaction(document_id, txn);

        // Ready for event loop

        self.renderer = Some(renderer);
        self.api = Some(api);
    }

    fn render(&self, &mut frame: ::monet::glium::Frame) {
        let mut txn = Transaction::new();
        let framebuffer_size = {
            let (width, height) = frame.dimensions;
            DeviceUintSize::new(width, height)
        };

        // TODO: Handle present mouse and keyboard state?

        let document_id = self.api.add_document(framebuffer_size, 0);
        let pipeline_id = PipelineId(0, 0);
        let layout_size = framebuffer_size.to_f32() / TypedScale::new(self.device_pixel_ratio);

        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

        // TODO: Add the already built DisplayList hierarchy

        txn.set_display_list(
            self.epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.generate_frame();
        self.api.send_transaction(document_id, txn);

        let renderer = self.renderer.unwrap();
        renderer.update();
        renderer.render(framebuffer_size).unwrap();
        let _ = renderer.flush_pipeline_info();
    }
}

use monet::{TargetProvider, TargetProviderID};
use monet::glium::Frame;

#[cfg_attr(feature = "cargo-clippy", allow(useless_format))]
impl TargetProvider for Gizmo {
    /// Critical
    fn submitted(&mut self, target: &External<Frame>, world: &mut World) {
        self.parked_frame = Some(target.steal().into_box());

        let size_pixels = self.window.gl_window().get_inner_size_pixels().unwrap();
        let device_pixel_ratio = self.window.gl_window().hidpi_factor();

        self.device_pixel_ratio = device_pixel_ratio;

        // TODO: Do I need to tell webrender to draw or wakeup here?
    }
}

mod kay_auto;
pub use self::kay_auto::*;
