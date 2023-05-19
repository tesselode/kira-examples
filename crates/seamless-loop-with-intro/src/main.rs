use std::{error::Error, io::stdin};

use kira::{
	manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

fn main() -> Result<(), Box<dyn Error>> {
	let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
	manager.play(StaticSoundData::from_file(
		"assets/drums.ogg",
		StaticSoundSettings::new().loop_region(3.6..6.0),
	)?)?;

	println!("Press enter to exit");
	wait_for_enter_press()?;

	Ok(())
}

fn wait_for_enter_press() -> Result<(), Box<dyn Error>> {
	stdin().read_line(&mut "".into())?;
	Ok(())
}
