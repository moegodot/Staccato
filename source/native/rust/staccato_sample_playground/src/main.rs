use staccato_application::ApplicationInformation;
use staccato_application::staccato_core::fallible::Fallible;
use staccato_application::staccato_core::rect::Size;
use staccato_application::staccato_core::tickable::Tickable;
use staccato_application::staccato_core::time_service::StdTimeService;
use staccato_application::staccato_hal::sdl_event_source::SdlEventSource;
use staccato_application::staccato_hal::wgpu_context::WgpuRenderContext;
use staccato_application::staccato_hal::wgpu_window::WgpuWindow;
use staccato_application::staccato_hal::window::{Window, WindowOption};
use staccato_application::staccato_shared::event::{AppEvent, Event, RawEvent};
use staccato_application::staccato_shared::event_dispatcher::{
    EventDispatcher, EventHandler, EventSource, StdEventDispatcher,
};
use staccato_application::staccato_shared::ticker::{StdTicker, Ticker};
use staccato_telemetry::initialize;
use std::convert::Infallible;
use tracing::{error, trace};

#[derive(Debug)]
pub struct Main<'a> {
    window: WgpuWindow<'a>,
    context: WgpuRenderContext,
    running: bool,
}

impl<'w> From<(WgpuRenderContext, WgpuWindow<'w>)> for Main<'w> {
    fn from(value: (WgpuRenderContext, WgpuWindow<'w>)) -> Self {
        Self {
            window: value.1,
            context: value.0,
            running: true,
        }
    }
}

impl Fallible for Main<'_> {
    type Error = Infallible;
}

impl Tickable for Main<'_> {
    fn pre_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn fixed_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn post_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl EventHandler for Main<'_> {
    fn handle(&mut self, event: &Event) -> Result<bool, Self::Error> {
        if let RawEvent::Quit = event.raw {
            self.running = false;
        }
        if let RawEvent::App {
            event: AppEvent::Terminating,
        } = event.raw
        {
            self.running = false;
        }
        if let RawEvent::WindowClose { id } = event.raw {
            self.running = false
        }

        Ok(true)
    }
}

fn main() {
    let app_info = ApplicationInformation {
        name: "staccato sample - playground".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        identifier: "moe.kawayi.staccato".to_string(),
        creator: "MoeGodot".to_string(),
        copyright: "2026 copyright - moegodot".to_string(),
        url: "http://github.com/moegodot/".to_string(),
        app_type: "application".to_string(),
    };

    let app = app_info.initialize_app(None).unwrap();

    let mut event_source = SdlEventSource::default();

    let event_dispatcher = StdEventDispatcher::default();

    let time_service = StdTimeService::new();

    let mut ticker: StdTicker<<Main<'_> as Fallible>::Error> = StdTicker::new(&time_service, 50);

    let window = Window::new(WindowOption {
        title: "hello world".into(),
        size: Size {
            width: 1024,
            height: 768,
        },
    })
    .unwrap();

    let mut main: Main<'_> = WgpuRenderContext::new_with_window(
        window,
        &Default::default(),
        &Default::default(),
        &Default::default(),
    )
    .unwrap()
    .into();

    while main.running {
        let events = event_source.poll();

        for event in events {
            event_dispatcher.fire(&mut [&mut main], event).unwrap();
        }

        ticker.drive(&time_service, &mut main).unwrap();
    }
}
