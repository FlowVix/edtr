

// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) tex_coords: vec2<f32>,
// }

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) tex_coords: vec2<f32>,
// }

// @vertex
// fn vs_main(
//     model: VertexInput,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     out.tex_coords = model.tex_coords;
//     out.clip_position = vec4<f32>(model.position, 1.0);
//     return out;
// }


// @vertex
// fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
//     let pi = 3.141592;
//     let x = cos( 2.0 * pi / 5.0 * f32(in_vertex_index));
//     let y = sin( 2.0 * pi / 5.0 * f32(in_vertex_index));
//     return vec4<f32>(x, y, 0.0, 1.0);
// }

// @fragment
// fn fs_main() -> @location(0) vec4<f32> {
//     return vec4<f32>(1.0, 0.0, 0.0, 0.5);
// }

// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}




// struct VertexInput {
//     [[location(0)]] position: vec2<i32>;
//     [[location(1)]] tex_coords: vec2<f32>;
//     // [[location(2)]] tint: vec4<f32>;
//     [[location(2)]] tint_index: i32;
//     [[location(3)]] target_coords: vec2<f32>;
// };

// struct VertexOutput {
//     [[builtin(position)]] clip_position: vec4<f32>;
//     [[location(0)]] tex_coords: vec2<f32>;
//     // [[location(1)]] tint: vec4<f32>;
//     [[location(1)]] tint_index: i32;
//     [[location(2)]] target_coords: vec2<f32>;
// };

// let total_shapes = 1024;

// let factor = 1000.0;
// struct Tint {
//     tint: array<array<atomic<u32>, 3>, total_shapes>;
//     counts: array<atomic<u32>, total_shapes>;
//     opacity: array<f32, total_shapes>;
//     diff: array<atomic<i32>, total_shapes>;
// };


// [[group(1), binding(0)]]
// var t_target: texture_2d<f32>;
// [[group(1), binding(1)]]
// var s_target: sampler;


// [[group(2), binding(0)]]
// var<storage, read_write> tint: Tint;

// [[group(3), binding(0)]]
// var t_current: texture_2d<f32>;
// [[group(3), binding(1)]]
// var s_current: sampler;


// [[stage(vertex)]]
// fn vs_main(
//     model: VertexInput,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     out.tex_coords = model.tex_coords;
//     let size = textureDimensions(t_target);
//     out.clip_position = vec4<f32>(
//         (f32(model.position.x) / f32(size.x)) * 2.0 - 1.0, 
//         -((f32(model.position.y) / f32(size.y)) * 2.0 - 1.0),
//         0.0, 1.0
//     );
//     out.tint_index = model.tint_index;
//     out.target_coords = model.target_coords;
//     return out;
// }