mod explicits;
use explicits::{
    Player,
    Monster,
};
use fsio::*;
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{error::Error, io};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Alignment, Constraint, Direction, Layout},
//     widgets::{Block, BorderType, Borders},
//     Frame, Terminal,
// };

// fn main() -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // create app and run it
//     let res = run_app(&mut terminal);

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{:?}", err)
//     }

//     Ok(())
// }

// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     loop {
//         terminal.draw(ui)?;

//         if let Event::Key(key) = event::read()? {
//             if let KeyCode::Char('q') = key.code {
//                 return Ok(());
//             }
//         }
//     }
// }

// fn ui<B: Backend>(f: &mut Frame<B>) {
//     // Wrapping block for a group
//     // Just draw the block and the group on the same area and build the group
//     // with at least a margin of 1
//     let size = f.size();

//     // Surrounding block
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title("")
//         .title_alignment(Alignment::Center)
//         .border_type(BorderType::Rounded);
//     f.render_widget(block, size);

//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(4)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(f.size());

//     // Bottom two inner blocks
//     let bottom_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(chunks[1]);

//     // Bottom left block with all default borders
//     let block = Block::default().title("Explicits").title_alignment(Alignment::Center).borders(Borders::ALL);
//     f.render_widget(block, bottom_chunks[0]);
// }
use std::io::{stdout, stdin, Write};
fn main(){




    //let mut player = Player::default();

    let mut player: Player = Player::load_config();
    player.power_iter();
    println!();
    println!("Default Heath: {}", player.health);
    player.heal();
    println!("New Heath: {}", player.health);
    println!();
    print!("Enter player name: ");
    
    stdout().flush().ok();
    stdin().read_line(&mut player.name).ok();
    println!();
    player.stats();

    println!();
    let mut monster = Monster::default();
    player.attack(&mut monster);
    monster.stats();
    println!();

    monster.attack(&mut player);
    player.stats();

    player.write_config();

}