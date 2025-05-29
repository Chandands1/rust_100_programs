#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    
    println!("Starting with {:?} light ({} seconds)", light, light.duration());
    
    // Cycle through the traffic lights
    for _ in 0..5 {
        light = light.next();
        println!("Changed to {:?} light ({} seconds)", light, light.duration());
    }
}