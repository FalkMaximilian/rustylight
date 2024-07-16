<h1 align="center">✨ Rustylight ✨</h1>

<p align=center>This project aims to provide an easy to use, efficient alternative to ambilight that can be realised for ~100€</p> 

## Links 🔗
- [Repo](https://github.com/FalkMaximilian/rustylight.git "Rustylight Repo")

## Prerequisites 

- Rust (as per [cargo-msrv](https://crates.io/crates/cargo-msrv) the MSRV seems to be 1.74.1)
- OpenCV development library (libopencv-dev)

## Building and Running 🏃

I recommend that you build the project on the target device itself. You are of course free to cross compile.

For my Raspberry Pi 3B I had to do the following:

1. Activate SPI through 
```
sudo raspi-config
```
2. Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Install clang and other requirements
```
sudo apt install build-essential clang libclang-dev curl
```
4. Install OpenCV
```
sudo apt install libopencv-dev
```
5. Install vim and git (Optional)
```
sudo apt install vim git
```

### Possible problems 🚨
I got an error that libclang.so or libclang-*.so could not be found. After installing libclang-dev and setting the environment variable LIBCLANG_PATH it worked. To find where your libclang.so is located you can do the following:
```
sudo find / -name 'libclang*.so'
```
In my case libclang.so was located in ...

### Wiring everything up 

So the GND cable of the lightstrip goes into one of the black pins. The DATA cable of the lightstrip goes into pin 18 (Top Row, 6th pin from the left)
![Raspberry Pi Pinout](https://raw.githubusercontent.com/pinout-xyz/Pinout.xyz/master/resources/raspberry-pi-pinout.png)
Thanks for the image pinout.xyz!



## Future updates ⬆️

- [x] Flexible amount of LEDs
- [x] Configure at which edge of the screen the lightstrip starts 
- [x] Select if the lightstrip is placed clockwise or counter clockwise
- [ ] Simple webserver to turn ambilight on/off (homekit)
- [ ] Eventually use V4L instead of OpenCV. OpenCVs many features aren't needed.



## Author

**Maximilian Falk**

- [Profile](https://github.com/FalkMaximilian "Maximilian Falk")
- [Email](mailto:falk.maximilian@outlook.com?subject=Rustylight% ":P")
- [Website](https://maximilian-falk.com/ "maximilian-falk.com")


## Contribution

So far only I am working on this project.

## Screenshots 📸

Will be added soon :)

