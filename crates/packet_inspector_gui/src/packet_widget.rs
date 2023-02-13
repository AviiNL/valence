// a wgui widget that displays a row in the packet inspector
// consistant of a up or down arrow, a packet type (hex value), and a packet name

use eframe::epaint::{PathShape, RectShape};
use egui::{
    Pos2, Rect, Response, Rgba, Rounding, Sense, Shape, Stroke, TextStyle, Ui, Vec2, Widget,
    WidgetText,
};

use crate::context::Packet;

#[derive(Clone)]
pub enum PacketDirection {
    ClientToServer,
    ServerToClient,
}

impl PacketDirection {
    fn get_shape(&self, outer_rect: &Rect) -> PathShape {
        let rect = Rect::from_min_size(
            Pos2 {
                x: outer_rect.left() + 6.0,
                y: outer_rect.top() + 8.0,
            },
            Vec2 { x: 8.0, y: 8.0 },
        );

        let color = match self {
            PacketDirection::ServerToClient => Rgba::from_rgb(255.0, 0.0, 0.0),
            PacketDirection::ClientToServer => Rgba::from_rgb(0.0, 255.0, 0.0),
        };

        let points = match self {
            PacketDirection::ServerToClient => vec![
                Pos2 {
                    x: rect.left() + (rect.width() / 2.0),
                    y: rect.top() + rect.height(),
                },
                Pos2 {
                    x: rect.left() + 0.0,
                    y: rect.top(),
                },
                Pos2 {
                    x: rect.left() + rect.width(),
                    y: rect.top(),
                },
            ],
            PacketDirection::ClientToServer => vec![
                Pos2 {
                    x: rect.left() + (rect.width() / 2.0),
                    y: rect.top() + 0.0,
                },
                Pos2 {
                    x: rect.left() + 0.0,
                    y: rect.top() + rect.height(),
                },
                Pos2 {
                    x: rect.left() + rect.width(),
                    y: rect.top() + rect.height(),
                },
            ],
        };

        let mut shape = PathShape::closed_line(points, Stroke::new(2.0, color));
        shape.fill = color.into();

        shape
    }
}

impl Widget for Packet {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_at_least(
            Vec2 {
                x: ui.available_width(),
                y: 24.0,
            },
            Sense::click(),
        );

        let fill = match self.selected {
            true => Rgba::from_rgba_premultiplied(0.0, 0.0, 0.0, 0.2),
            false => Rgba::from_rgba_premultiplied(0.0, 0.0, 0.0, 0.0),
        };

        if ui.is_rect_visible(rect) {
            ui.painter().add(Shape::Rect(RectShape {
                rect,
                rounding: Rounding::none(),
                fill: fill.into(),
                stroke: Stroke::new(1.0, Rgba::BLACK),
            }));

            let shape = self.direction.get_shape(&rect);
            ui.painter().add(Shape::Path(shape));

            let identifier: WidgetText = format!("0x{:X?}", self.packet_type).into();

            let identifier =
                identifier.into_galley(ui, Some(false), rect.width() - 20.0, TextStyle::Button);

            let label: WidgetText = self.packet_name.into();

            let label = label.into_galley(ui, Some(false), rect.width() - 60.0, TextStyle::Button);

            identifier.paint_with_fallback_color(
                ui.painter(),
                Pos2 {
                    x: rect.left() + 20.0,
                    y: rect.top() + 6.0,
                },
                ui.visuals().weak_text_color(),
            );

            let mut label_rect = rect.clone();
            label_rect.set_width(rect.width() - 5.0);

            label.paint_with_fallback_color(
                &ui.painter().with_clip_rect(label_rect),
                Pos2 {
                    x: rect.left() + 55.0,
                    y: rect.top() + 6.0,
                },
                ui.visuals().strong_text_color(),
            )
        }

        response

        // let mut response = ui
        //     .horizontal(|ui| {
        //         ui.add(self.direction);
        //         ui.label(format!("{:x?}", self.packet_type));
        //         ui.label(self.packet_name);
        //     })
        //     .response;

        // response.sense.click = true;

        // let mut new_response = Response::from(response);
        // new_response.sense = Sense {
        //     click: true,
        //     drag: false,
        //     focusable: false,
        // };

        // if let Some(pos) = new_response.hover_pos() {
        //     println!("{:?}", pos);
        // };

        // if new_response.clicked() {
        //     println!("Clicked");
        // }

        // new_response
    }
}

// impl PacketWidget {
//     pub fn new(raw: Vec<u8>) -> Self {}
// }

// impl PacketWidget {
//     pub fn new(
//         direction: PacketDirection,
//         packet_type: u8,
//         packet_name: Option<String>,
//         time: f64,
//     ) -> Self {
//         Self {
//             direction,
//             packet_type,
//             packet_name,
//             time,
//         }
//     }
// }

// impl Widget for PacketWidget {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let Self {
//             direction,
//             packet_type,
//             packet_name,
//             time,
//         } = self;

//         let mut label = String::new();

//         match direction {
//             PacketDirection::ClientToServer => label.push_str(" /\\ "),
//             PacketDirection::ServerToClient => label.push_str(" \\/ "),
//         }

//         write!(&mut label, "0x{:02x} ", packet_type).unwrap();

//         if let Some(packet_name) = packet_name {
//             label.push_str(&packet_name);
//         } else {
//             label.push_str("unknown");
//         }

//         ui.label(label)
//     }
// }
