use crate::log_model::LogModel;
use crate::models::config_model::Config;

pub struct FilterService {
    config: Config,
}

impl FilterService {
    pub fn new(config: Config) -> Self {
        FilterService { config }
    }


    /// Log'u filtreleme işlemi logic fonksiyonu
    pub fn filter_log(&self, log: LogModel) -> Option<LogModel> {
        // 1. Condition konfigürasyonlarına göre filtrele. Eger is_condition_met false dönerse log'u boş döndür. Eger true dönerse devam et
        if !self.is_condition_met(&log) {
            return None;
        }

        // 2. Log level konfigürasyonlarına göre filtrele. Eger is_log_level_allowed false dönerse log'u boş döndür. Eger true dönerse devam et
        if !self.is_log_level_allowed(&log) {
            return None;
        }

        // 3. Suspend konfigürasyonlarına göre filtrele. Eger is_suspended false dönerse log'u boş döndür. Eger true dönerse devam et
        if !self.is_suspended(&log) {
            return None;
        }

        // 4. Tag ekleme işlemi
        let mut log = log;
        self.add_tags(&mut log);

        Some(log)
    }


    /// Log'u filtreleme işlemi
    fn is_condition_met(&self, log: &LogModel) -> bool {
        // 1. Condition konfigürasyonlarına göre filtrele
        if self.config.condition.enabled {
            if self.config.condition.classes.contains(&log.class) || self.config.condition.services.contains(&log.service) {
                return false;
            }
        }

        true // Log'u kabul et
    }

    fn is_log_level_allowed(&self, log: &LogModel) -> bool {
        // 1. Log level konfigürasyonlarına göre filtrele
        if self.config.log.enabled {
            if !self.config.log.levels.contains(&log.level) {
                return false; // Filtrele
            }
        }

        true // Log'u kabul et
    }

    fn is_suspended(&self, log: &LogModel) -> bool {
        // 1. Suspend konfigürasyonlarına göre filtrele
        if self.config.suspend.enabled {
            if self.config.suspend.classes.contains(&log.class) {
                return false; // Filtrele
            }
            if self.config.suspend.services.contains(&log.service) {
                return false; // Filtrele
            }
        }

        true // Log'u kabul et
    }

    /// Log'a tag ekleme işlemi
    fn add_tags(&self, log: &mut LogModel) {
        if self.config.tag.enabled {
            // Class için tag ekle
            if let Some(tags) = self.config.tag.classes.get(&log.class) {
                log.tags.extend(tags.clone());
            }

            // Service için tag ekle
            if let Some(tags) = self.config.tag.services.get(&log.service) {
                log.tags.extend(tags.clone());
            }
        }
    }
} 