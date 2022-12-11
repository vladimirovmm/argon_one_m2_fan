use anyhow::Result;
use sysinfo::{ComponentExt, System, SystemExt};

const AVG_LENGTH: usize = 10;

pub struct Temperature {
    system_info: sysinfo::System,
    list: [u8; AVG_LENGTH + 1],
}

impl Temperature {
    pub fn new() -> Result<Temperature> {
        let mut system_info = System::new_all();
        let current_temperature = temperature(&mut system_info);

        Ok(Temperature {
            system_info,
            list: [current_temperature; AVG_LENGTH + 1],
        })
    }

    fn update(&mut self) {
        let mut pos = self
            .list
            .iter()
            .enumerate()
            .find(|(.., value)| **value == 0)
            .map(|(pos, ..)| pos)
            .unwrap_or_default();
        self.list[pos] = temperature(&mut self.system_info);

        pos += 1;
        if pos > AVG_LENGTH {
            pos = 0;
        }
        self.list[pos] = 0;
    }

    pub fn get(&mut self) -> u8 {
        self.update();
        let sum: usize = self
            .list
            .iter()
            .filter(|val| **val != 0)
            .map(|val| *val as usize)
            .sum();

        (sum / AVG_LENGTH) as u8
    }
}

fn temperature(system_info: &mut System) -> u8 {
    system_info.refresh_components();
    system_info.components()[0].temperature() as u8
}
