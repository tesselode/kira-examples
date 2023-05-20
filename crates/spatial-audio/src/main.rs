use kira::{
	manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	spatial::{emitter::EmitterSettings, listener::ListenerSettings, scene::SpatialSceneSettings},
	tween::Tween,
};
use macroquad::prelude::*;

const MOVE_SPEED: f32 = 6.0;
const LOOK_SPEED: f32 = 0.005;
const WORLD_UP: Vec3 = vec3(0.0, 1.0, 0.0);
const EMITTER_POSITION: Vec3 = vec3(0.0, 1.0, -6.0);

fn conf() -> Conf {
	Conf {
		window_title: String::from("Macroquad"),
		window_width: 1260,
		window_height: 768,
		fullscreen: false,
		..Default::default()
	}
}

#[macroquad::main(conf)]
async fn main() {
	let mut camera_controller = CameraController::new();

	let mut last_mouse_position: Vec2 = mouse_position().into();

	let mut audio_manager =
		AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
	let mut spatial_scene = audio_manager
		.add_spatial_scene(SpatialSceneSettings::default())
		.unwrap();
	let emitter = spatial_scene
		.add_emitter(EMITTER_POSITION, EmitterSettings::default())
		.unwrap();
	let mut listener = spatial_scene
		.add_listener(
			camera_controller.position,
			camera_controller.orientation(),
			ListenerSettings::default(),
		)
		.unwrap();
	audio_manager
		.play(
			StaticSoundData::from_file(
				"assets/drums.ogg",
				StaticSoundSettings::new()
					.loop_region(3.6..6.0)
					.output_destination(&emitter),
			)
			.unwrap(),
		)
		.unwrap();

	loop {
		let delta_time = get_frame_time();

		if is_key_pressed(KeyCode::Escape) {
			break;
		}

		let mouse_position: Vec2 = mouse_position().into();
		let mouse_delta = mouse_position - last_mouse_position;
		last_mouse_position = mouse_position;
		camera_controller.update(delta_time, mouse_delta);
		listener
			.set_position(camera_controller.position, Tween::default())
			.unwrap();
		listener
			.set_orientation(camera_controller.orientation(), Tween::default())
			.unwrap();

		clear_background(LIGHTGRAY);

		// Going 3d!

		set_camera(&camera_controller.camera());

		draw_grid(20, 1., BLACK, GRAY);

		draw_cube_wires(EMITTER_POSITION, vec3(2., 2., 2.), GREEN);

		// Back to screen space, render some text

		set_default_camera();

		next_frame().await
	}
}

struct CameraController {
	position: Vec3,
	yaw: f32,
	pitch: f32,
}

impl CameraController {
	fn new() -> Self {
		Self {
			position: vec3(0.0, 1.0, 0.0),
			yaw: 0.0,
			pitch: 0.0,
		}
	}

	fn update(&mut self, delta_time: f32, mouse_delta: Vec2) {
		if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
			self.position += self.front() * delta_time * MOVE_SPEED;
		}
		if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
			self.position -= self.front() * delta_time * MOVE_SPEED;
		}
		if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
			self.position -= self.right() * delta_time * MOVE_SPEED;
		}
		if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
			self.position += self.right() * delta_time * MOVE_SPEED;
		}
		self.yaw -= mouse_delta.x * LOOK_SPEED;
		self.pitch -= mouse_delta.y * LOOK_SPEED;
		self.pitch = self.pitch.clamp(-1.5, 1.5);
	}

	fn orientation(&self) -> Quat {
		Quat::from_euler(EulerRot::XYZ, self.pitch, self.yaw, 0.0)
	}

	fn camera(&self) -> Camera3D {
		Camera3D {
			position: self.position,
			target: self.position + self.front(),
			up: WORLD_UP,
			..Default::default()
		}
	}

	fn front(&self) -> Vec3 {
		(Quat::from_euler(EulerRot::XYZ, self.pitch, self.yaw, 0.0) * Vec3::new(0.0, 0.0, -1.0))
			.normalize()
	}

	fn right(&self) -> Vec3 {
		self.front().cross(WORLD_UP).normalize()
	}
}
