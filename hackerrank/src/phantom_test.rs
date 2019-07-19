use std::marker::PhantomData;

struct Km;
struct Mile;

#[repr(transparent)]
struct Distance<T> {
    data: f64,
    _unit: PhantomData<T>,
}

impl Distance<Km> {
    pub fn new_km(data: f64) -> Distance<Km> {
        Self {
            data,
            _unit: PhantomData::<Km>,
        }
    }

    pub fn into_mile(self) -> Distance<Mile> {
        Distance {
            data: self.data * 0.621_371,
            _unit: PhantomData::<Mile>,
        }
    }
}

impl Distance<Mile> {
    pub fn new_mile(data: f64) -> Distance<Mile> {
        Self {
            data,
            _unit: PhantomData::<Mile>,
        }
    }

    pub fn into_km(self) -> Distance<Km> {
        Distance {
            data: self.data * 1.609_34,
            _unit: PhantomData::<Km>,
        }
    }
}

#[test]
fn test() {
    let km = Distance::new_km(2.);
    // fails to compile:
    // let km = km.into_km();
    let ans = km.data * 0.621_371;
    // ml is a Distance<Mile>
    let ml = km.into_mile();
    assert_eq!(ml.data, ans);
}
