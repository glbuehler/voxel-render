use std::{num::NonZero, sync::Arc};

use wgpu::{util::DeviceExt, TextureView};

use crate::{camera::Camera, vertex::Vertex};

pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    window: Arc<winit::window::Window>,

    uniform_bind_group: wgpu::BindGroup,

    camera: Camera,
    vertex_buf: wgpu::Buffer,
    proj_view_matrix_buf: wgpu::Buffer,
}

impl<'a> State<'a> {
    pub async fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let mut surface_config = surface.get_default_config(&adapter, 800, 600).unwrap();
        surface_config.present_mode = wgpu::PresentMode::Fifo;
        surface.configure(&device, &surface_config);

        let proj_view_matrix_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view matrix buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: std::mem::size_of::<cgmath::Matrix4<f32>>() as u64,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("view matrix uniform bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view matrix uniform"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: proj_view_matrix_buf.as_entire_binding(),
            }],
        });

        let shader_module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            depth_stencil: None,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("pipeline layout descriptor"),
                    bind_group_layouts: &[&uniform_bind_group_layout],
                    ..Default::default()
                }),
            ),
            multiview: None,
            multisample: Default::default(),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            vertex: wgpu::VertexState {
                buffers: &[wgpu::VertexBufferLayout {
                    step_mode: Default::default(),
                    attributes: &Vertex::attributes(),
                    array_stride: Vertex::stride(),
                }],
                compilation_options: Default::default(),
                entry_point: "vx_main",
                module: &shader_module,
            },
            fragment: Some(wgpu::FragmentState {
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: Default::default(),
                })],
                compilation_options: Default::default(),
                entry_point: "fg_main",
                module: &shader_module,
            }),
        });

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&[
                Vertex {
                    pos: [0.0, 0.5, -2.0],
                    col: [1.0, 0.0, 0.0],
                },
                Vertex {
                    pos: [0.5, -0.5, -2.0],
                    col: [0.0, 0.0, 1.0],
                },
                Vertex {
                    pos: [-0.5, -0.5, -2.0],
                    col: [0.0, 1.0, 0.0],
                },
            ]),
        });

        Self {
            camera: Camera::new(surface_config.width as f32 / surface_config.height as f32),

            instance,
            surface,
            surface_config,
            adapter,
            device,
            queue,
            pipeline,
            window,

            uniform_bind_group,

            vertex_buf,
            proj_view_matrix_buf,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn device_input(&mut self, event: winit::event::DeviceEvent) {
        use winit::event::DeviceEvent;
        match event {
            DeviceEvent::MouseWheel {
                delta: winit::event::MouseScrollDelta::LineDelta(_, d),
            } => {
                self.camera.fovy = cgmath::Rad((self.camera.fovy.0 + 0.001 * d).clamp(0.01, 3.14));
            }
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.camera.pitch += cgmath::Rad(-dy as f32 / 256.0);
                self.camera.yaw += cgmath::Rad(dx as f32 / 256.0);
            }
            _ => (),
        }
    }

    pub fn window_input(&mut self, event: winit::event::WindowEvent) {
        use cgmath::*;
        use winit::{
            event::{KeyEvent, WindowEvent},
            keyboard::{KeyCode, PhysicalKey},
        };
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: winit::event::ElementState::Pressed,
                        physical_key,
                        ..
                    },
                ..
            } => match physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    let f = self.camera.forward();
                    self.camera.position += f * 0.1;
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    let f = self.camera.forward();
                    let left = Vector3::unit_y().cross(f);
                    self.camera.position += left * 0.1;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    let f = self.camera.forward();
                    self.camera.position -= f * 0.1;
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    let f = self.camera.forward();
                    let right = -1.0 * Vector3::unit_y().cross(f);
                    self.camera.position += right * 0.1;
                }
                PhysicalKey::Code(KeyCode::Space) => {
                    self.camera.position += Vector3::unit_y() * 0.1;
                }
                PhysicalKey::Code(KeyCode::ShiftLeft) => {
                    self.camera.position += Vector3::unit_y() * -0.1;
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);

        self.camera.aspect = new_size.width as f32 / new_size.height as f32;
    }

    pub fn width(&self) -> u32 {
        self.surface_config.width
    }

    pub fn height(&self) -> u32 {
        self.surface_config.height
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let mat: [[f32; 4]; 4] = self.camera.proj_view_matrix().into();
        self.queue
            .write_buffer(&self.proj_view_matrix_buf, 0, bytemuck::cast_slice(&mat));

        let out = self.surface.get_current_texture()?;
        let view = out.texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Discard,
                    },
                    resolve_target: None,
                    view: &view,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));

            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        out.present();
        Ok(())
    }
}
