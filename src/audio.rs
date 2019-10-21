use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{OggFormat, output::Output, Source, SourceHandle},
    ecs::prelude::{World, WorldExt},
};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

pub struct Sounds {
    pub bounce_sfx: SourceHandle,
    pub score_sfx: SourceHandle,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(
        file,
        OggFormat,
        (),
        &world.read_resource(),
    )
}

pub fn init_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            bounce_sfx: load_audio_track(&loader, world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, world, SCORE_SOUND),
        }
    };

    world.insert(sound_effects);
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.);
        }
    }
}