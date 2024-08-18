use rodio::{OutputStream, Sink, Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    music_file: String,
}

impl AudioPlayer {
    pub fn new(music_file: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(music_file).unwrap());
        let source = Decoder::new(file).unwrap();

        sink.append(source);
        sink.set_volume(0.5);

        AudioPlayer {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            music_file: music_file.to_string(),
        }
    }

    pub fn play(&mut self) {
        // Asegúrate de que `sink` esté correctamente inicializado y que `music_file` sea válido
        let file = BufReader::new(File::open(&self.music_file).unwrap());
        let source = Decoder::new(file).unwrap();
        
        let mut sink = self.sink.lock().unwrap();
        sink.append(source);
        sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.lock().unwrap().stop();
    }

    pub fn play_loop(&mut self) {
        let music_file = self.music_file.clone();
        let sink = self.sink.clone();
        
        // Crear un nuevo hilo para reproducir el audio en bucle
        std::thread::spawn(move || {
            loop {
                // Abre el archivo y crea una nueva fuente de audio
                let file = BufReader::new(File::open(&music_file).unwrap());
                let source = Decoder::new(file).unwrap();
                
                // Obtiene el sink bloqueando el mutex
                let mut sink = sink.lock().unwrap();
                
                // Detiene la reproducción actual
                sink.stop(); 
                
                // Añade la nueva fuente de audio y reproduce
                sink.append(source);
                sink.play();
                
                // Espera hasta que termine de reproducir el audio antes de reiniciarlo
                while !sink.empty() {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }
    
}
