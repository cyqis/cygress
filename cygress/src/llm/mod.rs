use async_trait::async_trait;
use pingora_core::modules::http::{HttpModule, HttpModuleBuilder, Module};
use pingora_error::Result;
use pingora_http::RequestHeader;
use std::any::Any;

pub struct LLMCtx {}

#[async_trait]
impl HttpModule for LLMCtx {
    async fn request_header_filter(&mut self, _req: &mut RequestHeader) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct LLM {}

impl HttpModuleBuilder for LLM {
    fn init(&self) -> Module {
        Box::new(LLMCtx {})
    }
}
