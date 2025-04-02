pub mod drivers;
pub mod qwen;

pub trait Driver {
    fn name(&self) -> &str;
    fn generate(&self, prompt: &str) -> Result<String>;
}
