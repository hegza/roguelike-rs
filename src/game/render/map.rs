use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::scenes::GameScene;
use tui::buffer::Buffer;
use tui::style::Style;
use rpglib::{Dungeon, Room};

pub struct Map {}
impl GameView for Map {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, scene: &GameScene) {
        let dr = RoomDrawer {
            current: &scene.current_room(),
            dungeon: &scene.dungeon,
        };
        dr.render(t, area);
    }
}

struct RoomDrawer<'a> {
    current: &'a Room,
    dungeon: &'a Dungeon,
}
impl<'a> Widget for RoomDrawer<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        Paragraph::default()
            .block(Block::default().borders(border::ALL).title("World Map"))
            .draw(area, buf);

        // Render the room to the left-top of the map-buffer
        let initial = (1, 1);
        const SPACING: usize = 6;
        for (idx, ref room) in self.dungeon.rooms.iter().enumerate() {
            self.draw_room(
                room,
                (initial.0 + (idx * SPACING) as u16, initial.1),
                area,
                buf,
            );
        }
        // HACK: workaround, CBA to write the actual passage renderer right now
        for i in 0..self.dungeon.rooms.len() - 1 {
            buf.get_mut(area.x + (5 + i * SPACING) as u16, area.y + 2)
                .symbol = " ".to_owned();
            buf.get_mut(area.x + (6 + i * SPACING) as u16, area.y + 2)
                .symbol = "ʭ".to_owned();
            buf.get_mut(area.x + (7 + i * SPACING) as u16, area.y + 2)
                .symbol = " ".to_owned();
        }
    }
}
impl<'a> RoomDrawer<'a> {
    fn draw_room(
        &self,
        room: &Room,
        offset: (u16, u16),
        area: &Rect,
        buf: &mut Buffer,
    ) -> (u16, u16) {
        let mut room_buf = draw_room_5x3(room);

        let room_size = (room_buf.area().width, room_buf.area().height);
        // Move the room buffer to it's place
        room_buf.resize(Rect::new(area.x, area.y, room_size.0, room_size.1));

        for y in area.y..area.y + room_size.1 {
            for x in area.x..area.x + room_size.0 {
                buf.get_mut(x + offset.0, y + offset.1).symbol = room_buf.get(x, y).symbol.clone();
            }
        }

        (offset.0 + room_size.0, offset.1 + room_size.1)
    }
}

/// Render a 5x3 room
///
/// # Examples
/// ┌───┐
/// │ m │
/// └───┘
fn draw_room_5x3(room: &Room) -> Buffer {
    let monster_sym = match room.monster {
        Some(_) => "m",
        None => "_",
    };
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: 5,
        height: 3,
    });
    let style = Style::default();
    buf.set_string(0, 0, "┌───┐", &style);
    buf.set_string(0, 1, &format!("│ {} │", monster_sym), &style);
    buf.set_string(0, 2, "└───┘", &style);
    buf
}

/// Render a 1x1 corridor
/// # Examples
/// -
fn draw_corridor_1x1() -> Buffer {
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    });
    buf.get_mut(0, 0).symbol = "-".to_owned();
    buf
}
