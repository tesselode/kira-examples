use std::{error::Error, io::stdin};

use kira::{
	manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use rand::{thread_rng, Rng};

fn main() -> Result<(), Box<dyn Error>> {
	let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
	let sound_data = StaticSoundData::from_file("assets/blip.ogg", StaticSoundSettings::default())?;
	let mut rng = thread_rng();

	println!("Press enter to play a sound");
	loop {
		wait_for_enter_press()?;
		let sound = sound_data.with_modified_settings(|s| {
			s.volume(rng.gen_range(0.5..=1.0))
				.playback_rate(rng.gen_range(0.5..=1.5))
		});
		manager.play(sound)?;
	}
}

fn wait_for_enter_press() -> Result<(), Box<dyn Error>> {
	stdin().read_line(&mut "".into())?;
	Ok(())
}
