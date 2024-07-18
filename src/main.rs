use skia_safe::{
    gpu::{
        self, backend_render_targets, direct_contexts, surfaces::wrap_backend_render_target,
        DirectContext,
    },
    FontMgr,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, VirtualKeyCode},
};

const FERRIS: &str = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100%" height="100%" viewBox="0 0 1200 800" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:1.41421;">
    <g id="Layer-1" serif:id="Layer 1">
        <g transform="matrix(1,0,0,1,597.344,637.02)">
            <path d="M0,-279.559C-121.238,-279.559 -231.39,-264.983 -312.939,-241.23L-312.939,-38.329C-231.39,-14.575 -121.238,0 0,0C138.76,0 262.987,-19.092 346.431,-49.186L346.431,-230.37C262.987,-260.465 138.76,-279.559 0,-279.559" style="fill:rgb(165,43,0);fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,1068.75,575.642)">
            <path d="M0,-53.32L-14.211,-82.761C-14.138,-83.879 -14.08,-84.998 -14.08,-86.121C-14.08,-119.496 -48.786,-150.256 -107.177,-174.883L-107.177,2.643C-79.932,-8.849 -57.829,-21.674 -42.021,-35.482C-46.673,-16.775 -62.585,21.071 -75.271,47.686C-96.121,85.752 -103.671,118.889 -102.703,120.53C-102.086,121.563 -94.973,110.59 -84.484,92.809C-60.074,58.028 -13.82,-8.373 -4.575,-25.287C5.897,-44.461 0,-53.32 0,-53.32" style="fill:rgb(165,43,0);fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,149.064,591.421)">
            <path d="M0,-99.954C0,-93.526 1.293,-87.194 3.788,-80.985L-4.723,-65.835C-4.723,-65.835 -11.541,-56.989 0.465,-38.327C11.055,-21.872 64.1,42.54 92.097,76.271C104.123,93.564 112.276,104.216 112.99,103.187C114.114,101.554 105.514,69.087 81.631,32.046C70.487,12.151 57.177,-14.206 49.189,-33.675C71.492,-19.559 100.672,-6.755 135.341,4.265L135.341,-204.17C51.797,-177.622 0,-140.737 0,-99.954" style="fill:rgb(165,43,0);fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(-65.8097,-752.207,-752.207,65.8097,621.707,796.312)">
            <path d="M0.991,-0.034L0.933,0.008C0.933,0.014 0.933,0.02 0.933,0.026L0.99,0.069C0.996,0.073 0.999,0.08 0.998,0.087C0.997,0.094 0.992,0.1 0.986,0.103L0.92,0.133C0.919,0.139 0.918,0.145 0.916,0.15L0.964,0.203C0.968,0.208 0.97,0.216 0.968,0.222C0.965,0.229 0.96,0.234 0.953,0.236L0.882,0.254C0.88,0.259 0.877,0.264 0.875,0.27L0.91,0.33C0.914,0.336 0.914,0.344 0.91,0.35C0.907,0.356 0.9,0.36 0.893,0.361L0.82,0.365C0.817,0.369 0.813,0.374 0.81,0.379L0.832,0.445C0.835,0.452 0.833,0.459 0.828,0.465C0.824,0.47 0.816,0.473 0.809,0.472L0.737,0.462C0.733,0.466 0.729,0.47 0.724,0.474L0.733,0.544C0.734,0.551 0.731,0.558 0.725,0.562C0.719,0.566 0.711,0.568 0.704,0.565L0.636,0.542C0.631,0.546 0.626,0.549 0.621,0.552L0.615,0.621C0.615,0.629 0.61,0.635 0.604,0.638C0.597,0.641 0.589,0.641 0.583,0.638L0.521,0.602C0.52,0.603 0.519,0.603 0.518,0.603L0.406,0.729C0.406,0.729 0.394,0.747 0.359,0.725C0.329,0.705 0.206,0.599 0.141,0.543C0.109,0.52 0.089,0.504 0.09,0.502C0.093,0.499 0.149,0.509 0.217,0.554C0.278,0.588 0.371,0.631 0.38,0.619C0.38,0.619 0.396,0.604 0.406,0.575C0.406,0.575 0.406,0.575 0.406,0.575C0.407,0.576 0.407,0.576 0.406,0.575C0.406,0.575 0.091,0.024 0.305,-0.531C0.311,-0.593 0.275,-0.627 0.275,-0.627C0.266,-0.639 0.178,-0.598 0.12,-0.566C0.055,-0.523 0.002,-0.513 0,-0.516C-0.001,-0.518 0.018,-0.533 0.049,-0.555C0.11,-0.608 0.227,-0.707 0.256,-0.726C0.289,-0.748 0.301,-0.73 0.301,-0.73L0.402,-0.615C0.406,-0.614 0.41,-0.613 0.415,-0.613L0.47,-0.658C0.475,-0.663 0.483,-0.664 0.49,-0.662C0.497,-0.66 0.502,-0.655 0.504,-0.648L0.522,-0.58C0.527,-0.578 0.533,-0.576 0.538,-0.574L0.602,-0.608C0.608,-0.612 0.616,-0.612 0.623,-0.608C0.629,-0.605 0.633,-0.599 0.633,-0.592L0.637,-0.522C0.642,-0.519 0.647,-0.515 0.652,-0.512L0.721,-0.534C0.728,-0.536 0.736,-0.535 0.741,-0.531C0.747,-0.526 0.75,-0.519 0.749,-0.512L0.738,-0.443C0.742,-0.439 0.746,-0.435 0.751,-0.431L0.823,-0.439C0.83,-0.44 0.837,-0.437 0.842,-0.432C0.847,-0.426 0.848,-0.419 0.845,-0.412L0.821,-0.347C0.824,-0.342 0.828,-0.337 0.831,-0.332L0.903,-0.327C0.911,-0.327 0.917,-0.322 0.92,-0.316C0.924,-0.31 0.924,-0.302 0.92,-0.296L0.883,-0.236C0.885,-0.231 0.887,-0.226 0.889,-0.22L0.959,-0.202C0.966,-0.2 0.972,-0.195 0.974,-0.188C0.976,-0.181 0.974,-0.174 0.969,-0.168L0.92,-0.116C0.921,-0.111 0.923,-0.105 0.924,-0.099L0.988,-0.068C0.995,-0.065 0.999,-0.059 1,-0.052C1.001,-0.045 0.997,-0.038 0.991,-0.034ZM0.406,0.575C0.406,0.575 0.406,0.575 0.406,0.575C0.406,0.575 0.406,0.575 0.406,0.575Z" style="fill:url(#_Linear1);fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,450.328,483.629)">
            <path d="M0,167.33C-1.664,165.91 -2.536,165.068 -2.536,165.068L140.006,153.391C23.733,0 -69.418,122.193 -79.333,135.855L-79.333,167.33L0,167.33Z" style="fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,747.12,477.333)">
            <path d="M0,171.974C1.663,170.554 2.536,169.71 2.536,169.71L-134.448,159.687C-18.12,0 69.421,126.835 79.335,140.497L79.335,171.974L0,171.974Z" style="fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(-1.53e-05,-267.211,-267.211,1.53e-05,809.465,764.23)">
            <path d="M1,-0.586C1,-0.586 0.768,-0.528 0.524,-0.165L0.5,-0.064C0.5,-0.064 1.1,0.265 0.424,0.731C0.424,0.731 0.508,0.586 0.405,0.197C0.405,0.197 0.131,0.376 0.14,0.736C0.14,0.736 -0.275,0.391 0.324,-0.135C0.324,-0.135 0.539,-0.691 1,-0.736L1,-0.586Z" style="fill:url(#_Linear2);fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,677.392,509.61)">
            <path d="M0,-92.063C0,-92.063 43.486,-139.678 86.974,-92.063C86.974,-92.063 121.144,-28.571 86.974,3.171C86.974,3.171 31.062,47.615 0,3.171C0,3.171 -37.275,-31.75 0,-92.063" style="fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,727.738,435.209)">
            <path d="M0,0.002C0,18.543 -10.93,33.574 -24.408,33.574C-37.885,33.574 -48.814,18.543 -48.814,0.002C-48.814,-18.539 -37.885,-33.572 -24.408,-33.572C-10.93,-33.572 0,-18.539 0,0.002" style="fill:white;fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,483.3,502.984)">
            <path d="M0,-98.439C0,-98.439 74.596,-131.467 94.956,-57.748C94.956,-57.748 116.283,28.178 33.697,33.028C33.697,33.028 -71.613,12.745 0,-98.439" style="fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(1,0,0,1,520.766,436.428)">
            <path d="M0,0C0,19.119 -11.27,34.627 -25.173,34.627C-39.071,34.627 -50.344,19.119 -50.344,0C-50.344,-19.124 -39.071,-34.627 -25.173,-34.627C-11.27,-34.627 0,-19.124 0,0" style="fill:white;fill-rule:nonzero;"/>
        </g>
        <g transform="matrix(-1.53e-05,-239.021,-239.021,1.53e-05,402.161,775.388)">
            <path d="M0.367,0.129C-0.364,-0.441 0.223,-0.711 0.223,-0.711C0.259,-0.391 0.472,-0.164 0.472,-0.164C0.521,-0.548 0.525,-0.77 0.525,-0.77C1.203,-0.256 0.589,0.161 0.589,0.161C0.627,0.265 0.772,0.372 0.906,0.451L1,0.77C0.376,0.403 0.367,0.129 0.367,0.129Z" style="fill:url(#_Linear3);fill-rule:nonzero;"/>
        </g>
    </g>
    <defs>
        <linearGradient id="_Linear1" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(1,0,1.38778e-17,-1,0,-0.000650515)"><stop offset="0" style="stop-color:rgb(247,76,0);stop-opacity:1"/><stop offset="0.33" style="stop-color:rgb(247,76,0);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(244,150,0);stop-opacity:1"/></linearGradient>
        <linearGradient id="_Linear2" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(1,0,0,-1,0,1.23438e-06)"><stop offset="0" style="stop-color:rgb(204,58,0);stop-opacity:1"/><stop offset="0.15" style="stop-color:rgb(204,58,0);stop-opacity:1"/><stop offset="0.74" style="stop-color:rgb(247,76,0);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(247,76,0);stop-opacity:1"/></linearGradient>
        <linearGradient id="_Linear3" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(1,1.32349e-23,1.32349e-23,-1,0,-9.1568e-07)"><stop offset="0" style="stop-color:rgb(204,58,0);stop-opacity:1"/><stop offset="0.15" style="stop-color:rgb(204,58,0);stop-opacity:1"/><stop offset="0.74" style="stop-color:rgb(247,76,0);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(247,76,0);stop-opacity:1"/></linearGradient>
    </defs>
