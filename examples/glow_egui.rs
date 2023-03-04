#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::egui;

use egui::{mutex::Mutex, panel::Side};
use glow::{HasContext, NativeBuffer, NativeProgram, NativeVertexArray};

use macaw::BoundingBox;
use regex::Regex;
use scebpl_system::*;
use std::{f32::consts::PI, fs, path::PathBuf, sync::Arc};

/// Rotation action arround the x axis.
pub struct MoveForward(pub f32, pub char);

impl LSystemAction for MoveForward {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle.forward(self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let x = params.get(0).unwrap();

        println!("Interpret {} ({})", "MoveForward", x);

        Some(MoveForward(x, 'F'))
    }

    fn name() -> &'static str {
        "MoveForward"
    }
}

pub struct RotateRight(pub f32, pub char);

impl LSystemAction for RotateRight {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        println!(
            "Rotate at {} with angle {}",
            context.turtle.origin(),
            self.0
        );
        context.turtle.rotate_z(self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let x = params.get(0).unwrap();

        Some(RotateRight(x, '+'))
    }

    fn name() -> &'static str {
        "RotateRight"
    }
}

pub struct RotateLeft(pub f32, pub char);

impl LSystemAction for RotateLeft {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle.rotate_z(-self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let x = params.get(0).unwrap();

        Some(RotateLeft(x, '-'))
    }

    fn name() -> &'static str {
        "RotateLeft"
    }
}

const WINDOW_X: f32 = 1000.0;
const WINDOW_Y: f32 = 500.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WINDOW_X, WINDOW_Y)),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct LScriptInstance {
    script: String,
    path: PathBuf,
}

impl LScriptInstance {
    pub fn load(path: PathBuf) -> Self {
        let script = fs::read_to_string(path.clone()).unwrap();

        Self { script, path }
    }

    pub fn safe(&self) {
        fs::write(&self.path, self.script.clone());
    }
}

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    lsystem_renderer: Arc<Mutex<Option<LSystemRenderer>>>,
    forward_len: f32,
    rotate_left: f32,
    rotate_right: f32,
    generations: u8,
    lsystem_script: LScriptInstance,
    gl: Arc<glow::Context>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend")
            .clone();

        Self {
            lsystem_renderer: Arc::new(Mutex::new(None)),
            lsystem_script: LScriptInstance::load(PathBuf::from("./examples/koch.ls")),
            forward_len: 1.0,
            rotate_left: PI / 2.0,
            rotate_right: PI / 2.0,
            generations: 3,
            gl,
        }
    }

    fn recompile_lsystem(&mut self) {
        let instantiated_script = self.lsystem_script.script.clone();

        let lexer = Lexer::new();

        let lex = lexer.lex(instantiated_script);
        let tokens = LexedTokens::new(lex);

        let item = parse(tokens);

        let mut lsystem = LSystemParser::parse(item);
        let alphabet = lsystem.generate(self.generations);

        let mut resolver = ActionResolver {
            actions: Default::default(),
        };
        resolver.add_action_resolver::<RotateLeft>();
        resolver.add_action_resolver::<RotateRight>();
        resolver.add_action_resolver::<MoveForward>();

        let context = lsystem.run(&resolver, &alphabet);

        self.lsystem_renderer = Arc::new(Mutex::new(Some(LSystemRenderer::new(&self.gl, context))))
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left, "canvas-painter")
            .exact_width((WINDOW_X / 3.0) * 2.0)
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui);
                });
            });

        egui::SidePanel::new(Side::Right, "code_editor")
            .exact_width((WINDOW_X / 3.0) * 1.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.spacing();
                    ui.separator();
                    ui.text_edit_multiline(&mut self.lsystem_script.script);

                    ui.horizontal(|ui| {
                        if ui.button("Recomile").clicked() {
                            self.recompile_lsystem()
                        }

                        if ui.button("Safe").clicked() {
                            self.lsystem_script.safe();
                        }
                    });

                    ui.separator();
                })});
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        let lock = self.lsystem_renderer.lock();
        if let (Some(gl), Some(render)) = (gl, lock.as_ref()) {
            render.destroy(gl);
        }
    }
}

impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, _response) = ui.allocate_exact_size(
            egui::Vec2::new((WINDOW_X / 3.0) * 2.0, WINDOW_Y * 0.8),
            egui::Sense::drag(),
        );

        let renderer = self.lsystem_renderer.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let mut lock = renderer.lock();
                if let Some(render) = lock.as_mut() {
                    render.run_compute_shader(painter.gl());
                    render.paint(painter.gl());
                }
            })),
        };
        ui.painter().add(callback);
    }
}

struct LSystemRenderer {
    compute_program: glow::Program,
    render_program: glow::Program,

    vbo_pos: glow::NativeBuffer,
    rectangle_vbo: glow::NativeBuffer,

    render_vbo: glow::NativeBuffer,
    render_vao: glow::NativeVertexArray,
    triangles_verts: Vec<f32>,
    triangle_verts_indicies: usize,
    should_run_compute: bool,
}

impl LSystemRenderer {
    fn new(gl: &glow::Context, lcontext: ExecuteContext) -> Self {
        let mut bounds = BoundingBox::ZERO;

        let positions = lcontext
            .snapshot
            .iter()
            .flat_map(|turtle| {
                let x = turtle.turtle.origin()[0];
                let y = turtle.turtle.origin()[1];
                let z = turtle.turtle.origin()[2];

                if x < bounds.min.x {
                    bounds.min.x = x;
                }
                if y < bounds.min.y {
                    bounds.min.y = y;
                }
                if x > bounds.max.x {
                    bounds.min.x = x;
                }
                if y > bounds.max.y {
                    bounds.min.y = y;
                }

                vec![
                    turtle.turtle.origin()[0],
                    turtle.turtle.origin()[1],
                    turtle.turtle.origin()[2],
                    0.0,
                ]
            })
            .collect::<Vec<f32>>();

        let verticies = 4;

        let path_count = (positions.len() / verticies) - 1;
        let triangles_per_path = 2;
        let triangle_indicies_per_path = triangles_per_path * 3;
        let total_indicies = triangle_indicies_per_path * path_count;
        let total_floats = total_indicies * verticies;

        let triangles_verts = vec![0.0; total_floats];

        assert_eq!(triangles_verts.len(), total_floats);
        assert_eq!(triangles_verts.capacity(), total_floats);

        let (compute_program, render_program, vbo_pos, rectangle_vbo, (render_vbo, render_vao)) = unsafe {
            (
                Self::create_compute_program(gl),
                Self::create_render_program(gl),
                Self::create_storeage_buf(gl, to_bytes(positions.as_slice()), 0),
                Self::create_storeage_buf(gl, to_bytes(triangles_verts.as_slice()), 1),
                Self::create_vao_buf(gl, to_bytes(triangles_verts.as_slice()), 0),
            )
        };

        Self {
            compute_program,
            render_program,
            vbo_pos,
            rectangle_vbo,

            triangles_verts,
            triangle_verts_indicies: total_indicies,
            render_vbo,
            render_vao,
            should_run_compute: true,
        }
    }

    unsafe fn create_render_program(gl: &glow::Context) -> NativeProgram {
        let shader_sources = [
            (glow::VERTEX_SHADER, include_str!("./shader.vert")),
            (glow::FRAGMENT_SHADER, include_str!("./shader.frag")),
        ];

        Self::compile_shaders(gl, &shader_sources)
    }

    unsafe fn create_compute_program(gl: &glow::Context) -> NativeProgram {
        let shader_sources = [(glow::COMPUTE_SHADER, include_str!("./shader.comp"))];

        Self::compile_shaders(gl, &shader_sources)
    }

    unsafe fn compile_shaders(gl: &glow::Context, shader_sources: &[(u32, &str)]) -> NativeProgram {
        let program = gl.create_program().expect("Cannot create program");

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(|(shader_type, shader_source)| {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                assert!(
                    gl.get_shader_compile_status(shader),
                    "Failed to compile {shader_type}: {}",
                    gl.get_shader_info_log(shader)
                );
                gl.attach_shader(program, shader);
                shader
            })
            .collect();

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        program
    }

    unsafe fn create_storeage_buf(gl: &glow::Context, data: &[u8], index: u32) -> NativeBuffer {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::SHADER_STORAGE_BUFFER, data, glow::DYNAMIC_DRAW);
        gl.bind_buffer_base(glow::SHADER_STORAGE_BUFFER, index, Some(vbo));

