extern crate gerber;

use gerber::Layer;
use gerber::types::{FileAttribute, Part, FileFunction, ExtendedPosition, CopperType};
//use gerber::{Aperture, Circle, Rectangular};

fn main() {
    let mut layer = Layer::new();

    // Set attributes
    layer.set_file_attribute(FileAttribute::Part(Part::Single));
    layer.set_file_attribute(FileAttribute::FileFunction(FileFunction::Copper {
        layer: 0,
        pos: ExtendedPosition::Top,
        copper_type: Some(CopperType::Signal),
    }));

    //// Create apertures for traces
    //let trace_power = layer.create_aperture(Aperture::Circle(Circle::new(1.0)));
    //
    //// Create apertures for pads
    //let pad_round = layer.create_aperture(Aperture::Circle(Circle::with_hole(1.2, 0.8)));
    //let pad_square = layer.create_aperture(Aperture::Rectangle(Rectangular::with_hole(1.2, 1.2, 0.8)));
    //
    //// Flash pads
    //layer.flash((10.0, 10.0), pad_round);
    //layer.flash((20.0, 10.0), pad_round);
    //layer.flash((30.0, 10.0), pad_square);
    //
    //// Draw trace connecting all pads
    //layer.draw_line((10.0, 10.0), (30.0, 10.0), trace_power);

    println!("{:?}", layer);
}
