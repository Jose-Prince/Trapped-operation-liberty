use crate::AudioPlayer;
use crate::Framebuffer;
use crate::Color;
use crate::polygon::Polygon;
use crate::maze::{render, render3d, render_enemies_pos, render_enemy, draw_player_position, draw_enemies_position, draw_enemy_fov, minimap};
use crate::texture::Texture;
use crate::player::Player;
use crate::enemy::Enemy;


use std::time::{Duration, Instant};
use minifb::{Window, Key};
use image::GenericImageView;
use nalgebra_glm::Vec2;
use std::f32::consts::PI;


fn calculate_fps(start_time: Instant, frame_count: usize) -> f64 {
    let duration = start_time.elapsed().as_secs_f64();
    frame_count as f64 / duration
}

pub fn game_start(width: usize, height: usize, framebuffer: &mut Framebuffer, window: &mut Window) {
    let mut audio = AudioPlayer::new("Audio/Inicio.mp3", 0.5);
    
    audio.play();

    let begin_page = "textures/Inicio.png";
    
    let blink_interval = Duration::from_millis(200);
    let mut last_blink_time = Instant::now();
    let mut show_text = true;
    let mut enter_pressed = false;

    while window.is_open() && !enter_pressed && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();
        framebuffer.draw_image(&begin_page, width, height);
        
        if last_blink_time.elapsed() >= blink_interval {
            show_text = !show_text;
            last_blink_time = Instant::now();
        }

        if show_text {
            framebuffer.draw_text(width / 5, (4 * height) / 5 - 25, "Press ENTER to start game", Color::new(255, 255, 255), 70.0);
        }

        // Detectar cuando Enter se presiona por primera vez
        if window.is_key_down(minifb::Key::Enter) {
            enter_pressed = true;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    // Ahora esperar a que se suelte la tecla Enter
    while enter_pressed && window.is_open() {
        framebuffer.clear();

        // Continuar solo cuando la tecla Enter se ha soltado
        if !window.is_key_down(minifb::Key::Enter) {
            enter_pressed = false;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    audio.stop();

    level_selector(framebuffer, window, width, height);
}

pub fn level_selector(framebuffer: &mut Framebuffer, window: &mut Window, width: usize, height: usize) {
    let mut option = 0;

    // Definición de los polígonos de fondo para cada nivel
    let first_level_background: Vec<[isize; 2]> = vec![
        [0, 0],
        [0, height as isize],
        [width as isize / 3, height as isize],
        [width as isize / 3, 0],
    ];

    let second_level_background: Vec<[isize; 2]> = vec![
        [width as isize / 3, 0],
        [width as isize / 3, height as isize],
        [2 * (width as isize) / 3, height as isize],
        [2 * (width as isize) / 3, 0],
    ];

    let third_level_background: Vec<[isize; 2]> = vec![
        [2 * (width as isize) / 3, 0],
        [2 * (width as isize) / 3, height as isize],
        [width as isize, height as isize],
        [width as isize, 0],
    ];

    // Bucle principal de la selección de nivel
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();

        // Dibujar el fondo según la opción seleccionada
        match option {
            0 => framebuffer.polygon(&first_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            1 => framebuffer.polygon(&second_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            2 => framebuffer.polygon(&third_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            _ => println!("Invalid option"),
        }

        // Cambiar opción con las teclas de flecha
        if window.is_key_down(minifb::Key::Left) && option > 0 {
            option -= 1;
        } else if window.is_key_down(minifb::Key::Right) && option < 2 {
            option += 1;
        }

        // Dibujar los textos y las imágenes
        framebuffer.draw_text(width / 24 + 40, height / 5, "Level 1", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison1.jpeg", width / 4, height / 4, width / 24 - 1, 2 * height / 5);

        framebuffer.draw_text(2 * width / 5 + 30, height / 5, "Level 2", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison2.jpg", width / 4, height / 4, 2 * width / 5 - 25, 2 * height / 5);

        framebuffer.draw_text(4 * width / 6 + 85, height / 5, "Level 3", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison3.jpg", width / 4, height / 4, 4 * width / 6 + 45, 2 * height / 5);

        framebuffer.draw_text(width / 3 + 5, height / 30, "Level selector", Color::new(255, 255, 255), 60.0);

        // Salir del ciclo si se presiona Enter
        if window.is_key_down(minifb::Key::Enter) {
            break;
        }

        // Actualizar la ventana con el contenido del framebuffer
        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    let mut file_path = "";

    match option {
        0 => file_path = "src/maze1.txt",
        1 => file_path = "src/maze2.txt",
        2 => file_path = "src/maze3.txt",
        _ => file_path = "src/maze1.txt",
    }

    gameplay(framebuffer, file_path, width, height, window);
}


pub fn gameplay(framebuffer: &mut Framebuffer, file_path: &str, width: usize, height: usize, window: &mut Window) {
    let (mut maze, player_pos) = render(framebuffer, file_path, 0.5);
    let mut key_down = String::new(); // Cambiado a String

    let enemies_pos = render_enemies_pos(framebuffer, file_path);
    let block_size = std::cmp::min(
        framebuffer.get_width() / maze[0].len(),
        framebuffer.get_height() / maze.len(),
    ) as f32;
    
    let mut enemies: Vec<Enemy> = Vec::new();

    for pos in &enemies_pos {
        enemies.push(Enemy::new(*pos, PI / 2.0, -10.0, 20.0, framebuffer.get_height() as f32));
    }

    let mut player = Player::new(player_pos.x, player_pos.y, 0.0, PI / 3.0);

    let mut og_pos = player.get_pos();
    let mut new_pos = player.get_pos();

    let texture = Texture::from_file("textures/prison_wall.png");
    let texture_cell = Texture::from_file("textures/Cell.png");
    let texture_door = Texture::from_file("textures/Door.jpeg");
    let enemy_texture = Texture::from_file("textures/Police.png");

    let mut frame_count = 0;
    let start_time = Instant::now();

    // Inicializa el z_buffer
    let mut z_buffer = vec![f32::INFINITY; framebuffer.get_width()];

    let mut audio = AudioPlayer::new("Audio/Footsteps.wav",0.1);

    let mut enemy_collision = true;
    
    while window.is_open() && !window.is_key_down(Key::Escape) && enemy_collision {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            player.update_mouse(mouse_x as f32, mouse_y as f32, width as f32, height as f32);
        }
    
        // Cambia `audio` a referencia mutable
        let (key_down_str, new_pos) = player.process_events(&window, &maze, block_size, framebuffer, &mut audio);

        key_down = key_down_str; // Actualiza el valor de `key_down` con el valor de `key_down_str`

        if key_down == "/" {
            break;
        }
    
        framebuffer.clear();
    
        let mut wall_heights = vec![0; framebuffer.get_width()];
    
        // Renderiza el mapa en 3D
        render3d(framebuffer, &player, &maze, block_size, &texture, &texture_cell, &texture_door, &mut wall_heights);
    
        // Renderiza los enemigos
        for enemy in &enemies {
            render_enemy(
                framebuffer,
                &player,
                &enemy.get_pos(),
                &mut z_buffer,
                &enemy_texture,
                &wall_heights,
                300.0,
                &maze,
                block_size,
            );
        }
    
        maze = minimap(framebuffer, maze.clone(), 0.5, key_down, player.get_a(), og_pos, new_pos);
    
        let delta_time = 1.0 / 30.0;
    
        // Actualiza todos los enemigos
        for enemy in &mut enemies {
            let check_collision = enemy.update(delta_time, &maze, block_size);
            draw_enemies_position(framebuffer, &enemy.get_pos(), block_size as usize);
            draw_enemy_fov(framebuffer, &enemy, 30, &maze, block_size);
            if check_collision {
                enemy_collision = false;
                break;
            }
        }
    
        let (maze, player_pos) = render(framebuffer, file_path, 0.5);
    
        // Dibuja la posición del jugador en el minimapa
        draw_player_position(framebuffer, player.get_pos(), block_size as usize);
    
        frame_count += 1;
        let fps = calculate_fps(start_time, frame_count);
    
        if window.is_key_down(Key::F) {
            framebuffer.draw_text(width - 100, 10, &format!("FPS: {:.2}", fps), Color::new(0, 255, 0), 20.0);
        }
    
        og_pos = new_pos;
    
        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    if enemy_collision {
        framebuffer.clear();
        win_screen(framebuffer, window, width, height);
    } else {
        framebuffer.clear();
        defeat_screen(framebuffer, window, width, height);
    }  
}


pub fn win_screen(framebuffer: &mut Framebuffer, window: &mut Window, width: usize, height: usize) {
    let win_page = "textures/Ganar.png";
    let mut restart_game = false;

    let mut audio_shot = AudioPlayer::new("Audio/Shot.wav", 0.5);
    let mut audio_music = AudioPlayer::new("Audio/Liberado.mp3", 0.5);
    let mut shot_count = 0;

    let mut show_victory_screen = false;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Limpiar el framebuffer a negro antes de mostrar cualquier cosa
        framebuffer.clear();

        if shot_count < 2 {
            // Reproducir el audio de disparo y esperar un segundo entre reproducciones
            audio_shot.play();
            shot_count += 1;
            std::thread::sleep(Duration::from_millis(1000));
        } else if !show_victory_screen {
            // Solo después del segundo disparo se muestra la pantalla de victoria
            show_victory_screen = true;
        }

        if show_victory_screen {
            // Mostrar la imagen y el texto solo después del segundo disparo
            framebuffer.draw_image(&win_page, width, height);
            framebuffer.draw_text(width / 5 + 55, 5 * height / 6, "Press R to play again", Color::new(255, 255, 255), 60.0);
            
            // Reproducir la música de fondo
            audio_music.play();
        }

        // Comprobar si se ha presionado la tecla 'R' para continuar con el juego
        if window.is_key_down(minifb::Key::R) {
            restart_game = true;
            break;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
    
    if restart_game {
        audio_music.stop();
        game_start(width, height, framebuffer, window);
    }
}


pub fn defeat_screen(framebuffer: &mut Framebuffer, window: &mut Window, width: usize, height: usize) {
    let defeat_screen = "textures/Perdida.png";

    let mut restart_game = false;

    let mut audio_end = AudioPlayer::new("Audio/Atrapado.mp3", 0.5);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();

        framebuffer.draw_image(&defeat_screen, width, height);
        framebuffer.draw_text(width / 5 + 55, 5 * height / 6, "Press R to play again", Color::new(255, 255, 255), 60.0);

        if window.is_key_down(minifb::Key::R) {
            restart_game = true;
            break;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    if restart_game {
        audio_end.stop();
        game_start(width, height, framebuffer, window);
    }
}