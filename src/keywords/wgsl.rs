/*!
Keywords for [WGSL][wgsl] (WebGPU Shading Language).

[wgsl]: https://gpuweb.github.io/gpuweb/wgsl.html
*/

// https://gpuweb.github.io/gpuweb/wgsl/#keyword-summary
pub const RESERVED: &[&str] = &[
    // type-defining
    "array",
    "atomic",
    "bool",
    "f32",
    "f16",
    "i32",
    "mat2x2",
    "mat2x3",
    "mat2x4",
    "mat3x2",
    "mat3x3",
    "mat3x4",
    "mat4x2",
    "mat4x3",
    "mat4x4",
    "override",
    "ptr",
    "sampler",
    "sampler_comparison",
    "struct",
    "texture_1d",
    "texture_2d",
    "texture_2d_array",
    "texture_3d",
    "texture_cube",
    "texture_cube_array",
    "texture_multisampled_2d",
    "texture_storage_1d",
    "texture_storage_2d",
    "texture_storage_2d_array",
    "texture_storage_3d",
    "texture_depth_2d",
    "texture_depth_2d_array",
    "texture_depth_cube",
    "texture_depth_cube_array",
    "texture_depth_multisampled_2d",
    "u32",
    "vec2",
    "vec3",
    "vec4",
    // other
    "bitcast",
    "break",
    "case",
    "const",
    "continue",
    "continuing",
    "default",
    "discard",
    "else",
    "enable",
    "fallthrough",
    "false",
    "fn",
    "for",
    "function",
    "if",
    "let",
    "loop",
    "private",
    "return",
    "storage",
    "switch",
    "true",
    "type",
    "uniform",
    "var",
    "while",
    "workgroup",
    // reserved
    "AppendStructuredBuffer",
    "BlendState",
    "Buffer",
    "ByteAddressBuffer",
    "CompileShader",
    "ComputeShader",
    "ConsumeStructuredBuffer",
    "DepthStencilState",
    "DepthStencilView",
    "DomainShader",
    "GeometryShader",
    "Hullshader",
    "InputPatch",
    "LineStream",
    "NULL",
    "OutputPatch",
    "PixelShader",
    "PointStream",
    "RWBuffer",
    "RWByteAddressBuffer",
    "RWStructuredBuffer",
    "RWTexture1D",
    "RWTexture1DArray",
    "RWTexture2D",
    "RWTexture2DArray",
    "RWTexture3D",
    "RasterizerState",
    "RenderTargetView",
    "SamplerComparisonState",
    "SamplerState",
    "Self",
    "StructuredBuffer",
    "Texture1D",
    "Texture1DArray",
    "Texture2D",
    "Texture2DArray",
    "Texture2DMS",
    "Texture2DMSArray",
    "Texture3D",
    "TextureCube",
    "TextureCubeArray",
    "TriangleStream",
    "VertexShader",
    "abstract",
    "active",
    "alignas",
    "alignof",
    "as",
    "asm",
    "asm_fragment",
    "async",
    "atomic_uint",
    "attribute",
    "auto",
    "await",
    "become",
    "bf16",
    "cast",
    "catch",
    "cbuffer",
    "centroid",
    "char",
    "class",
    "co_await",
    "co_return",
    "co_yield",
    "coherent",
    "column_major",
    "common",
    "compile",
    "compile_fragment",
    "concept",
    "const_cast",
    "consteval",
    "constexpr",
    "constinit",
    "crate",
    "debugger",
    "decltype",
    "delete",
    "demote",
    "demote_to_helper",
    "do",
    "dword",
    "dynamic_cast",
    "enum",
    "explicit",
    "export",
    "extends",
    "extern",
    "external",
    "f64",
    "filter",
    "final",
    "finally",
    "fixed",
    "flat",
    "friend",
    "from",
    "fvec2",
    "fvec3",
    "fvec4",
    "fxgroup",
    "get",
    "goto",
    "groupshared",
    "handle",
    "highp",
    "hvec2",
    "hvec3",
    "hvec4",
    "i16",
    "i64",
    "i8",
    "iimage1D",
    "iimage1DArray",
    "iimage2D",
    "iimage2DArray",
    "iimage2DMS",
    "iimage2DMSArray",
    "iimage2DRect",
    "iimage3D",
    "iimageBuffer",
    "iimageCube",
    "iimageCubeArray",
    "image1D",
    "image1DArray",
    "image2D",
    "image2DArray",
    "image2DMS",
    "image2DMSArray",
    "image2DRect",
    "image3D",
    "imageBuffer",
    "imageCube",
    "imageCubeArray",
    "impl",
    "implements",
    "import",
    "inline",
    "inout",
    "instanceof",
    "interface",
    "invariant",
    "isampler1D",
    "isampler1DArray",
    "isampler2D",
    "isampler2DArray",
    "isampler2DMS",
    "isampler2DMSArray",
    "isampler2DRect",
    "isampler3D",
    "isamplerBuffer",
    "isamplerCube",
    "isamplerCubeArray",
    "isubpassInput",
    "isubpassInputMS",
    "itexture1D",
    "itexture1DArray",
    "itexture2D",
    "itexture2DArray",
    "itexture2DMS",
    "itexture2DMSArray",
    "itexture2DRect",
    "itexture3D",
    "itextureBuffer",
    "itextureCube",
    "itextureCubeArray",
    "layout",
    "line",
    "lineadj",
    "linear",
    "lowp",
    "macro",
    "macro_rules",
    "mat",
    "match",
    "matrix",
    "mediump",
    "meta",
    "mod",
    "module",
    "move",
    "mut",
    "mutable",
    "namespace",
    "new",
    "nil",
    "noexcept",
    "noinline",
    "nointerpolation",
    "noperspective",
    "null",
    "nullptr",
    "of",
    "operator",
    "package",
    "packoffset",
    "partition",
    "pass",
    "patch",
    "pixelfragment",
    "point",
    "precise",
    "precision",
    "premerge",
    "priv",
    "protected",
    "pub",
    "public",
    "readonly",
    "ref",
    "regardless",
    "register",
    "reinterpret_cast",
    "requires",
    "resource",
    "restrict",
    "row_major",
    "samper",
    "sample",
    "sampler1D",
    "sampler1DArray",
    "sampler1DArrayShadow",
    "sampler1DShadow",
    "sampler2D",
    "sampler2DArray",
    "sampler2DArrayShadow",
    "sampler2DMS",
    "sampler2DMSArray",
    "sampler2DRect",
    "sampler2DRectShadow",
    "sampler2DShadow",
    "sampler3D",
    "sampler3DRect",
    "samplerBuffer",
    "samplerCube",
    "samplerCubeArray",
    "samplerCubeArrayShadow",
    "samplerCubeShadow",
    "samplerShadow",
    "self",
    "set",
    "shared",
    "signed",
    "sizeof",
    "smooth",
    "snorm",
    "stateblock",
    "stateblock_state",
    "static",
    "static_assert",
    "static_cast",
    "std",
    "string",
    "subpassInput",
    "subpassInputMS",
    "subroutine",
    "super",
    "superp",
    "target",
    "tbuffer",
    "technique",
    "technique10",
    "technique11",
    "template",
    "texture1D",
    "texture1DArray",
    "texture2D",
    "texture2DArray",
    "texture2DMS",
    "texture2DMSArray",
    "texture2DRect",
    "texture3D",
    "textureBuffer",
    "textureCube",
    "textureCubeArray",
    "this",
    "thread_local",
    "throw",
    "trait",
    "triangle",
    "triangleadj",
    "try",
    "typedef",
    "typeid",
    "typename",
    "typeof",
    "u16",
    "u64",
    "u8",
    "uimage1D",
    "uimage1DArray",
    "uimage2D",
    "uimage2DArray",
    "uimage2DMS",
    "uimage2DMSArray",
    "uimage2DRect",
    "uimage3D",
    "uimageBuffer",
    "uimageCube",
    "uimageCubeArray",
    "union",
    "unless",
    "unorm",
    "unsafe",
    "unsigned",
    "unsized",
    "usampler1D",
    "usampler1DArray",
    "usampler2D",
    "usampler2DArray",
    "usampler2DMS",
    "usampler2DMSArray",
    "usampler2DRect",
    "usampler3D",
    "usamplerBuffer",
    "usamplerCube",
    "usamplerCubeArray",
    "use",
    "using",
    "usubpassInput",
    "usubpassInputMS",
    "utexture1D",
    "utexture1DArray",
    "utexture2D",
    "utexture2DArray",
    "utexture2DMS",
    "utexture2DMSArray",
    "utexture2DRect",
    "utexture3D",
    "utextureBuffer",
    "utextureCube",
    "utextureCubeArray",
    "varying",
    "vec",
    "vector",
    "vertexfragment",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "wgsl",
    "where",
    "with",
    "writeonly",
    "yield",
];
