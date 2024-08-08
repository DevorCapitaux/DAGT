use crate::{attr_bitstruct, attr_enum};
use std::ffi::{c_char, c_long, c_uchar};

pub type GLchar = c_char;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLfloat = f32;
pub type GLsizei = i32;
pub type GLenum = i32;
pub type GLbitfield = i32;
pub type GLsizeiptr = c_long;
pub type GLboolean = c_uchar;

pub const GL_TRUE: GLint = 0x1;
pub const GL_FALSE: GLint = 0x0;

attr_enum!(
    pub enum GlBoolean {
        True = GL_TRUE,
        False = GL_FALSE,
    }
);

impl From<GlBoolean> for u8 {
    fn from(value: GlBoolean) -> Self {
        value as u8
    }
}

pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x4000;
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x0100;
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x0400;

attr_bitstruct!(
    pub bitstruct GlClearMask(0) {
        color = GL_COLOR_BUFFER_BIT,
        depth = GL_DEPTH_BUFFER_BIT,
        stencil = GL_STENCIL_BUFFER_BIT,
    }
);

pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_HALF_FLOAT: GLenum = 0x140B;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_FIXED: GLenum = 0x140C;
pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;

attr_enum!(
    pub enum GlElType {
        Byte = GL_BYTE,
        UnsignedByte = GL_UNSIGNED_BYTE,
        Short = GL_SHORT,
        UnsignedShort = GL_UNSIGNED_SHORT,
        Int = GL_INT,
        UnsignedInt = GL_UNSIGNED_INT,
        HalfFloat = GL_HALF_FLOAT,
        Float = GL_FLOAT,
        Fixed = GL_FIXED,
        Int2_10_10_10Rev = GL_INT_2_10_10_10_REV,
        UnsignedInt2_10_10_10Rev = GL_UNSIGNED_INT_2_10_10_10_REV,
    }
);

pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C7F;
pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;

attr_enum!(
    pub enum GlBuffTarget {
        ArrayBuffer = GL_ARRAY_BUFFER,
        AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER,
        CopyReadBuffer = GL_COPY_READ_BUFFER,
        CopyWriteBuffer = GL_COPY_WRITE_BUFFER,
        DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER,
        DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER,
        ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER,
        PixelPackBuffer = GL_PIXEL_PACK_BUFFER,
        PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER,
        ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER,
        TextureBuffer = GL_TEXTURE_BUFFER,
        TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER,
        UniformBuffer = GL_UNIFORM_BUFFER,
    }
);

pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STREAM_READ: GLenum = 0x88E1;
pub const GL_STREAM_COPY: GLenum = 0x88E2;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_READ: GLenum = 0x88E5;
pub const GL_STATIC_COPY: GLenum = 0x88E6;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;

attr_enum!(
    pub enum GlBuffUsage {
        StreamDraw = GL_STREAM_DRAW,
        StreamRead = GL_STREAM_READ,
        StreamCopy = GL_STREAM_COPY,
        StaticDraw = GL_STATIC_DRAW,
        StaticRead = GL_STATIC_READ,
        StaticCopy = GL_STATIC_COPY,
        DynamicDraw = GL_DYNAMIC_DRAW,
        DynamicRead = GL_DYNAMIC_READ,
        DynamicCopy = GL_DYNAMIC_COPY,
    }
);

pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;

attr_enum!(
    pub enum GlShaderType {
        Vertex = GL_VERTEX_SHADER,
        Fragment = GL_FRAGMENT_SHADER,
        Compute = GL_COMPUTE_SHADER,
    }
);

pub const GL_SHADER_TYPE: GLenum = 0x8B4f;
pub const GL_DELETE_STATUS: GLenum = 0x8B80;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;

attr_enum!(
    pub enum GlShaderParam {
        ShaderType = GL_SHADER_TYPE,
        DeleteStatus = GL_DELETE_STATUS,
        CompileStatus = GL_COMPILE_STATUS,
        InfoLogLength = GL_INFO_LOG_LENGTH,
        ShaderSourceLength = GL_SHADER_SOURCE_LENGTH,
    }
);

pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub const GL_PROGRAM_SEPARABLE: GLenum = 0x8258;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub const GL_TESS_GEN_MODE: GLenum = 0x8E76;
pub const GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub const GL_TESS_GEN_SPACING: GLenum = 0x8E77;
pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;

attr_enum!(
    pub enum GlProgramParam {
        ActiveAtomicCounterBuffers = GL_ACTIVE_ATOMIC_COUNTER_BUFFERS,
        ActiveAttributes = GL_ACTIVE_ATTRIBUTES,
        ActiveAttributeMaxLength = GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
        ActiveUniforms = GL_ACTIVE_UNIFORMS,
        ActiveUniformBlocks = GL_ACTIVE_UNIFORM_BLOCKS,
        ActiveUniformBlockMaxNameLength = GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
        ActiveUniformMaxLength = GL_ACTIVE_UNIFORM_MAX_LENGTH,
        AttachedShaders = GL_ATTACHED_SHADERS,
        ComputeWorkGroupSize = GL_COMPUTE_WORK_GROUP_SIZE,
        DeleteStatus = GL_DELETE_STATUS,
        GeometryShaderInvocations = GL_GEOMETRY_SHADER_INVOCATIONS,
        InfoLogLength = GL_INFO_LOG_LENGTH,
        LinkStatus = GL_LINK_STATUS,
        ProgramBinaryRetrievableHint = GL_PROGRAM_BINARY_RETRIEVABLE_HINT,
        ProgramSeparable = GL_PROGRAM_SEPARABLE,
        TessControlOutputVertices = GL_TESS_CONTROL_OUTPUT_VERTICES,
        TessGenMode = GL_TESS_GEN_MODE,
        TessGenPointMode = GL_TESS_GEN_POINT_MODE,
        TessGenSpacing = GL_TESS_GEN_SPACING,
        TessGenVertexOrder = GL_TESS_GEN_VERTEX_ORDER,
        TransformFeedbackBufferMode = GL_TRANSFORM_FEEDBACK_BUFFER_MODE,
        TransformFeedbackVaryings = GL_TRANSFORM_FEEDBACK_VARYINGS,
        TransformFeedbackVaryingMaxLength = GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
        ValidateStatus = GL_VALIDATE_STATUS,
    }
);

pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const GL_PATCHES: GLenum = 0x000E;

attr_enum!(
    pub enum GlDrawMode {
        Points = GL_POINTS,
        LineStrip = GL_LINE_STRIP,
        LineLoop = GL_LINE_LOOP,
        Lines = GL_LINES,
        LineStripAdjacency = GL_LINE_STRIP_ADJACENCY,
        LinesAdjacency = GL_LINES_ADJACENCY,
        TriangleStrip = GL_TRIANGLE_STRIP,
        TriangleFan = GL_TRIANGLE_FAN,
        Triangles = GL_TRIANGLES,
        TriangleStripAdjacency = GL_TRIANGLE_STRIP_ADJACENCY,
        TrianglesAdjacency = GL_TRIANGLES_ADJACENCY,
        Patches = GL_PATCHES,
    }
);

pub const GL_BLEND: GLenum = 0x0BE2;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_DITHER: GLenum = 0x0BD0;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const GL_SAMPLE_MASK: GLenum = 0x8E51;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
pub const GL_STENCIL_TEST: GLenum = 0x0B90;

attr_enum!(
    pub enum GlCap {
        Blend = GL_BLEND,
        CullFace = GL_CULL_FACE,
        DepthTest = GL_DEPTH_TEST,
        Dither = GL_DITHER,
        PolygonOffsetFill = GL_POLYGON_OFFSET_FILL,
        PrimitiveRestartFixedIndex = GL_PRIMITIVE_RESTART_FIXED_INDEX,
        RasterizerDiscard = GL_RASTERIZER_DISCARD,
        SampleAlphaToCoverage = GL_SAMPLE_ALPHA_TO_COVERAGE,
        SampleCoverage = GL_SAMPLE_COVERAGE,
        SampleMask = GL_SAMPLE_MASK,
        ScissorTest = GL_SCISSOR_TEST,
        StencilTest = GL_STENCIL_TEST,
    }
);

pub const GL_ZERO: GLenum = 0x0;
pub const GL_ONE: GLenum = 0x1;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;

attr_enum!(
    pub enum GlBlendFact {
        Zero = GL_ZERO,
        One = GL_ONE,
        SrcColor = GL_SRC_COLOR,
        OneMinusSrcColor = GL_ONE_MINUS_SRC_COLOR,
        DstColor = GL_DST_COLOR,
        OneMinusDstColor = GL_ONE_MINUS_DST_COLOR,
        SrcAlpha = GL_SRC_ALPHA,
        OneMinusSrcAlpha = GL_ONE_MINUS_SRC_ALPHA,
        DstAlpha = GL_DST_ALPHA,
        OneMinusDstAlpha = GL_ONE_MINUS_DST_ALPHA,
        ConstantColor = GL_CONSTANT_COLOR,
        OneMinusConstantColor = GL_ONE_MINUS_CONSTANT_COLOR,
        ConstantAlpha = GL_CONSTANT_ALPHA,
        OneMinusConstantAlpha = GL_ONE_MINUS_CONSTANT_ALPHA,
        SrcAlphaSaturate = GL_SRC_ALPHA_SATURATE,
    }
);

pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_ALWAYS: GLenum = 0x0207;

attr_enum!(
    pub enum GlStencilFunc {
        Never = GL_NEVER,
        Less = GL_LESS,
        LessOrEqual = GL_LEQUAL,
        Greater = GL_GREATER,
        GreaterOrEqual = GL_GEQUAL,
        Equal = GL_EQUAL,
        NotEqual = GL_NOTEQUAL,
        Always = GL_ALWAYS,
    }
);

pub const GL_KEEP: GLenum = 0x1E00;
pub const GL_REPLACE: GLenum = 0x1E01;
pub const GL_INCR: GLenum = 0x1E02;
pub const GL_INCR_WRAP: GLenum = 0x8507;
pub const GL_DECR: GLenum = 0x1E03;
pub const GL_DECR_WRAP: GLenum = 0x8508;
pub const GL_INVERT: GLenum = 0x150A;

attr_enum!(
    pub enum GlStencilOp {
        Keep = GL_KEEP,
        Zero = GL_ZERO,
        Replace = GL_REPLACE,
        Incr = GL_INCR,
        IncrWrap = GL_INCR_WRAP,
        Decr = GL_DECR,
        DecrWrap = GL_DECR_WRAP,
        Invert = GL_INVERT,
    }
);
