mod range_iterator;
use super::{PortRange, ScanOrder};
use rand::seq::SliceRandom;
use rand::thread_rng;
use range_iterator::RangeIterator;

#[derive(Debug)]
pub enum PortStrategy {
    Manual(Vec<u16>),
    Serial(SerialRange),
    Random(RandomRange),
}

impl PortStrategy {
    pub fn pick(range: &Option<PortRange>, ports: Option<Vec<u16>>, order: ScanOrder) -> Self {
        match order {
            ScanOrder::Serial if ports.is_none() => {
                let range = range.as_ref().unwrap();
                PortStrategy::Serial(SerialRange {
                    start: range.start,
                    end: range.end,
                })
            }
            ScanOrder::Random if ports.is_none() => {
                let range = range.as_ref().unwrap();
                PortStrategy::Random(RandomRange {
                    start: range.start,
                    end: range.end,
                })
            }
            ScanOrder::Serial => PortStrategy::Manual(ports.unwrap()),
            ScanOrder::Random => {
                let mut rng = thread_rng();
                let mut ports = ports.unwrap();
                ports.shuffle(&mut rng);
                PortStrategy::Manual(ports)
            }
        }
    }

    pub fn order(&self) -> Vec<u16> {
        match self {
            PortStrategy::Manual(ports) => ports.clone(),
            PortStrategy::Serial(range) => range.generate(),
            PortStrategy::Random(range) => range.generate(),
        }
    }
}

trait RangeOrder {
    fn generate(&self) -> Vec<u16>;
}

#[derive(Debug)]
pub struct SerialRange {
    start: u16,
    end: u16,
}

impl RangeOrder for SerialRange {
    fn generate(&self) -> Vec<u16> {
        (self.start..self.end).collect()
    }
}

#[derive(Debug)]
pub struct RandomRange {
    start: u16,
    end: u16,
}

impl RangeOrder for RandomRange {
    fn generate(&self) -> Vec<u16> {
        RangeIterator::new(self.start.into(), self.end.into()).collect()
    }
}
