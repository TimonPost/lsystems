#version 450

layout(location = 0) in vec4 in_position;

void main() {
    mat4 projection =  mat4(1.0);
    mat4 view = mat4(1.0);
    mat4 scale = mat4(0.00000000000000001);
    
    gl_Position = (projection * view * scale * vec4(in_position.x,in_position.y,in_position.z,1.0));
}
