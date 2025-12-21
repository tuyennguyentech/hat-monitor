use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HatSample {
  pub timestamp: u64,
  pub temperature: f32,
  pub humidity: f32,
  pub r_zero: f32,
  pub corrected_r_zero: f32,
  pub resistance: f32,
  pub ppm: f32,
  pub corrected_ppm: f32,
}
