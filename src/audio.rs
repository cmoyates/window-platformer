use once_cell::sync::Lazy;
use sfml::{audio::SoundBuffer, SfBox};

const DEATH_SFX_DATA: &'static [u8] = include_bytes!("../assets/audio/Explosion 12.ogg");
pub static mut DEATH_SFX: Lazy<SfBox<SoundBuffer>> =
    Lazy::new(|| SoundBuffer::from_memory(DEATH_SFX_DATA).unwrap());

const LEVEL_COMPLETE_SFX_DATA: &'static [u8] = include_bytes!("../assets/audio/Powerup 22.ogg");
pub static mut LEVEL_COMPLETE_SFX: Lazy<SfBox<SoundBuffer>> =
    Lazy::new(|| SoundBuffer::from_memory(LEVEL_COMPLETE_SFX_DATA).unwrap());

const JUMP_SFX_DATA: &'static [u8] = include_bytes!("../assets/audio/Jump 47.ogg");
pub static mut JUMP_SFX: Lazy<SfBox<SoundBuffer>> =
    Lazy::new(|| SoundBuffer::from_memory(JUMP_SFX_DATA).unwrap());
