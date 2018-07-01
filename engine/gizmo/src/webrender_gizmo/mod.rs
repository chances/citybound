use kay::{ActorSystem, External, World, Actor};
use monet::{Display};
#[allow(unused_imports)]
use monet::glium::glutin::{Event, WindowEvent, MouseScrollDelta, ElementState, MouseButton};
use winit::EventsLoopProxy;

use webrender;
use webrender::api::*;
use euclid::TypedScale;

use notifier::Notifier;

#[derive(Compact, Clone)]
pub struct Gizmo {
    id: GizmoID,
    inner: External<GizmoInner>,
}

pub struct GizmoInner {
    window: Display,
    events_proxy: EventsLoopProxy,
    parked_frame: Option<Box<Frame>>,
    renderer: Option<webrender::Renderer>,
    api: Option<RenderApi>,
    device_pixel_ratio: f32,
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
        events_proxy: &External<EventsLoopProxy>,
        world: &mut World,
    ) -> Gizmo {
        Gizmo {
            id,
            inner: External::new(GizmoInner {
                window: *window.steal().into_box(),
                events_proxy: *events_proxy.steal().into_box(),
                parked_frame: None,
                renderer: None,
                api: None,
                device_pixel_ratio: 0.0,
                epoch: Epoch(0),
            }),
        }
    }

    fn init(&self) {
        let mut frame = self.parked_frame.expect("Should have parked target");

        println!("Loading webrender shaders...");
        let pixel_ratio = self.device_pixel_ratio;
        let opts = webrender::RendererOptions {
            resource_override_path: None,
            precache_shaders: false,
            device_pixel_ratio: pixel_ratio,
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
        let context = self.window.context.clone();
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

        let api = self.api.unwrap();

        let document_id = api.add_document(framebuffer_size, 0);
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
        api.send_transaction(document_id, txn);

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
