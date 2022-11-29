use chrono::{DateTime};
use chrono::{Local};

mod timeframe;
use timeframe::Timeframe;
use super::Company;
use super::Software;


// World for our Software and Company to live in
//
pub struct World {
    _global_economic_factors: u16,        // 0-1000
    _competition_in_market: u16,          // 0-1000
    _job_market: u16,                     // 0-1000
    _timeframe: Timeframe
}

impl World {

    pub fn new(global_economic_factors :u16, competition_in_market :u16, job_market :u16, speed :u16, game_ticks :u32) -> World {   
        return World { _global_economic_factors: global_economic_factors, _competition_in_market: competition_in_market, _job_market: job_market, _timeframe: Timeframe::new(speed, game_ticks) };
    }

    pub fn global_economic_factors(& self) -> u16 {
        self._global_economic_factors
    }

    pub fn competition_in_market(&self) -> u16 {
        self._competition_in_market
    }

    pub fn job_market(&self) -> u16 {
        self._job_market
    }

    pub fn speed(&self) -> u16 {
        self._timeframe.speed()
    }

    pub fn game_ticks(&self) -> u32 {
        self._timeframe.game_ticks()
    }

    pub fn last_tick_time(&self) -> DateTime<Local> {
        self._timeframe.last_tick_time()
    }

    //pub fn game_start_time(&self) -> DateTime<Local> {
    //    self._game_start_time
    //}
    
    pub fn increment_game_ticks(&mut self, company: &Company, software: &Software, time_now: DateTime<Local>) {
        
        // 
        self._timeframe.increment_game_ticks();

        // run the update
        self.do_game_update(company, software);

        // Update time
        //
        self._timeframe.set_current_time(time_now);
    }

    pub fn get_game_elapse_time(& self) -> chrono::Duration {
        self._timeframe.get_game_elapse_time()
    }


    // Process a tick of progress (how do we work this out?)
    //
    // The world defines how our software and company do the update and what that
    // means to the company - so the logical placing of everything appears right at
    // the moment.
    //
    fn do_game_update (&mut self, company: &Company, software: &Software) {
        
    }

    
}

#[cfg(test)]
mod test {
    use crate::company::CompanyDirection;

    use super::*;

    #[test]
    fn time_tests() {

        let mut world = World::new(100, 100, 100, 100, 0);

        let company: Company = Company::new(100, 100, 100, CompanyDirection::B2B);
        let software: Software = Software::new(100, 100, 100, 100);
        world.increment_game_ticks(&company, &software, Local::now());
        assert_eq!(world.game_ticks(), 1);
    }


}
