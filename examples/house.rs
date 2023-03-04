#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::egui;

use egui::mutex::Mutex;
use glow::{HasContext, NativeBuffer, NativeProgram, NativeVertexArray};

use scebpl_system::*;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 380.0)),
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

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    rotating_triangle: Arc<Mutex<LSystemRenderer>>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let definition = format!(
            "lsystem KochCurve {{
                axiom F;
    
                replace F by F+F-F-F+F;
                interpret + as RotateXAction(10);
                interpret - as RotateXAction(10);
            }}
        ",
        );

        let lexer = Lexer::new();

        let lex = lexer.lex(definition);
        let tokens = LexedTokens::new(lex);

        let item = parse(tokens);

        let mut lsystem = LSystemParser::parse(item);
        let alphabet = lsystem.generate(2);

        let mut resolver = ActionResolver {
            actions: Default::default(),
        };
        resolver.add_action_resolver::<RotateXAction>();

        let context = lsystem.run(&resolver, &alphabet);

        Self {
            rotating_triangle: Arc::new(Mutex::new(LSystemRenderer::new(gl, context))),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.label("Drag to rotate!");
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.rotating_triangle.lock().destroy(gl);
        }
    }
}

impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        // Clone locals so we can move them into the paint callback:

        let rotating_triangle = self.rotating_triangle.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                rotating_triangle.lock().paint(painter.gl(), 0.0);
            })),
        };
        ui.painter().add(callback);
    }
}

struct LSystemRenderer {
    program: glow::Program,
    vao_pos: glow::VertexArray,
    vbo_pos: glow::NativeBuffer,
    context: ExecuteContext,
}

impl LSystemRenderer {
    fn new(gl: &glow::Context, lcontext: ExecuteContext) -> Self {
        let points = vec![
            -0.5, 0.5, // top-left
            0.5, 0.5, // top-right
            0.5, -0.5, // bottom-right
            -0.5, -0.5, // bottom-left
        ];

        let (program, (vbo_pos, vao_pos)) = unsafe {
            (
                Self::create_program(gl),
                Self::create_vertex_buffer(gl, &lcontext, points, 0),
            )
        };

        Self {
            program,
            vao_pos,
            vbo_pos,
            context: lcontext,
        }
    }

    unsafe fn create_program(gl: &glow::Context) -> NativeProgram {
        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        let program = gl.create_program().expect("Cannot create program");

        let shader_sources = [
            (glow::VERTEX_SHADER, VERTEX_SHADER),
            (glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
            (glow::GEOMETRY_SHADER, GEOMETRY_SHADER),
        ];

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(|(shader_type, shader_source)| {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
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

    unsafe fn create_vertex_buffer(
        gl: &glow::Context,
        _lcontext: &ExecuteContext,
        data: Vec<f32>,
        index: u32,
    ) -> (NativeBuffer, NativeVertexArray) {
        let turtle_path_data_u8: &[u8] = core::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * core::mem::size_of::<f32>(),
        );

        // We construct a buffer and upload the data
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, turtle_path_data_u8, glow::STATIC_DRAW);

        // We now construct a vertex array to describe the format of the input buffer
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_pointer_f32(index, 2, glow::FLOAT, false, 8, 0);

        (vbo, vao)
    }

    unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
        let uniform_location = gl.get_uniform_location(program, name);
        // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
        gl.uniform_1_f32(uniform_location.as_ref(), value)
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao_pos);
        }
    }

    fn paint(&self, gl: &glow::Context, _angle: f32) {
        use glow::HasContext as _;

        unsafe {
            gl.use_program(Some(self.program));
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.draw_arrays(glow::POINTS, 0, 4);
        }
    }
}

const VERTEX_SHADER: &str = r#"
layout (location = 0) in vec2 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0); 
}
"#;

const FRAGMENT_SHADER: &str = r#"
out vec4 FragColor;

void main()
{
    FragColor = vec4(0.0, 1.0, 0.0, 1.0);   
}  
"#;

const GEOMETRY_SHADER: &str = r#"
layout (points) in;
layout (points, max_vertices = 1) out;

void main() {    
    gl_Position = gl_in[0].gl_Position; 
    EmitVertex();
    EndPrimitive();
}  
"#;
