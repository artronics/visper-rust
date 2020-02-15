#version 450

layout(location = 0) in vec2 v_Pos;

layout (set = 0, binding = 0) uniform Globals {
    mat4 u_Transform;
    float u_Scale;
};

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    vec2 p_Scale = vec2(1.0, 1.0);
    vec2 p_Pos = vec2(1.0, 1.0);

    mat4 i_Transform = mat4(
        vec4(p_Scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, p_Scale.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(p_Pos - vec2(0.5, 0.5), 0.0, 1.0)
    );
    //    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    gl_Position = u_Transform * vec4(v_Pos, 0.0, 1.0);
}
