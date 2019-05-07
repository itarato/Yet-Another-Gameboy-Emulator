use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::Sdl;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct SoundPacket {
  pitch: f32,                   // 1.0 .. ~k
  volume: f32,                  // 0.0 .. 1.0
  envelope_sweep_length: usize, // 22050 = 1s
  envelope_direction_down: bool,
  waveform: f32, // 0.0 .. 1.0
  restart: bool,
}

impl SoundPacket {
  fn new(
    pitch: f32,
    volume: f32,
    envelope_sweep_length: usize,
    envelope_direction_down: bool,
    waveform: f32,
  ) -> SoundPacket {
    SoundPacket {
      pitch,
      volume,
      envelope_sweep_length,
      envelope_direction_down,
      waveform,
      restart: true,
    }
  }
}

struct SquareWave {
  freq: f32,
  phase: f32,
  pocket: Arc<Mutex<Option<SoundPacket>>>,
  envelope_sweep_counter: usize,
}

impl AudioCallback for SquareWave {
  type Channel = f32;

  fn callback(&mut self, out: &mut [f32]) {
    for x in out.iter_mut() {
      let mut pocket = self.pocket.lock().unwrap();

      *x = if pocket.is_some() {
        if (*pocket).as_ref().unwrap().restart {
          (*pocket).as_mut().unwrap().restart = false;
          self.envelope_sweep_counter = (*pocket).as_mut().unwrap().envelope_sweep_length;
        }

        let pitch = pocket.as_ref().unwrap().pitch;
        self.phase = (self.phase + (pitch / self.freq)) % 1.0;

        if (*pocket).as_ref().unwrap().envelope_sweep_length > 0 {
          if self.envelope_sweep_counter > 0 {
            self.envelope_sweep_counter -= 1;
          } else {
            (*pocket).as_mut().unwrap().volume +=
              if (*pocket).as_mut().unwrap().envelope_direction_down {
                -1f32 / 15f32
              } else {
                1f32 / 15f32
              };
            self.envelope_sweep_counter = (*pocket).as_mut().unwrap().envelope_sweep_length;
          }
        }

        if (*pocket).as_ref().unwrap().volume < 0f32 {
          (*pocket).as_mut().unwrap().volume = 0.0;
        } else if (*pocket).as_ref().unwrap().volume > 1f32 {
          (*pocket).as_mut().unwrap().volume = 1.0;
        }

        if self.phase <= pocket.as_ref().unwrap().waveform {
          pocket.as_ref().unwrap().volume
        } else {
          -pocket.as_ref().unwrap().volume
        }
      } else {
        0.0
      };
    }
  }
}

pub struct Sound {
  pub nr10: u8,
  pub nr11: u8,
  pub nr12: u8,
  pub nr13: u8,
  pub nr14: u8,

  pub nr50: u8,
  pub nr51: u8,
  pub nr52: u8,

  audio_device: AudioDevice<SquareWave>,

  channel1_out: Arc<Mutex<Option<SoundPacket>>>,
}

impl Sound {
  pub fn new(sdl: Rc<Sdl>) -> Sound {
    let desired_spec = AudioSpecDesired {
      freq: Some(44_100),
      channels: Some(1),
      samples: None,
    };

    let pocket = Arc::new(Mutex::new(None));

    let device = sdl
      .audio()
      .unwrap()
      .open_playback(None, &desired_spec, |spec| SquareWave {
        freq: spec.freq as f32,
        phase: 0.5,
        pocket: pocket.clone(),
        envelope_sweep_counter: 0,
      })
      .unwrap();
    device.resume();

    Sound {
      nr10: 0,
      nr11: 0,
      nr12: 0,
      nr13: 0,
      nr14: 0,

      nr50: 0,
      nr51: 0,
      nr52: 0,

      audio_device: device,
      channel1_out: pocket,
    }
  }

  pub fn reset(&mut self) {
    self.nr52 = 0x0;
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    // println!("0x{:>04x} = 0b{:>08b}", addr, w);
    match addr {
      0xff10 => self.nr10 = w,
      0xff11 => self.nr11 = w,
      0xff12 => self.nr12 = w,
      0xff13 => self.nr13 = w,
      0xff14 => {
        self.nr14 = w;
        self.handle_channel_1_out();
      }

      0xff24 => self.nr50 = w,
      0xff25 => self.nr51 = w,
      0xff26 => self.nr52 = w,
      0xff10...0xff3f => panic!("Unimplemented sound addr: 0x{:>04x}", addr),
      _ => unimplemented!("Unsupported sound addr: 0x{:>04x}", addr),
    };
  }

  pub fn read_word(&self, addr: u16) -> u8 {
    unimplemented!("Unimplemented sound chip reg read at 0x{:>04x}", addr);
  }

  fn is_sound_reg_enabled(&self) -> bool {
    bitn!(self.nr52, 7) == 0x1
  }

  fn handle_channel_1_out(&self) {
    if !self.is_sound_reg_enabled() {
      return;
    }

    if bitn!(self.nr14, 7) != 1 {
      return;
    }

    let wave_duty = self.nr11 >> 6;
    let sound_length: f32 = (64 - (self.nr11 & 0b11_1111)) as f32 * (1f32 / 256f32);
    let freq_raw: u32 = self.nr13 as u32 | (((self.nr14 & 0b111) as u32) << 8);
    let freq: u32 = 131072 / (2048 - freq_raw);
    let envelope: u8 = self.nr12 >> 4;
    let is_envelope_inc: bool = bitn!(self.nr12, 3) == 1;
    let envelope_steps: u8 = self.nr12 & 0b111;
    let should_use_length: bool = bitn!(self.nr14, 6) == 1;

    // @TODO: "When sound output is finished, bit 0 of register NR52, the Sound 1 ON flag, is reset."

    // println!(
    //   "WaveDuty 0b{:b} SoundLen {:?} Freq {:?} Envelope {:?} EnvelopeSteps {:?}",
    //   wave_duty, sound_length, freq, envelope, envelope_steps
    // );

    {
      let mut pocket = self.channel1_out.lock().unwrap();
      *pocket = Some(SoundPacket::new(
        freq as f32,
        envelope as f32 / 15f32,
        (44_100 * envelope_steps as usize) / 64,
        true,
        0.5,
      ));
    }
  }
}
