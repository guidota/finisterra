use std::collections::HashMap;

use bevy_ecs::schedule::{Schedule, ScheduleLabel, Schedules};
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
use roma::render::renderer::{LayoutKind, PhysicalSize};
use roma::render::texture::Texture;
use roma::{assets, render::renderer};
use roma::{run, BindGroupLayout, Event, Game, RenderPipeline, Resources};

mod render;
mod startup;

struct Finisterra {
    world: World,
}

#[derive(Resource)]
pub struct Renderer {
    pub inner: renderer::Renderer,
}

#[derive(Resource)]
pub struct RenderData {
    pub textures: HashMap<String, Texture>,
    pub layouts: HashMap<LayoutKind, BindGroupLayout>,
    pub pipelines: HashMap<LayoutKind, RenderPipeline>,
}

pub struct Textures {}

#[derive(Resource)]
pub struct Assets {
    pub inner: assets::Assets,
}

#[derive(ScheduleLabel, PartialEq, Eq, Debug, Clone, Hash)]
enum FinisterraSchedule {
    Startup,
    Input,
    Update,
    Render,
    PostUpdate,
}

impl Finisterra {
    fn new(resources: Resources) -> Self {
        let mut world = World::new();
        world.insert_resource(Renderer {
            inner: resources.renderer,
        });
        world.insert_resource(RenderData {
            textures: HashMap::new(),
            layouts: HashMap::new(),
            pipelines: HashMap::new(),
        });
        world.insert_resource(Assets {
            inner: resources.assets,
        });
        world.insert_resource(Schedules::default());
        let mut schedules: HashMap<FinisterraSchedule, Schedule> = HashMap::new();

        schedules
            .entry(FinisterraSchedule::Startup)
            .or_default()
            .add_system(startup::systems::spawn_sprite)
            .add_system(startup::systems::prepare_pipelines)
            .run(&mut world);

        schedules.entry(FinisterraSchedule::Input).or_default();
        schedules.entry(FinisterraSchedule::PostUpdate).or_default();
        schedules
            .entry(FinisterraSchedule::Update)
            .or_default()
            .add_system(render::systems::prepare);
        schedules
            .entry(FinisterraSchedule::Render)
            .or_default()
            .add_system(render::systems::insert_sprite_buffers)
            .add_system(render::systems::update_sprite_buffers)
            .add_system(render::systems::render);

        for schedule in schedules {
            world.add_schedule(schedule.1, schedule.0);
        }
        Self { world }
    }
}

impl Game for Finisterra {
    fn new(resources: roma::Resources) -> Self {
        Self::new(resources)
    }

    fn input<T>(&mut self, _event: Event<T>) {
        self.world.run_schedule(FinisterraSchedule::Input);
    }

    fn update(&mut self) {
        self.world.run_schedule(FinisterraSchedule::Update);
    }

    fn render(&mut self) {
        self.world.run_schedule(FinisterraSchedule::Render);
    }

    fn post_update(&mut self) {
        self.world.run_schedule(FinisterraSchedule::PostUpdate);
    }

    fn resize(&mut self, physical_size: PhysicalSize<u32>, scale_factor: f64) {
        if let Some(mut renderer) = self.world.get_resource_mut::<Renderer>() {
            renderer.inner.resize(physical_size, scale_factor);
        }
    }
}

fn main() {
    pollster::block_on(run::<Finisterra>())
}
