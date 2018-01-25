use errors::*;
use plugin::*;

#[derive(Deserialize, Debug)]
struct HttpInputConfig {
    target: String,
}

#[derive(Debug)]
struct HttpInput {}

impl Input for HttpInput {
    fn setup(&self, ctx: PluginContext) -> Result<Box<InputInstance>> {
        let _c: HttpInputConfig = ctx.decode_config()?;
        Ok(Box::new(HttpInputInstance::new()))
    }
}

#[derive(Debug)]
struct HttpInputInstance {}

impl HttpInputInstance {
    pub fn new() -> HttpInputInstance {
        return HttpInputInstance {};
    }
}

impl InputInstance for HttpInputInstance {}

pub fn input() -> Result<Box<Input>> {
    Ok(Box::new(HttpInput {}))
}
