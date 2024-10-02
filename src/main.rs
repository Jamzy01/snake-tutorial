#![no_std]
#![no_main]

use arduino_hal::Adc;
use fruits::Fruits;
use joystick::joystick_input_from_raw;
use panic_halt as _;
use max7219::MAX7219;
use max7219_canvas::DisplayCanvas;
use const_random::const_random;
use rand::{ rngs::SmallRng, RngCore, SeedableRng };
use snake::{ Direction, Snake };

pub mod fruits;
pub mod snake;
pub mod joystick;

const GAME_RUN_RATE: usize = 200;

fn setup_game<const L: usize>(
    rng: &mut impl RngCore,
    snake: &mut Snake<L>,
    fruits: &mut Fruits,
    snake_direction: &mut Direction
) {
    *snake = Snake::new(4, 4, 2);
    *fruits = Fruits::new();
    *snake_direction = Direction::default().opposite();

    fruits.spawn_fruit_at_random_location(rng, &snake.as_layer());

    arduino_hal::delay_ms(1000);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut test_led = pins.d13.into_output();

    // Display pins

    let cs = pins.d11.into_output();
    let clk = pins.d10.into_output();
    let din = pins.d12.into_output();

    // Create adc

    let mut adc = Adc::new(dp.ADC, Default::default());

    // Create rng

    let mut rng = SmallRng::seed_from_u64(const_random!(u64));

    // Setup display

    let mut display = MAX7219::from_pins(1, din, cs, clk).unwrap();
    display.power_on().unwrap();
    display.set_intensity(0, 0x0f).unwrap();

    // Setup joystick

    let x_pos = pins.a1.into_analog_input(&mut adc);
    let y_pos = pins.a0.into_analog_input(&mut adc);

    // Create canvas

    let mut canvas: DisplayCanvas<2, 1> = DisplayCanvas::new();

    // Setup gameplay

    let mut fruits = Fruits::new();
    let mut snake: Snake<64> = Snake::new(0, 0, 0);
    let mut snake_direction = Direction::default();

    setup_game(&mut rng, &mut snake, &mut fruits, &mut snake_direction);

    let mut cycle: usize = 0;

    loop {
        let update_game = cycle % GAME_RUN_RATE == 0;

        // Get joystick position

        let joystick_x = joystick_input_from_raw(x_pos.analog_read(&mut adc), true);
        let joystick_y = joystick_input_from_raw(y_pos.analog_read(&mut adc), true);

        // Control snake direction

        if let Some(new_snake_direction) = Direction::from_joystick(joystick_x, joystick_y) {
            snake_direction = new_snake_direction;
        }

        // Move snake

        if update_game {
            match snake.next_position(&snake_direction) {
                Some(next_snake_position) => {
                    // Gameplay

                    if fruits.is_fruit(next_snake_position.0, next_snake_position.1) {
                        fruits.despawn_fruit(next_snake_position.0, next_snake_position.1);
                        fruits.spawn_fruit_at_random_location(&mut rng, &snake.as_layer());
                        snake.move_snake(&snake_direction, true);
                    } else {
                        snake.move_snake(&snake_direction, false);
                    }
                }
                None => setup_game(&mut rng, &mut snake, &mut fruits, &mut snake_direction),
            }
        }

        // Update display

        canvas.update_layer(0, fruits.as_layer());
        canvas.update_layer(1, snake.as_layer());
        canvas.write_to_display(0, &mut display);

        cycle += 1;
    }
}