</svg>
"#;

mod renderer {
    #![allow(clippy::unusual_byte_groupings)]
    use skia_safe::{svg, FontMgr};

    use crate::FERRIS;

    pub fn render_frame(canvas: &skia_safe::canvas::Canvas, font_mgr: impl Into<FontMgr>) {
        let mut svg_dom = svg::Dom::from_str(&FERRIS.trim(), font_mgr).unwrap();
        canvas.save();
        svg_dom.set_container_size((400., 400.0));
        svg_dom.render(canvas);
        canvas.restore();
    }
}
fn main() {
    use gl::types::*;
    use glutin::{
        config::{ConfigTemplateBuilder, GlConfig},
        context::{
            ContextApi, ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor,
            PossiblyCurrentContext,
        },
        display::{GetGlDisplay, GlDisplay},
        prelude::GlSurface,
        surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
    };
    use glutin_winit::DisplayBuilder;
    use raw_window_handle::HasRawWindowHandle;

    use std::{
        ffi::CString,
        num::NonZeroU32,
        time::{Duration, Instant},
    };

    use winit::{
        event::{Event, KeyboardInput, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    };

    use skia_safe::{
        gpu::{gl::FramebufferInfo, SurfaceOrigin},
        Color, ColorType, Surface,
    };

    let font_mgr = FontMgr::default();

    let el = EventLoop::new();
    let winit_window_builder = WindowBuilder::new()
        .with_title("rust-skia-gl-window")
        .with_inner_size(PhysicalSize::new(400, 600));

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));
    let (window, gl_config) = display_builder
        .build(&el, template, |configs| {
            // Find the config with the maximum number of samples, so our display will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let mut window = window.expect("Could not create window with OpenGL context");
    let raw_window_handle = window.raw_window_handle();

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };

    let (width, height): (u32, u32) = window.inner_size().into();

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Could not create gl window surface")
    };

    let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current when setting up skia renderer");

    gl::load_with(|s| {
        gl_config
            .display()
            .get_proc_address(CString::new(s).unwrap().as_c_str())
    });
    let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
        if name == "eglGetCurrentDisplay" {
            return std::ptr::null();
        }
        gl_config
            .display()
            .get_proc_address(CString::new(name).unwrap().as_c_str())
    })
    .expect("Could not create interface");

    let mut gr_context =
        direct_contexts::make_gl(interface, None).expect("Could not create direct context");

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            protected: gpu::Protected::No,
        }
    };

    /// Create the surface for Skia to render in
    pub fn create_surface(
        window: &mut Window,
        fb_info: FramebufferInfo,
        gr_context: &mut DirectContext,
        num_samples: usize,
        stencil_size: usize,
    ) -> Surface {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target =
            backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);
        wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create skia surface")
    }

    let num_samples = 0; //gl_config.num_samples() as usize;
    let stencil_size = gl_config.stencil_size() as usize;

    let surface = create_surface(
        &mut window,
        fb_info,
        &mut gr_context,
        num_samples,
        stencil_size,
    );

    // Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
    // `DirectContext`.
    //
    // https://github.com/rust-skia/rust-skia/issues/476
    struct Env {
        surface: Surface,
        gl_surface: GlutinSurface<WindowSurface>,
        gr_context: skia_safe::gpu::DirectContext,
        gl_context: PossiblyCurrentContext,
        window: Window,
    }

    let mut env = Env {
        surface,
        gl_surface,
        gl_context,
        gr_context,
        window,
    };
    let mut previous_frame_start = Instant::now();

    el.run(move |event, _, control_flow| {
        let frame_start = Instant::now();
        let mut draw_frame = false;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::Resized(physical_size) => {
                    env.surface = create_surface(
                        &mut env.window,
                        fb_info,
                        &mut env.gr_context,
                        num_samples,
                        stencil_size,
                    );
                    /* First resize the opengl drawable */
                    let (width, height): (u32, u32) = physical_size.into();
                    env.gl_surface.resize(
                        &env.gl_context,
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    );
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => {
                    std::thread::spawn(|| {
                        println!("Picking file...");

                        let path = rfd::FileDialog::new().pick_file();

                        println!("{path:?}");
                    });
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                draw_frame = true;
            }
            _ => (),
        }
        let expected_frame_length_seconds = 1.0 / 20.0;
        let frame_duration = Duration::from_secs_f32(expected_frame_length_seconds);

        if frame_start - previous_frame_start > frame_duration {
            draw_frame = true;
            previous_frame_start = frame_start;
        }
        if draw_frame {
            let canvas = env.surface.canvas();
            canvas.clear(Color::WHITE);
            renderer::render_frame(&canvas, &font_mgr);
            env.gr_context.flush_and_submit();
            env.gl_surface.swap_buffers(&env.gl_context).unwrap();
        }

        *control_flow = ControlFlow::WaitUntil(previous_frame_start + frame_duration)
    });
}
