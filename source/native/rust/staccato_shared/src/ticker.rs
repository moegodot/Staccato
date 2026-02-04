use staccato_core::fallible::Fallible;
use staccato_core::tickable::Tickable;
use staccato_core::time_service::TimeService;
use std::error::Error;
use std::marker::PhantomData;

pub trait Ticker: Fallible {
    fn tick_per_second(&self) -> u64;
    fn set_tick_per_second(&mut self, set_to: u64);
    fn drive(
        &mut self,
        time_service: &dyn TimeService,
        tickable: &mut dyn Tickable<Error = Self::Error>,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct StdTicker<E: Error + Sync + Send + 'static> {
    tick_per_second: u64,
    last_tick: u64,
    last_update: u64,
    error_type: PhantomData<E>,
}

impl<E: Error + Sync + Send + 'static> Fallible for StdTicker<E> {
    type Error = E;
}

impl<E: Error + Sync + Send + 'static> StdTicker<E> {
    pub fn new(time_service: &dyn TimeService, tick_per_second: u64) -> Self {
        let last_tick = 0;
        let last_update = time_service.get_timestamp_ns();
        Self {
            tick_per_second,
            last_tick,
            last_update,
            error_type: Default::default(),
        }
    }

    fn compute_next_tick_ns(&self) -> u64 {
        if self.tick_per_second == 0 {
            return u64::MAX;
        }

        let elapse = 1_000_000_000 / self.tick_per_second;

        self.last_tick + elapse
    }

    fn should_tick(&self, current: u64) -> bool {
        current >= self.compute_next_tick_ns()
    }
}

impl<E: Error + Sync + Send + 'static> Ticker for StdTicker<E> {
    fn tick_per_second(&self) -> u64 {
        self.tick_per_second
    }

    fn set_tick_per_second(&mut self, set_to: u64) {
        self.tick_per_second = set_to
    }

    fn drive(
        &mut self,
        time_service: &dyn TimeService,
        tickable: &mut dyn Tickable<Error = Self::Error>,
    ) -> Result<(), Self::Error> {
        let current = time_service.get_timestamp_ns();
        let elapsed = self.last_update.saturating_sub(current);
        let should_tick = self.should_tick(current);

        tickable.pre_update(elapsed)?;

        if should_tick {
            tickable.fixed_update(elapsed)?;
        }

        tickable.update(elapsed)?;

        tickable.post_update(elapsed)?;

        Ok(())
    }
}
