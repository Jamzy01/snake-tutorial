use max7219_canvas::layer::CanvasLayer;
use rand::{ Rng, RngCore };

pub struct Fruits {
    fruit_locations: [[bool; 8]; 8],
}

impl Default for Fruits {
    fn default() -> Self {
        Self::new()
    }
}

impl Fruits {
    pub fn new() -> Self {
        Self {
            fruit_locations: [[false; 8]; 8],
        }
    }

    pub fn as_layer(&self) -> CanvasLayer<1> {
        let mut layer = CanvasLayer::new();

        for x in 0..8 {
            for y in 0..8 {
                if self.fruit_locations[x][y] {
                    layer.set_pixel(x, y, true);
                }
            }
        }

        layer
    }

    pub fn spawn_fruit(&mut self, x: usize, y: usize) {
        self.fruit_locations[x][y] = true;
    }

    pub fn despawn_fruit(&mut self, x: usize, y: usize) {
        self.fruit_locations[x][y] = false;
    }

    pub fn is_fruit(&self, x: usize, y: usize) -> bool {
        self.fruit_locations[x][y]
    }

    pub fn spawn_fruit_at_random_location(
        &mut self,
        rng: &mut impl RngCore,
        blocklisted_locations: &CanvasLayer<1>
    ) {
        if blocklisted_locations.is_full() {
            return;
        }

        let (x, y) = loop {
            let x = rng.gen_range(0..8);
            let y = rng.gen_range(0..8);

            if !blocklisted_locations.get_pixel(x, y) {
                break (x, y);
            }
        };

        self.spawn_fruit(x, y)
    }
}
