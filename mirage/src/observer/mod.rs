// use log::{debug, warn};
// use rand::Rng;
// use std::{
//     sync::{Arc, Mutex},
//     thread,
//     time::Duration,
// };

// use crate::{
//     tools::{memory_heap_tamper::MemoryHeapTamper, memory_scramble::MemoryScramble, Tool},
//     utils::environment::operation_system::gather_os_info,
// };

// pub struct Observer {
//     is_observed: Arc<Mutex<bool>>,
//     temperature: Arc<Mutex<f32>>,
//     reconnaissance_enabled: Arc<Mutex<bool>>,
// }

// impl Observer {
//     pub fn new() -> Self {
//         Observer {
//             is_observed: Arc::new(Mutex::new(true)),
//             temperature: Arc::new(Mutex::new(1.0)),
//             reconnaissance_enabled: Arc::new(Mutex::new(false)),
//         }
//     }

//     pub fn start(&self) {
//         let is_observed = Arc::clone(&self.is_observed);
//         let temperature = Arc::clone(&self.temperature);
//         let reconnaissance_enabled = Arc::clone(&self.reconnaissance_enabled);

//         let mut memory_heap_tamper = MemoryHeapTamper::new();
//         let mut memory_scrambler = MemoryScramble::new();

//         if cfg!(feature = "development") {
//             debug!("[Observer] initializing...");
//         }

//         thread::spawn(move || {
//             let mut rng = rand::thread_rng();

//             loop {
//                 let current_temp = *temperature.lock().unwrap();
//                 let is_recon_enabled = *reconnaissance_enabled.lock().unwrap();

//                 if cfg!(feature = "development") {
//                     debug!("[Observer]: current temperature: {}", current_temp);
//                     debug!("[Observer]: reconnaissance status: {}", is_recon_enabled)
//                 }

//                 if is_recon_enabled {
//                     let os_info = gather_os_info();
//                     if cfg!(feature = "development") {
//                         debug!("[Observer]: OS information: \n{:#?}", os_info);
//                     }
//                 }

//                 if *is_observed.lock().unwrap() {
//                     if cfg!(feature = "development") {
//                         warn!("[Observer]: Detected observation.");
//                     }

//                     // if current_temp >= 0.8 {
//                     //     memory_heap_tamper.start();
//                     // }

//                     memory_scrambler.start();
//                 } else {
//                     if cfg!(feature = "development") {
//                         debug!("[Observer]: current temperature: {}", current_temp);
//                         debug!("[Observer]: No observation detected, sleeping...");
//                     }
//                 }

//                 let sleep_time = if current_temp >= 0.5 {
//                     rng.gen_range(1..6)
//                 } else {
//                     rng.gen_range(6..11)
//                 };
//                 thread::sleep(Duration::from_secs(sleep_time));
//             }
//         });
//     }

//     pub fn set_temperature(&self, temperature: f32) {
//         let mut temp = self.temperature.lock().unwrap();
//         *temp = temperature.clamp(0.0, 1.0);
//     }

//     pub fn set_reconnaissance_mode(&self, enable: bool) {
//         let mut recon = self.reconnaissance_enabled.lock().unwrap();
//         *recon = enable;
//     }
// }
