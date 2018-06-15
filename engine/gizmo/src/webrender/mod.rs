use webrender;
use webrender::api::*;

struct Notifier {
    events_proxy: winit::EventsLoopProxy,
}

impl Notifier {
    fn new(events_proxy: winit::EventsLoopProxy) -> Notifier {
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

struct Gizmo {
    window: winit::Window,
    events_proxy: winit::EventsLoopProxy,
    device_pixel_ratio: uint,
}

impl Gizmo {
    fn init(&self, &mut frame: ::monet::glium::Frame) {
        println!("Loading webrender shaders...");
        let pixel_ratio = self.device_pixel_ratio;
        let opts = webrender::RendererOptions {
            resource_override_path: None,
            precache_shaders: E::PRECACHE_SHADERS,
            pixel_ratio,
            clear_color: Some(ColorF::new(0.0, 0.0, 0.0, 0.0)),
            //scatter_gpu_cache_updates: false,
            debug_flags: webrender::DebugFlags::ECHO_DRIVER_MESSAGES,
            ..options.unwrap_or(webrender::RendererOptions::default())
        };

        let framebuffer_size = {
            let (width, height) = self.window.get_inner_size().unwrap();
            DeviceUintSize::new(width, height)
        };
        let notifier = Box::new(Notifier::new(self.events_proxy));
        let context = frame.context.clone();
        let (mut renderer, sender) = webrender::Renderer::new(context.gl, notifier, opts).unwrap();
        let api = sender.create_api();
        let document_id = api.add_document(framebuffer_size, 0);

        let epoch = Epoch(0);
        let pipeline_id = PipelineId(0, 0);
        let layout_size = framebuffer_size.to_f32() / euclid::TypedScale::new(pixel_ratio);
        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
        let mut txn = Transaction::new();

        // TODO: Put some geometry in the DisplayList builder

        txn.set_display_list(
            epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.set_root_pipeline(pipeline_id);
        txn.generate_frame();
        api.send_transaction(document_id, txn);

        // Ready for event loop
    }

    fn render(&self) {
        let mut txn = Transaction::new();
        let framebuffer_size = {
            let (width, height) = self.window.get_inner_size().unwrap();
            DeviceUintSize::new(width, height)
        };

        // TODO: Handle present mouse and keyboard state?

        let document_id = api.add_document(framebuffer_size, 0);
        let pipeline_id = PipelineId(0, 0);
        let layout_size = framebuffer_size.to_f32() / euclid::TypedScale::new(pixel_ratio);

        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

        // TODO: Add the already built DisplayList hierarchy

        txn.set_display_list(
            epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.generate_frame();
        api.send_transaction(document_id, txn);

        renderer.update();
        renderer.render(framebuffer_size).unwrap();
        let _ = renderer.flush_pipeline_info();
    }
}