        vbo
    }

    unsafe fn create_vao_buf(
        gl: &glow::Context,
        data: &[u8],
        index: u32,
    ) -> (NativeBuffer, NativeVertexArray) {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.vertex_attrib_pointer_f32(index, 4, glow::FLOAT, false, 16, 0);
        gl.enable_vertex_attrib_array(index);

        gl.bind_vertex_array(None);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);

        (vbo, vao)
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.compute_program);
        }
    }

    fn run_compute_shader(&mut self, gl: &glow::Context) {
        use glow::HasContext as _;

        if !self.should_run_compute {
            return;
        }

        unsafe {
            gl.use_program(Some(self.compute_program));

            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(self.vbo_pos));
            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(self.rectangle_vbo));

            gl.dispatch_compute(1, 1, 1);
            gl.memory_barrier(glow::SHADER_STORAGE_BARRIER_BIT);

            gl.get_buffer_sub_data(
                glow::SHADER_STORAGE_BUFFER,
                0,
                to_bytes_mut(&mut self.triangles_verts),
            );

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.render_vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                to_bytes(self.triangles_verts.as_slice()),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        self.should_run_compute = false;
    }

    fn paint(&mut self, gl: &glow::Context) {
        use glow::HasContext as _;

        unsafe {
            gl.use_program(Some(self.render_program));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.render_vbo));
            gl.bind_vertex_array(Some(self.render_vao));
            gl.enable_vertex_attrib_array(0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.draw_arrays(glow::TRIANGLES, 0, self.triangle_verts_indicies as i32);
        }
    }
}

fn to_bytes<'a>(elements: &'a [f32]) -> &'a [u8] {
    unsafe {
        core::slice::from_raw_parts(
            elements.as_ptr() as *const u8,
            elements.len() * core::mem::size_of::<f32>(),
        )
    }
}

fn to_bytes_mut<'a>(elements: &'a mut [f32]) -> &'a mut [u8] {
    unsafe {
        core::slice::from_raw_parts_mut(
            elements.as_ptr() as *mut u8,
            elements.len() * core::mem::size_of::<f32>(),
        )
    }
}

// Debug purposes
#[allow(unused)]
fn print_verts(verts: Vec<f32>) {
    let mut i = 0;
    while i < verts.len() {
        let verts = &verts[i..i + 4];
        println!("({},\t{},\t{})", verts[0], verts[1], verts[2]);
        i += 4;
    }
}

// Debug purposes
#[allow(unused)]
fn rust_shader(positions: &Vec<f32>, out_triangle: &mut Vec<f32>) {
    let verticies = 4;

    let mut i = 0;
    while i < positions.len() - verticies {
        let next_i = i + verticies;

        let pos = &positions[i..next_i];
        let next_pos = &positions[next_i..next_i + verticies];

        let start = macaw::Vec3::new(pos[0], pos[1], pos[2]);
        let end = macaw::Vec3::new(next_pos[0], next_pos[1], next_pos[2]);

        if let Some(dir) = (end - start).try_normalize() {
            let right = macaw::Vec3::new(0.0, 0.0, 1.0).cross(dir).normalize();
            let _up = dir.cross(right).normalize();

            let thickness = 0.01;

            let p0 = start + right * thickness * 0.5;
            let p1 = start - right * thickness * 0.5;
            let p2 = end + right * thickness * 0.5;
            let p3 = end - right * thickness * 0.5;

            let mut index = (i / verticies) * 18;
            out_triangle[index..index + verticies].copy_from_slice(&[p0.x, p0.y, p0.z, 0.0]);

            index += verticies;
            out_triangle[index..index + verticies].copy_from_slice(&[p1.x, p1.y, p1.z, 0.0]);

            index += verticies;
            out_triangle[index..index + verticies].copy_from_slice(&[p2.x, p2.y, p2.z, 0.0]);

            index += verticies;
            out_triangle[index..index + verticies].copy_from_slice(&[p1.x, p1.y, p1.z, 0.0]);

            index += verticies;
            out_triangle[index..index + verticies].copy_from_slice(&[p2.x, p2.y, p2.z, 0.0]);

            index += verticies;
            out_triangle[index..index + verticies].copy_from_slice(&[p3.x, p3.y, p3.z, 0.0]);
        }
        i += verticies;
    }
}
