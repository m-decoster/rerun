use type_map::concurrent::{self, TypeMap};

use crate::{
    config::RenderContextConfig,
    global_bindings::GlobalBindings,
    renderer::Renderer,
    resource_managers::{MeshManager, TextureManager2D},
    resource_pools::WgpuResourcePools,
    FileResolver, FileServer, FileSystem, RecommendedFileResolver,
};

// ---

/// Any resource involving wgpu rendering which can be re-used across different scenes.
/// I.e. render pipelines, resource pools, etc.
pub struct RenderContext {
    pub(crate) shared_renderer_data: SharedRendererData,
    pub(crate) renderers: Renderers,
    pub(crate) resource_pools: WgpuResourcePools,
    pub(crate) resolver: RecommendedFileResolver,
    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
    pub(crate) err_tracker: std::sync::Arc<crate::error_tracker::ErrorTracker>,

    pub mesh_manager: MeshManager,
    pub texture_manager_2d: TextureManager2D,

    // TODO(andreas): Add frame/lifetime statistics, shared resources (e.g. "global" uniform buffer), ??
    frame_index: u64,
}

/// Immutable data that is shared between all [`Renderer`]
pub struct SharedRendererData {
    pub(crate) config: RenderContextConfig,

    /// Global bindings, always bound to 0 bind group slot zero.
    /// [`Renderer`] are not allowed to use bind group 0 themselves!
    pub(crate) global_bindings: GlobalBindings,
}

/// Struct owning *all* [`Renderer`].
/// [`Renderer`] are created lazily and stay around indefinitely.
pub(crate) struct Renderers {
    renderers: concurrent::TypeMap,
}

impl Renderers {
    pub fn get_or_create<Fs: FileSystem, R: 'static + Renderer + Send + Sync>(
        &mut self,
        shared_data: &SharedRendererData,
        resource_pools: &mut WgpuResourcePools,
        device: &wgpu::Device,
        resolver: &mut FileResolver<Fs>,
    ) -> &R {
        self.renderers.entry().or_insert_with(|| {
            crate::profile_scope!("create_renderer", std::any::type_name::<R>());
            R::create_renderer(shared_data, resource_pools, device, resolver)
        })
    }

    pub fn get<R: 'static + Renderer>(&self) -> Option<&R> {
        self.renderers.get::<R>()
    }
}

impl RenderContext {
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, config: RenderContextConfig) -> Self {
        let mut resource_pools = WgpuResourcePools::default();
        let global_bindings = GlobalBindings::new(&mut resource_pools, device);

        // Validate capabilities of the device.
        assert!(
            config.hardware_tier.limits().check_limits(&device.limits()),
            "The given device doesn't support the required limits for the given hardware tier {:?}.
            Required:
            {:?}
            Actual:
            {:?}",
            config.hardware_tier,
            config.hardware_tier.limits(),
            device.limits(),
        );
        assert!(
            device.features().contains(config.hardware_tier.features()),
            "The given device doesn't support the required features for the given hardware tier {:?}.
            Required:
            {:?}
            Actual:
            {:?}",
            config.hardware_tier,
            config.hardware_tier.features(),
            device.features(),
        );
        // Can't check downlevel feature flags since they sit on the adapter, not on the device.

        // In debug builds, make sure to catch all errors, never crash, and try to
        // always let the user find a way to return a poisoned pipeline back into a
        // sane state.
        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
        let err_tracker = {
            let err_tracker = std::sync::Arc::new(crate::error_tracker::ErrorTracker::default());
            device.on_uncaptured_error({
                let err_tracker = std::sync::Arc::clone(&err_tracker);
                move |err| err_tracker.handle_error(err)
            });
            err_tracker
        };

        RenderContext {
            shared_renderer_data: SharedRendererData {
                config,
                global_bindings,
            },

            renderers: Renderers {
                renderers: TypeMap::new(),
            },
            resource_pools,

            mesh_manager: MeshManager::default(),
            texture_manager_2d: TextureManager2D::default(),

            resolver: crate::new_recommended_file_resolver(),

            #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
            err_tracker,

            frame_index: 0,
        }
    }

    pub fn frame_maintenance(&mut self, device: &wgpu::Device) {
        // Tick the error tracker so that it knows when to reset!
        // Note that we're ticking on frame_maintenance rather than raw frames, which
        // makes a world of difference when we're in a poisoned state.
        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
        self.err_tracker.tick();

        // Clear the resolver cache before we start reloading shaders!
        self.resolver.clear();

        // The set of files on disk that were modified in any way since last frame,
        // ignoring deletions.
        // Always an empty set in release builds.
        let modified_paths = FileServer::get_mut(|fs| fs.collect(&mut self.resolver));
        if !modified_paths.is_empty() {
            re_log::debug!(?modified_paths, "got some filesystem events");
        }

        self.mesh_manager.frame_maintenance(self.frame_index);
        self.texture_manager_2d.frame_maintenance(self.frame_index);

        {
            let WgpuResourcePools {
                bind_group_layouts: _,
                bind_groups,
                pipeline_layouts,
                render_pipelines,
                samplers,
                shader_modules,
                textures,
                buffers,
            } = &mut self.resource_pools; // not all pools require maintenance

            // Render pipeline maintenance must come before shader module maintenance since
            // it registers them.
            render_pipelines.frame_maintenance(
                device,
                self.frame_index,
                shader_modules,
                pipeline_layouts,
            );

            shader_modules.frame_maintenance(
                device,
                &mut self.resolver,
                self.frame_index,
                &modified_paths,
            );

            // Bind group maintenance must come before texture/buffer maintenance since it
            // registers texture/buffer use
            bind_groups.frame_maintenance(self.frame_index, textures, buffers, samplers);

            textures.frame_maintenance(self.frame_index);
            buffers.frame_maintenance(self.frame_index);
        }

        self.frame_index += 1;
    }
}
