extern crate conv;
pub extern crate gerber_types as types;

use std::io::{BufWriter, Write};
use std::rc::Rc;

use conv::TryFrom;
use types::GerberCode;


#[derive(Debug)]
pub struct Layer {
    file_attributes: Vec<types::FileAttribute>,
    apertures: Vec<Rc<types::ApertureDefinition>>,
    coordinate_format: types::CoordinateFormat,
    commands: Vec<types::Command>,
}

impl Layer {
    pub fn new(coordinate_format: types::CoordinateFormat) -> Self {
        Layer {
            file_attributes: vec![],
            apertures: vec![],
            coordinate_format: coordinate_format,
            commands: vec![],
        }
    }

    pub fn set_file_attribute(&mut self, attribute: types::FileAttribute) {
        self.file_attributes.push(attribute);
    }

    /// Create a new aperture definition.
    pub fn create_aperture(&mut self, aperture: types::Aperture) -> Rc<types::ApertureDefinition> {
        let aperture_definition = Rc::new(types::ApertureDefinition {
            code: 10 + self.apertures.len() as i32,
            aperture: aperture,
        });
        self.apertures.push(aperture_definition.clone());
        aperture_definition
    }

    /// Flash the specified aperture at the specified coordinates.
    ///
    /// This function will panic if one of the coordinates is `NaN` or
    /// `Infinite`.
    pub fn flash(&mut self, coords: (f64, f64), aperture: &types::ApertureDefinition) {
        // TODO: Optimize this so that SelectAperture is not used multiple times.
        self.commands.push(
            types::FunctionCode::DCode(
                types::DCode::SelectAperture(aperture.code)
            ).into()
        );
        self.commands.push(
            types::FunctionCode::DCode(
                types::DCode::Operation(
                    types::Operation::Flash(
                        types::Coordinates::new(
                            types::CoordinateNumber::try_from(coords.0).unwrap(),
                            types::CoordinateNumber::try_from(coords.1).unwrap(),
                            self.coordinate_format
                        )
                    )
                )
            ).into(),
        );
    }

    /// Generate the Gerber code for this layer.
    ///
    /// Will panic if the gerber-types-rs library generates invalid UTF-8 data
    /// (which is always a bug in that library).
    pub fn to_code(&self) -> String {
        let mut writer = BufWriter::new(vec![]);

        // Write attributes
        // TODO: This is unelegant
        &self.file_attributes
            .iter()
            .map(|a| {
                types::Command::ExtendedCode(
                    types::ExtendedCode::FileAttribute(
                        a.to_owned()
                    )
                )
            })
            .collect::<Vec<_>>()
            .to_code(&mut writer)
            .unwrap();
        write!(writer, "\n").unwrap();

        // Write apertures
        &self.apertures
            .iter()
            .cloned()
            .map(|mut a| {
                types::Command::ExtendedCode(
                    types::ExtendedCode::ApertureDefinition(
                        Rc::make_mut(&mut a).to_owned()
                    )
                )
            })
            .collect::<Vec<_>>()
            .to_code(&mut writer)
            .unwrap();
        write!(writer, "\n").unwrap();

        // Write commands
        self.commands.to_code(&mut writer).unwrap();
        write!(writer, "\n").unwrap();

        // Write end of file
        types::Command::FunctionCode(
            types::FunctionCode::MCode(
                types::MCode::EndOfFile
            ),
        ).to_code(&mut writer).unwrap();
        write!(writer, "\n").unwrap();

        String::from_utf8(writer.into_inner().unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
