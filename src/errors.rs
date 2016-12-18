use ::plugin;

use tokio_timer::TimerError;

#[derive(Debug)]
pub enum MainError {
    PluginPoll(plugin::PollError),
    PluginUpdate(plugin::UpdateError),
    Timer(TimerError)
}
