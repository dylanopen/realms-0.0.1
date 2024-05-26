# Example #2 - Shapes & Events

(doc in progress)

``` rust
use realms::event::Event;
use realms::shape::Rect;
use realms::window::Window;
use realms::Colour;

fn main()
{
  let mut window = Window::new("Shapes and events", 800, 600).unwrap();
  let mut running = true;

  let mut mouse_x = 400;
  let mut mouse_y = 600;
  
  while running
  {
    window.fill(Colour::from_rgb(255, 255, 255)).unwrap();
    for event in window.get_events()
    {
      match event
      {
        Event::Quit => {
          running = false;
        },
        Event::MouseMotion(event) => {
          mouse_x = event.x;
          mouse_y = event.y;
        },
        _ => {  }
      }
    }
    let rect = Rect::new(
      mouse_x-16, mouse_y-16,
      32, 32,
      Colour::from_rgb(255, 100, 0)
    );
    rect.draw(&mut window).unwrap();
    window.draw();
  }
}
```
