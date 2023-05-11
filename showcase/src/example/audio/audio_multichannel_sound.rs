/*******************************************************************************************
*
*   raylib [audio] example - Multichannel sound playing
*
*   This example has been created using raylib 2.6 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(
        thread,
        "raylib [audio] example - Multichannel sound playing",
    );

    let mut audio = RaylibAudio::init_audio_device(); // Initialize audio device

    let mut fxWav = Sound::load_sound("original/audio/resources/sound.wav").unwrap(); // Load WAV audio file
    let fxOgg = Sound::load_sound("original/audio/resources/tanatana.ogg").unwrap(); // Load OGG audio file

    audio.set_sound_volume(&mut fxWav, 0.2);

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER)
            {

                audio.play_sound(&fxWav); // Play a new wav sound instance
            }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
            {

                audio.play_sound(&fxOgg); // Play a new ogg sound instance
            }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("MULTICHANNEL SOUND PLAYING", 20, 20, 20, Color::GRAY);
        d.draw_text("Press SPACE to play new ogg instance!", 200, 120, 20, Color::LIGHTGRAY);
        d.draw_text("Press ENTER to play new wav instance!", 200, 180, 20, Color::LIGHTGRAY);

        let sounds_playing = (audio.is_sound_playing(&fxWav) as usize) + (audio.is_sound_playing(&fxOgg) as usize);

        d.draw_text(&format!("CONCURRENT SOUNDS PLAYING: {:02}", sounds_playing), 220, 280, 20,Color::RED);

        //----------------------------------------------------------------------------------
    },
    );
}
