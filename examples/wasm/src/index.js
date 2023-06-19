import { Chip8, Key, KeyState } from '@deanrumsby/chip8_core';
import { memory } from '@deanrumsby/chip8_core/chip8_core_bg.wasm';

async function run() {
  const chip8 = new Chip8(BigInt(Math.floor(Math.random() * 1000)));
  const view = new Uint8ClampedArray(memory.buffer, chip8.frame_buffer_mut_ptr(), chip8.frame_buffer_len());
  let previousTimeStamp;

  const keys = {
    '1': Key.Key1,
    '2': Key.Key2,
    '3': Key.Key3,
    '4': Key.KeyC,
    'q': Key.Key4,
    'w': Key.Key5,
    'e': Key.Key6,
    'r': Key.KeyD,
    'a': Key.Key7,
    's': Key.Key8,
    'd': Key.Key9,
    'f': Key.KeyE,
    'z': Key.KeyA,
    'x': Key.Key0,
    'c': Key.KeyB,
    'v': Key.KeyF,
  };

  const input = document.querySelector('#input');
  input.addEventListener('change', (e) => {
    const fileReader = new FileReader();
    fileReader.onload = () => {
      const typedArray = new Uint8Array(fileReader.result);
      chip8.load(typedArray);
    };
    fileReader.readAsArrayBuffer(input.files[0]);
  });

  const canvas = document.querySelector('#canvas');
  const ctx = canvas.getContext('2d');

  const animate = (timeStamp) => {
    if (!previousTimeStamp) {
      previousTimeStamp = timeStamp;
    }
    const elapsed = timeStamp - previousTimeStamp;
    const elapsedMicroSeconds = convertTimeStamp(elapsed);
    previousTimeStamp = timeStamp;
    chip8.update(elapsedMicroSeconds);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const imageData = new ImageData(view, canvas.width, canvas.height);
    ctx.putImageData(imageData, 0, 0);
    window.requestAnimationFrame(animate);
  };

  const start = document.querySelector('#start');
  start.addEventListener('click', () => window.requestAnimationFrame(animate));

  window.addEventListener('keydown', (e) => {
    const key = keys[e.key];
    const keyState = KeyState.Pressed;
    chip8.handle_key_event(key, keyState);
  });
  window.addEventListener('keyup', (e) => {
    const key = keys[e.key];
    const keyState = KeyState.Released;
    chip8.handle_key_event(key, keyState);
  });

  const convertTimeStamp = (timestamp) => BigInt(Math.floor(timestamp * 1000));
}

run();