use crate::render::gl::raw::*;
use crate::render::gl::shader::shader_program::*;
use cgmath::*;

static VERTEX: &str = r#"
#version 330

const float PI     = 3.141592653589793238462643383279;
const float TWO_PI = 6.283185307179586476925286766559;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_size;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;
layout(location = 4) in float a_rotation;
out vec2 v_uv;
out vec4 v_color;

uniform mat4 ortho;

uniform vec2 pos_lut[4] = vec2[4](
    vec2(0.0, 65536.0),     // left top
    vec2(0.0, 0.0),         // left bottom
    vec2(65536.0, 65536.0), // right top
    vec2(65536.0, 0.0));    // right bottom

vec4 rotateZ(vec3 pos, float psi) {
    vec2 origin = vec2(
        a_pos.x + (a_size.x * 32768.0),
        a_pos.y + (a_size.y * 32768.0));
    return vec4(
        (cos(psi) * (pos.x - origin.x)) - (sin(psi) * (pos.y - origin.y)) + origin.x,
        (sin(psi) * (pos.x - origin.x)) + (cos(psi) * (pos.y - origin.y)) + origin.y,
        pos.z,
        1.0);
}

void main() {
    // (x:left, y:right, z:bottom, w:top)
    vec2 uv[4];
    uv[0] = vec2(a_uv.x, a_uv.w); // left top
    uv[1] = vec2(a_uv.x, a_uv.z); // left bottom
    uv[2] = vec2(a_uv.y, a_uv.w); // right top
    uv[3] = vec2(a_uv.y, a_uv.z); // right bottom
    v_uv = uv[gl_VertexID];
    v_color = a_color;
    
    vec3 pos = a_pos + vec3(a_size * pos_lut[gl_VertexID], 0.0);
    gl_Position = ortho * rotateZ(pos, TWO_PI * a_rotation);
}
"#;
static FRAGMENT: &str = r#"
#version 330

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D tex;

void main() {
    a_color = texture(tex, v_uv) * v_color;
    if (a_color.a <= 0.0) {
        discard;
    }
}
"#;

pub struct TextureShader {
    program: ShaderProgram,
    uniform_ortho: i32,
    uniform_texture: i32,
}

impl TextureShader {
    pub fn new() -> TextureShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        let uniform_texture = program.get_uniform_location("tex");
        TextureShader {
            program: program,
            uniform_ortho: uniform_ortho,
            uniform_texture: uniform_texture,
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    /// Updates the ortho uniform in the shader.
    pub fn ortho(&self, matrix: &Matrix4<f32>) {
        uniform_matrix_4fv(self.uniform_ortho, 1, false, matrix.as_ptr());
    }

    /// Updates the texture uniform in the shader.
    pub fn texture(&self, unit: TextureUnit) {
        let unit = (unit as u32 - TextureUnit::Atlas as u32) as i32;
        uniform_1i(self.uniform_texture, unit);
    }
}
