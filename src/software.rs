use rand::Rng;
use crate::world::timeframe::YearWeek;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Architecture {
    ProofofConcept,
    Monolith,
    Microservices,
    EventDriven
}

pub enum MonetizationModel {
    OpenSource,
    Freemium,
    FreeTier,
    Proprietary
}


pub struct Software {
    _lines_of_code:         u32,    // total
    _age_of_code:           u16,    // weeks
    _complexity_of_code:    u16,    // 0 - 100
    _feature_richness:      u16,    // 0 - 100
    _ease_of_use:           u16,    // 0 - 100
    _components:            u16,    // components 1 - 100
    _services:              u16,    // services 1 - 100
    _dependencies:          u16,    // dependencies 1 - 100
    _cost_of_service:       u16,    // price of service
    _architecture: Architecture,    // what is the predominant architecture
    _customer_satisfaction: u16,    // 0 - 100
    _customers: u16,                // number of customers
    _active_users: u32,             // number of active users
    _capacity_percentage_active_users: u16,             // number of active users (not same as customers)
    _percentage_free_users: u16,    // percentage of free users
    _monetization_model:    MonetizationModel,
    _releases:              u16,    // how many releases have their been
    _last_release_yearweek: YearWeek,
    _reliability:           u16,    // 0 - 100
    _technical_debt:        u16     // 0 - 100
}


// Active users can be way more than customers depending on model (B2B or B2C etc)
//
impl Software {

    pub fn new(lines_of_code: u32, age_of_code: u16, complexity_of_code: u16, cost_of_service: u16) -> Software {   
        return Software { _lines_of_code: lines_of_code,
                          _age_of_code: age_of_code,
                          _complexity_of_code: complexity_of_code,
                          _feature_richness: 0,
                          _ease_of_use: 0,
                          _components: 0,
                          _services: 0,
                          _dependencies: 0,
                          _cost_of_service: cost_of_service,
                          _architecture: Architecture::ProofofConcept,
                          _customer_satisfaction: 0,
                          _customers: 0,
                          _active_users: 0,
                          _capacity_percentage_active_users: 0,            // 0 - 100 percentage to capacity - but can be over capacity too
                          _percentage_free_users: 0,
                          _monetization_model: MonetizationModel::Proprietary,
                          _releases: 0,
                          _last_release_yearweek: YearWeek::new(2000, 1),
                          _reliability: 100,
                          _technical_debt: 0
                        };
    }


    // Measure of quality from complexity_of_code, feature_richness, ease_of_use, components, services, dependencies, architecture
    pub fn quality(&self) -> u16 {
        if self._complexity_of_code < 10 || self._services < 2 || self._components < 5 {
            return 100;
        }

        // etc
        return 0;
    }

    // Simple value which is an average of the various usability factors
    // 
    pub fn usability_factor(&self) -> u16 {
        if self._releases == 0 {
            return 0
        }

        let usability = ( self._ease_of_use + self._feature_richness ) / 2 - self._technical_debt;

        if usability > 0 {
            return usability as u16
        } else  {
            return 0
        }
    }

    // Factor of number of current customers, current users and the monetization model
    // 
    //
    pub fn market_popularity(&self, current_yearweek: &YearWeek) -> u16 {

        if self._releases == 0 {
            return 0
        }

        let mut popularity = ( self._customers as i16 + self._capacity_percentage_active_users as i16 ) / 2;
        
        // Now when was the last release?
        //
        let difference_weeks = self._last_release_yearweek.difference_weeks(current_yearweek);

        if difference_weeks < 1 {
            return popularity as u16
        } else if difference_weeks < 10  {
            popularity += 15
        } else if difference_weeks < 20 {
            popularity += 5
        } else if difference_weeks < 40 {
            popularity /= 2
        } else {
            popularity /= 4
        }

        return popularity as u16

    }

    pub fn lines_of_code(&self) -> u32 {
        self._lines_of_code
    }

    pub fn releases(&self) -> u16 {
        self._releases
    }

    pub fn reliability(&self) -> u16 {
        self._reliability
    }

    pub fn add_customers(&mut self, customers: u16, b2b: bool) {
        self._customers += customers;

        // Adjust users according to b2b selling
        //
        if b2b {
            let mut rng = rand::thread_rng();
            let rand_factor: f32 = rng.gen();
            let rand_users: f32 = rng.gen();
    
            self._active_users += ( customers as f32 * rand_factor * rand_users * 20.0f32 ) as u32

        } else {
            self._active_users += customers as u32;
        }
    }

    pub fn remove_customers(&mut self, customers: u16) {

        if customers > self._customers {
            // remove a percentage of active users
            //
            let factor = customers as f64 / self._customers as f64;
            self._active_users /= ( self._active_users as f64 * factor) as u32;
            self._customers -= customers
        } else {
            self._customers = 0;
            self._active_users = 0;
        }
    }

    pub fn age_of_code(&self) -> u16 {
        self._age_of_code
    }

    pub fn complexity_of_code(&self) -> u16 {
        self._complexity_of_code
    }

    pub fn cost_of_service(&self) -> u16 {
        self._cost_of_service
    }


    // Three key methods
    //
    // - Work on Features
    // - Refactor
    // - Bug Fix
    //

    // Work of features
    //
    // Finger very much in the air
    pub fn work_on_features(&mut self, number_of_devs: u16, dev_focus: u16, days: u16) {
        let loc_per_day = 250;
        self._lines_of_code += ( number_of_devs as f32 * (dev_focus as f32 / 100.0f32 ) ) as u32 * loc_per_day * days as u32;

        self._complexity_of_code = 50;
    }

    pub fn refactor(&mut self) {

    }

    pub fn bug_fix(&mut self) {

    }

    pub fn add_lines(&mut self, lines :u32) {
        self._lines_of_code += lines
    }

    pub fn remove_lines(&mut self, lines: u32) {
        self._lines_of_code -= lines
    }

    pub fn add_age(&mut self, age :u16) {
        self._age_of_code += age
    }

    pub fn add_complexity(&mut self, complexity :u16) {
        self._complexity_of_code += complexity
    }

    pub fn remove_complexity(&mut self, complexity :u16) {
        self._complexity_of_code -= complexity
    }

    pub fn get_architecture(&self) -> Architecture {
        self._architecture
    }

    pub fn customers(&self) -> u16 {
        self._customers
    }

    pub fn capacity_percentage_active_users(&self) -> u16 {
        self._capacity_percentage_active_users
    }

    pub fn set_capacity_percentage_active_users(&mut self, users: u16) {
        self._capacity_percentage_active_users = users
    }

}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_lines_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_lines(100);
        assert_eq!(software.lines_of_code(), 100);

        software.remove_lines(50);
        assert_eq!(software.lines_of_code(), 50);
    }

    #[test]
    fn software_age_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_age(100);
        assert_eq!(software.age_of_code(), 100);
    }

    #[test]
    fn software_complexity_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_complexity(100);
        assert_eq!(software.complexity_of_code(), 100);

        software.remove_complexity(50);
        assert_eq!(software.complexity_of_code(), 50);
    }

    #[test]
    fn software_customers_tests() {
        let mut software = Software::new(0, 0, 0, 0);

        software.add_customers(20, false);
        assert_eq!(software.customers(), 20);
    }


}