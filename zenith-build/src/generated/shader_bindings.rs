// File automatically generated by wgsl_bindgen^
//
// ^ wgsl_bindgen version 0.20.1
// Changes made to this file will not be saved.
// SourceHash: 3fe42aefc12f1158d757ae31b0d728620c15dda041f49c9a76768c29197ddfb1

#![allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderEntry {
    Triangle,
    Mesh,
}
impl ShaderEntry {
    pub fn create_pipeline_layout(&self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        match self {
            Self::Triangle => triangle::create_pipeline_layout(device),
            Self::Mesh => mesh::create_pipeline_layout(device),
        }
    }
    pub fn create_shader_module_relative_path(
        &self,
        device: &wgpu::Device,
        base_dir: &str,
        entry_point: ShaderEntry,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
        load_file: impl Fn(&str) -> Result<String, std::io::Error>,
    ) -> Result<wgpu::ShaderModule, naga_oil::compose::ComposerError> {
        match self {
            Self::Triangle => triangle::create_shader_module_relative_path(
                device,
                base_dir,
                *self,
                shader_defs,
                load_file,
            ),
            Self::Mesh => mesh::create_shader_module_relative_path(
                device,
                base_dir,
                *self,
                shader_defs,
                load_file,
            ),
        }
    }
    pub fn relative_path(&self) -> &'static str {
        match self {
            Self::Triangle => triangle::SHADER_ENTRY_PATH,
            Self::Mesh => mesh::SHADER_ENTRY_PATH,
        }
    }
}
mod _root {
    pub use super::*;
    #[doc = r" Visits and processes all shader files in a dependency tree."]
    #[doc = r""]
    #[doc = r" This function traverses the shader dependency tree and calls the visitor function"]
    #[doc = r" for each file encountered. This allows for custom processing like hot reloading,"]
    #[doc = r" caching, or debugging."]
    #[doc = r""]
    #[doc = r" # Arguments"]
    #[doc = r""]
    #[doc = r" * `base_dir` - The base directory for resolving relative paths"]
    #[doc = r" * `entry_point` - The shader entry point to start traversal from"]
    #[doc = r" * `shader_defs` - Shader defines to be used during processing"]
    #[doc = r" * `load_file` - Function to load file contents from a path"]
    #[doc = r" * `visitor` - Function called for each file with (file_path, file_content)"]
    #[doc = r""]
    #[doc = r" # Returns"]
    #[doc = r""]
    #[doc = r" Returns `Ok(())` if all files were processed successfully, or an error string."]
    pub fn visit_shader_files(
        base_dir: &str,
        entry_point: ShaderEntry,
        load_file: impl Fn(&str) -> Result<String, std::io::Error>,
        mut visitor: impl FnMut(&str, &str),
    ) -> Result<(), String> {
        fn visit_dependencies_recursive(
            base_dir: &str,
            source: &str,
            current_path: &str,
            load_file: &impl Fn(&str) -> Result<String, std::io::Error>,
            visitor: &mut impl FnMut(&str, &str),
            visited: &mut std::collections::HashSet<String>,
        ) -> Result<(), String> {
            let (_, imports, _) = naga_oil::compose::get_preprocessor_data(source);
            for import in imports {
                let import_path = if import.import.starts_with('\"') {
                    import
                        .import
                        .chars()
                        .skip(1)
                        .take_while(|c| *c != '\"')
                        .collect::<String>()
                } else {
                    let module_path = if let Some(double_colon_pos) = import.import.find("::") {
                        &import.import[..double_colon_pos]
                    } else {
                        &import.import
                    };
                    format!("{module_path}.wgsl")
                };
                let full_import_path =
                    if import_path.starts_with('/') || import_path.starts_with('\\') {
                        format!("{base_dir}{import_path}")
                    } else {
                        std::path::Path::new(base_dir)
                            .join(import_path)
                            .display()
                            .to_string()
                    };
                if visited.contains(&full_import_path) {
                    continue;
                }
                visited.insert(full_import_path.clone());
                let import_source = load_file(&full_import_path)
                    .map_err(|e| format!("Failed to load {full_import_path}: {e}"))?;
                visitor(&full_import_path, &import_source);
                visit_dependencies_recursive(
                    base_dir,
                    &import_source,
                    full_import_path.trim_start_matches(&format!("{base_dir}/")),
                    load_file,
                    visitor,
                    visited,
                )?;
            }
            Ok(())
        }
        let entry_path = format!("{}/{}", base_dir, entry_point.relative_path());
        let entry_source = load_file(&entry_path)
            .map_err(|e| format!("Failed to load entry point {entry_path}: {e}"))?;
        visitor(&entry_path, &entry_source);
        let mut visited = std::collections::HashSet::new();
        visit_dependencies_recursive(
            base_dir,
            &entry_source,
            entry_point.relative_path(),
            &load_file,
            &mut visitor,
            &mut visited,
        )?;
        Ok(())
    }
    pub fn load_naga_module_from_path(
        base_dir: &str,
        entry_point: ShaderEntry,
        composer: &mut naga_oil::compose::Composer,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
        load_file: impl Fn(&str) -> Result<String, std::io::Error>,
    ) -> Result<wgpu::naga::Module, String> {
        let mut files = std::collections::HashMap::<String, String>::new();
        visit_shader_files(
            base_dir,
            entry_point,
            &load_file,
            |file_path, file_content| {
                files.insert(file_path.to_string(), file_content.to_string());
            },
        )?;
        let entry_path = format!("{}/{}", base_dir, entry_point.relative_path());
        for (file_path, file_content) in &files {
            if *file_path == entry_path {
                continue;
            }
            let relative_path = file_path.trim_start_matches(&format!("{base_dir}/"));
            let as_name = std::path::Path::new(relative_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());
            composer
                .add_composable_module(naga_oil::compose::ComposableModuleDescriptor {
                    source: file_content,
                    file_path: relative_path,
                    language: naga_oil::compose::ShaderLanguage::Wgsl,
                    shader_defs: shader_defs.clone(),
                    as_name,
                    ..Default::default()
                })
                .map_err(|e| format!("Failed to add composable module: {e}"))?;
        }
        let entry_source = files
            .get(&entry_path)
            .ok_or_else(|| format!("Entry point file not found: {entry_path}"))?;
        composer
            .make_naga_module(naga_oil::compose::NagaModuleDescriptor {
                source: entry_source,
                file_path: entry_point.relative_path(),
                shader_defs,
                ..Default::default()
            })
            .map_err(|e| format!("Failed to create final module: {e}"))
    }
    pub trait SetBindGroup {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        );
    }
    impl SetBindGroup for wgpu::RenderPass<'_> {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        ) {
            self.set_bind_group(index, bind_group, offsets);
        }
    }
    impl SetBindGroup for wgpu::RenderBundleEncoder<'_> {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        ) {
            self.set_bind_group(index, bind_group, offsets);
        }
    }
}
pub mod layout_asserts {
    use super::{_root, _root::*};
    const WGSL_BASE_TYPE_ASSERTS: () = {
        assert!(std::mem::size_of::<glam::IVec2>() == 8);
        assert!(std::mem::align_of::<glam::IVec2>() == 4);
        assert!(std::mem::size_of::<glam::IVec3>() == 12);
        assert!(std::mem::align_of::<glam::IVec3>() == 4);
        assert!(std::mem::size_of::<glam::IVec4>() == 16);
        assert!(std::mem::align_of::<glam::IVec4>() == 4);
        assert!(std::mem::size_of::<glam::UVec2>() == 8);
        assert!(std::mem::align_of::<glam::UVec2>() == 4);
        assert!(std::mem::size_of::<glam::UVec3>() == 12);
        assert!(std::mem::align_of::<glam::UVec3>() == 4);
        assert!(std::mem::size_of::<glam::UVec4>() == 16);
        assert!(std::mem::align_of::<glam::UVec4>() == 4);
        assert!(std::mem::size_of::<glam::Vec2>() == 8);
        assert!(std::mem::align_of::<glam::Vec2>() == 4);
        assert!(std::mem::size_of::<glam::Vec3>() == 12);
        assert!(std::mem::align_of::<glam::Vec3>() == 4);
        assert!(std::mem::size_of::<glam::Vec4>() == 16);
        assert!(std::mem::align_of::<glam::Vec4>() == 16);
        assert!(std::mem::size_of::<glam::Mat2>() == 16);
        assert!(std::mem::align_of::<glam::Mat2>() == 16);
        assert!(std::mem::size_of::<glam::Mat3A>() == 48);
        assert!(std::mem::align_of::<glam::Mat3A>() == 16);
        assert!(std::mem::size_of::<glam::Mat4>() == 64);
        assert!(std::mem::align_of::<glam::Mat4>() == 16);
    };
    const TRIANGLE_UNIFORMS_ASSERTS: () = {
        assert!(std::mem::offset_of!(triangle::Uniforms, transform) == 0);
        assert!(std::mem::size_of::<triangle::Uniforms>() == 64);
    };
    const MESH_VIEW_UNIFORMS_ASSERTS: () = {
        assert!(std::mem::offset_of!(mesh::ViewUniforms, view_proj) == 0);
        assert!(std::mem::size_of::<mesh::ViewUniforms>() == 64);
    };
    const MESH_MODEL_UNIFORMS_ASSERTS: () = {
        assert!(std::mem::offset_of!(mesh::ModelUniforms, model) == 0);
        assert!(std::mem::offset_of!(mesh::ModelUniforms, base_color) == 64);
        assert!(std::mem::size_of::<mesh::ModelUniforms>() == 80);
    };
}
pub mod triangle {
    use super::{_root, _root::*};
    #[repr(C, align(16))]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Uniforms {
        #[doc = "offset: 0, size: 64, type: `mat4x4<f32>`"]
        pub transform: glam::Mat4,
    }
    impl Uniforms {
        pub const fn new(transform: glam::Mat4) -> Self {
            Self { transform }
        }
    }
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct VertexInput {
        pub position: glam::Vec3,
        pub color: glam::Vec3,
    }
    impl VertexInput {
        pub const fn new(position: glam::Vec3, color: glam::Vec3) -> Self {
            Self { position, color }
        }
    }
    impl VertexInput {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(Self, position) as u64,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(Self, color) as u64,
                shader_location: 1,
            },
        ];
        pub const fn vertex_buffer_layout(
            step_mode: wgpu::VertexStepMode,
        ) -> wgpu::VertexBufferLayout<'static> {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Self>() as u64,
                step_mode,
                attributes: &Self::VERTEX_ATTRIBUTES,
            }
        }
    }
    pub const ENTRY_VS_MAIN: &str = "vs_main";
    pub const ENTRY_FS_MAIN: &str = "fs_main";
    #[derive(Debug)]
    pub struct VertexEntry<const N: usize> {
        pub entry_point: &'static str,
        pub buffers: [wgpu::VertexBufferLayout<'static>; N],
        pub constants: Vec<(&'static str, f64)>,
    }
    pub fn vertex_state<'a, const N: usize>(
        module: &'a wgpu::ShaderModule,
        entry: &'a VertexEntry<N>,
    ) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module,
            entry_point: Some(entry.entry_point),
            buffers: &entry.buffers,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &entry.constants,
                ..Default::default()
            },
        }
    }
    pub fn vs_main_entry(vertex_input: wgpu::VertexStepMode) -> VertexEntry<1> {
        VertexEntry {
            entry_point: ENTRY_VS_MAIN,
            buffers: [VertexInput::vertex_buffer_layout(vertex_input)],
            constants: Default::default(),
        }
    }
    #[derive(Debug)]
    pub struct FragmentEntry<const N: usize> {
        pub entry_point: &'static str,
        pub targets: [Option<wgpu::ColorTargetState>; N],
        pub constants: Vec<(&'static str, f64)>,
    }
    pub fn fragment_state<'a, const N: usize>(
        module: &'a wgpu::ShaderModule,
        entry: &'a FragmentEntry<N>,
    ) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module,
            entry_point: Some(entry.entry_point),
            targets: &entry.targets,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &entry.constants,
                ..Default::default()
            },
        }
    }
    pub fn fs_main_entry(targets: [Option<wgpu::ColorTargetState>; 1]) -> FragmentEntry<1> {
        FragmentEntry {
            entry_point: ENTRY_FS_MAIN,
            targets,
            constants: Default::default(),
        }
    }
    #[derive(Debug)]
    pub struct WgpuBindGroup0EntriesParams<'a> {
        pub uniforms: wgpu::BufferBinding<'a>,
    }
    #[derive(Clone, Debug)]
    pub struct WgpuBindGroup0Entries<'a> {
        pub uniforms: wgpu::BindGroupEntry<'a>,
    }
    impl<'a> WgpuBindGroup0Entries<'a> {
        pub fn new(params: WgpuBindGroup0EntriesParams<'a>) -> Self {
            Self {
                uniforms: wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(params.uniforms),
                },
            }
        }
        pub fn into_array(self) -> [wgpu::BindGroupEntry<'a>; 1] {
            [self.uniforms]
        }
        pub fn collect<B: FromIterator<wgpu::BindGroupEntry<'a>>>(self) -> B {
            self.into_array().into_iter().collect()
        }
    }
    #[derive(Debug)]
    pub struct WgpuBindGroup0(wgpu::BindGroup);
    impl WgpuBindGroup0 {
        pub const LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor<'static> =
            wgpu::BindGroupLayoutDescriptor {
                label: Some("Triangle::BindGroup0::LayoutDescriptor"),
                entries: &[
                    #[doc = " @binding(0): \"uniforms\""]
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                                _root::triangle::Uniforms,
                            >(
                            )
                                as _),
                        },
                        count: None,
                    },
                ],
            };
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&Self::LAYOUT_DESCRIPTOR)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: WgpuBindGroup0Entries) -> Self {
            let bind_group_layout = Self::get_bind_group_layout(device);
            let entries = bindings.into_array();
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Triangle::BindGroup0"),
                layout: &bind_group_layout,
                entries: &entries,
            });
            Self(bind_group)
        }
        pub fn set(&self, pass: &mut impl SetBindGroup) {
            pass.set_bind_group(0, &self.0, &[]);
        }
    }
    #[doc = " Bind groups can be set individually using their set(render_pass) method, or all at once using `WgpuBindGroups::set`."]
    #[doc = " For optimal performance with many draw calls, it's recommended to organize bindings into bind groups based on update frequency:"]
    #[doc = "   - Bind group 0: Least frequent updates (e.g. per frame resources)"]
    #[doc = "   - Bind group 1: More frequent updates"]
    #[doc = "   - Bind group 2: More frequent updates"]
    #[doc = "   - Bind group 3: Most frequent updates (e.g. per draw resources)"]
    #[derive(Debug, Copy, Clone)]
    pub struct WgpuBindGroups<'a> {
        pub bind_group0: &'a WgpuBindGroup0,
    }
    impl<'a> WgpuBindGroups<'a> {
        pub fn set(&self, pass: &mut impl SetBindGroup) {
            self.bind_group0.set(pass);
        }
    }
    #[derive(Debug)]
    pub struct WgpuPipelineLayout;
    impl WgpuPipelineLayout {
        pub fn bind_group_layout_entries(
            entries: [wgpu::BindGroupLayout; 1],
        ) -> [wgpu::BindGroupLayout; 1] {
            entries
        }
    }
    pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle::PipelineLayout"),
            bind_group_layouts: &[&WgpuBindGroup0::get_bind_group_layout(device)],
            push_constant_ranges: &[],
        })
    }
    pub const SHADER_ENTRY_PATH: &str = "triangle.wgsl";
    pub fn create_shader_module_relative_path(
        device: &wgpu::Device,
        base_dir: &str,
        entry_point: ShaderEntry,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
        load_file: impl Fn(&str) -> Result<String, std::io::Error>,
    ) -> Result<wgpu::ShaderModule, naga_oil::compose::ComposerError> {
        let mut composer = naga_oil::compose::Composer::default();
        let module = load_naga_module_from_path(
            base_dir,
            entry_point,
            &mut composer,
            shader_defs,
            load_file,
        )
        .map_err(|e| naga_oil::compose::ComposerError {
            inner: naga_oil::compose::ComposerErrorInner::ImportNotFound(e, 0),
            source: naga_oil::compose::ErrSource::Constructing {
                path: "load_naga_module_from_path".to_string(),
                source: "Generated code".to_string(),
                offset: 0,
            },
        })?;
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle.wgsl"),
            source: wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(module)),
        });
        Ok(shader_module)
    }
}
pub mod bytemuck_impls {
    use super::{_root, _root::*};
    unsafe impl bytemuck::Zeroable for triangle::Uniforms {}
    unsafe impl bytemuck::Pod for triangle::Uniforms {}
    unsafe impl bytemuck::Zeroable for triangle::VertexInput {}
    unsafe impl bytemuck::Pod for triangle::VertexInput {}
    unsafe impl bytemuck::Zeroable for mesh::ViewUniforms {}
    unsafe impl bytemuck::Pod for mesh::ViewUniforms {}
    unsafe impl bytemuck::Zeroable for mesh::ModelUniforms {}
    unsafe impl bytemuck::Pod for mesh::ModelUniforms {}
    unsafe impl bytemuck::Zeroable for mesh::VertexInput {}
    unsafe impl bytemuck::Pod for mesh::VertexInput {}
}
pub mod mesh {
    use super::{_root, _root::*};
    #[repr(C, align(16))]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ViewUniforms {
        #[doc = "offset: 0, size: 64, type: `mat4x4<f32>`"]
        pub view_proj: glam::Mat4,
    }
    impl ViewUniforms {
        pub const fn new(view_proj: glam::Mat4) -> Self {
            Self { view_proj }
        }
    }
    #[repr(C, align(16))]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ModelUniforms {
        #[doc = "offset: 0, size: 64, type: `mat4x4<f32>`"]
        pub model: glam::Mat4,
        #[doc = "offset: 64, size: 12, type: `vec3<f32>`"]
        pub base_color: glam::Vec3,
        pub _pad_base_color: [u8; 0x4],
    }
    impl ModelUniforms {
        pub const fn new(model: glam::Mat4, base_color: glam::Vec3) -> Self {
            Self {
                model,
                base_color,
                _pad_base_color: [0; 0x4],
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ModelUniformsInit {
        pub model: glam::Mat4,
        pub base_color: glam::Vec3,
    }
    impl ModelUniformsInit {
        pub fn build(&self) -> ModelUniforms {
            ModelUniforms {
                model: self.model,
                base_color: self.base_color,
                _pad_base_color: [0; 0x4],
            }
        }
    }
    impl From<ModelUniformsInit> for ModelUniforms {
        fn from(data: ModelUniformsInit) -> Self {
            data.build()
        }
    }
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct VertexInput {
        pub position: glam::Vec3,
        pub normal: glam::Vec3,
        pub tex_coord: glam::Vec2,
    }
    impl VertexInput {
        pub const fn new(position: glam::Vec3, normal: glam::Vec3, tex_coord: glam::Vec2) -> Self {
            Self {
                position,
                normal,
                tex_coord,
            }
        }
    }
    impl VertexInput {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(Self, position) as u64,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(Self, normal) as u64,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(Self, tex_coord) as u64,
                shader_location: 2,
            },
        ];
        pub const fn vertex_buffer_layout(
            step_mode: wgpu::VertexStepMode,
        ) -> wgpu::VertexBufferLayout<'static> {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Self>() as u64,
                step_mode,
                attributes: &Self::VERTEX_ATTRIBUTES,
            }
        }
    }
    pub const ENTRY_VS_MAIN: &str = "vs_main";
    pub const ENTRY_FS_MAIN: &str = "fs_main";
    #[derive(Debug)]
    pub struct VertexEntry<const N: usize> {
        pub entry_point: &'static str,
        pub buffers: [wgpu::VertexBufferLayout<'static>; N],
        pub constants: Vec<(&'static str, f64)>,
    }
    pub fn vertex_state<'a, const N: usize>(
        module: &'a wgpu::ShaderModule,
        entry: &'a VertexEntry<N>,
    ) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module,
            entry_point: Some(entry.entry_point),
            buffers: &entry.buffers,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &entry.constants,
                ..Default::default()
            },
        }
    }
    pub fn vs_main_entry(vertex_input: wgpu::VertexStepMode) -> VertexEntry<1> {
        VertexEntry {
            entry_point: ENTRY_VS_MAIN,
            buffers: [VertexInput::vertex_buffer_layout(vertex_input)],
            constants: Default::default(),
        }
    }
    #[derive(Debug)]
    pub struct FragmentEntry<const N: usize> {
        pub entry_point: &'static str,
        pub targets: [Option<wgpu::ColorTargetState>; N],
        pub constants: Vec<(&'static str, f64)>,
    }
    pub fn fragment_state<'a, const N: usize>(
        module: &'a wgpu::ShaderModule,
        entry: &'a FragmentEntry<N>,
    ) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module,
            entry_point: Some(entry.entry_point),
            targets: &entry.targets,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &entry.constants,
                ..Default::default()
            },
        }
    }
    pub fn fs_main_entry(targets: [Option<wgpu::ColorTargetState>; 1]) -> FragmentEntry<1> {
        FragmentEntry {
            entry_point: ENTRY_FS_MAIN,
            targets,
            constants: Default::default(),
        }
    }
    #[derive(Debug)]
    pub struct WgpuBindGroup0EntriesParams<'a> {
        pub view: wgpu::BufferBinding<'a>,
        pub model: wgpu::BufferBinding<'a>,
        pub base_color_texture: &'a wgpu::TextureView,
        pub base_color_sampler: &'a wgpu::Sampler,
    }
    #[derive(Clone, Debug)]
    pub struct WgpuBindGroup0Entries<'a> {
        pub view: wgpu::BindGroupEntry<'a>,
        pub model: wgpu::BindGroupEntry<'a>,
        pub base_color_texture: wgpu::BindGroupEntry<'a>,
        pub base_color_sampler: wgpu::BindGroupEntry<'a>,
    }
    impl<'a> WgpuBindGroup0Entries<'a> {
        pub fn new(params: WgpuBindGroup0EntriesParams<'a>) -> Self {
            Self {
                view: wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(params.view),
                },
                model: wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(params.model),
                },
                base_color_texture: wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(params.base_color_texture),
                },
                base_color_sampler: wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(params.base_color_sampler),
                },
            }
        }
        pub fn into_array(self) -> [wgpu::BindGroupEntry<'a>; 4] {
            [
                self.view,
                self.model,
                self.base_color_texture,
                self.base_color_sampler,
            ]
        }
        pub fn collect<B: FromIterator<wgpu::BindGroupEntry<'a>>>(self) -> B {
            self.into_array().into_iter().collect()
        }
    }
    #[derive(Debug)]
    pub struct WgpuBindGroup0(wgpu::BindGroup);
    impl WgpuBindGroup0 {
        pub const LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor<'static> =
            wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh::BindGroup0::LayoutDescriptor"),
                entries: &[
                    #[doc = " @binding(0): \"view\""]
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                                _root::mesh::ViewUniforms,
                            >(
                            )
                                as _),
                        },
                        count: None,
                    },
                    #[doc = " @binding(1): \"model\""]
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                                _root::mesh::ModelUniforms,
                            >(
                            )
                                as _),
                        },
                        count: None,
                    },
                    #[doc = " @binding(2): \"base_color_texture\""]
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    #[doc = " @binding(3): \"base_color_sampler\""]
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            };
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&Self::LAYOUT_DESCRIPTOR)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: WgpuBindGroup0Entries) -> Self {
            let bind_group_layout = Self::get_bind_group_layout(device);
            let entries = bindings.into_array();
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Mesh::BindGroup0"),
                layout: &bind_group_layout,
                entries: &entries,
            });
            Self(bind_group)
        }
        pub fn set(&self, pass: &mut impl SetBindGroup) {
            pass.set_bind_group(0, &self.0, &[]);
        }
    }
    #[doc = " Bind groups can be set individually using their set(render_pass) method, or all at once using `WgpuBindGroups::set`."]
    #[doc = " For optimal performance with many draw calls, it's recommended to organize bindings into bind groups based on update frequency:"]
    #[doc = "   - Bind group 0: Least frequent updates (e.g. per frame resources)"]
    #[doc = "   - Bind group 1: More frequent updates"]
    #[doc = "   - Bind group 2: More frequent updates"]
    #[doc = "   - Bind group 3: Most frequent updates (e.g. per draw resources)"]
    #[derive(Debug, Copy, Clone)]
    pub struct WgpuBindGroups<'a> {
        pub bind_group0: &'a WgpuBindGroup0,
    }
    impl<'a> WgpuBindGroups<'a> {
        pub fn set(&self, pass: &mut impl SetBindGroup) {
            self.bind_group0.set(pass);
        }
    }
    #[derive(Debug)]
    pub struct WgpuPipelineLayout;
    impl WgpuPipelineLayout {
        pub fn bind_group_layout_entries(
            entries: [wgpu::BindGroupLayout; 1],
        ) -> [wgpu::BindGroupLayout; 1] {
            entries
        }
    }
    pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Mesh::PipelineLayout"),
            bind_group_layouts: &[&WgpuBindGroup0::get_bind_group_layout(device)],
            push_constant_ranges: &[],
        })
    }
    pub const SHADER_ENTRY_PATH: &str = "mesh.wgsl";
    pub fn create_shader_module_relative_path(
        device: &wgpu::Device,
        base_dir: &str,
        entry_point: ShaderEntry,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
        load_file: impl Fn(&str) -> Result<String, std::io::Error>,
    ) -> Result<wgpu::ShaderModule, naga_oil::compose::ComposerError> {
        let mut composer = naga_oil::compose::Composer::default();
        let module = load_naga_module_from_path(
            base_dir,
            entry_point,
            &mut composer,
            shader_defs,
            load_file,
        )
        .map_err(|e| naga_oil::compose::ComposerError {
            inner: naga_oil::compose::ComposerErrorInner::ImportNotFound(e, 0),
            source: naga_oil::compose::ErrSource::Constructing {
                path: "load_naga_module_from_path".to_string(),
                source: "Generated code".to_string(),
                offset: 0,
            },
        })?;
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mesh.wgsl"),
            source: wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(module)),
        });
        Ok(shader_module)
    }
}
