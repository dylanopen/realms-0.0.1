# Example #1 - Setting Up A Window

(doc in progress)

``` rust
use realms::event::Event;
use realms::window::Window
use realms::Colour;

fn main()
{
  let mut window = Window::new("My Amazing Game", 800, 450).unwrap();
  let fill_colour = Colour::from_rgb(0, 100, 255);
  let mut running = true;

  while running
  {
    window.fill(fill_colour.clone()).unwrap();
    for event in window.get_events()
    {
      match event
      {
        Event::Quit => {
          running = false;
        },
        _ => {  }
      }
    }
    window.draw();
  }
}
```
