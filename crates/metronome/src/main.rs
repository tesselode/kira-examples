use std::{error::Error, io::stdin, time::Duration};

use kira::{
	clock::{ClockSpeed, ClockTime},
	manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

fn main() -> Result<(), Box<dyn Error>> {
	let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
	let sound_data = StaticSoundData::from_file("assets/blip.ogg", StaticSoundSettings::default())?;
	let clock = manager.add_clock(ClockSpeed::TicksPerMinute(120.0))?;
	// queue up the first 2 metronome clicks
	manager.play(sound_data.with_modified_settings(|s| {
		s.playback_rate(2.0).start_time(ClockTime {
			clock: clock.id(),
			ticks: 0,
		})
	}))?;
	manager.play(sound_data.with_modified_settings(|s| {
		s.playback_rate(1.0).start_time(ClockTime {
			clock: clock.id(),
			ticks: 1,
		})
	}))?;

	println!("Press enter to start the metronome");
	wait_for_enter_press()?;
	clock.start()?;

	let mut previous_clock_time = clock.time();
	loop {
		std::thread::sleep(Duration::from_millis(10));
		let current_clock_time = clock.time();
		if current_clock_time.ticks > previous_clock_time.ticks {
			// whenever the clock ticks, queue up a metronome click for the next tick
			manager.play(sound_data.with_modified_settings(|s| {
				let playback_rate = if is_next_tick_beginning_of_measure(current_clock_time) {
					2.0
				} else {
					1.0
				};
				s.playback_rate(playback_rate).start_time(clock.time() + 1)
			}))?;
			previous_clock_time = current_clock_time;
		}
	}
}

fn is_next_tick_beginning_of_measure(current_clock_time: ClockTime) -> bool {
	(current_clock_time.ticks + 1) % 4 == 0
}

fn wait_for_enter_press() -> Result<(), Box<dyn Error>> {
	stdin().read_line(&mut "".into())?;
	Ok(())
}
