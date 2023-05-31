use plotters::prelude::*;
use plotters::coord::Shift;

fn draw_arrow<DB: DrawingBackend>(drawing_area: &DrawingArea<DB, Shift>) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
where
    DB: DrawingBackend,
{
    let arrow_line = PathElement::new(vec![(75, 75), (225, 75)], ShapeStyle::from(&BLACK).stroke_width(2));
    drawing_area.draw(&arrow_line)?;

    let arrow_head = PathElement::new(
        vec![(220, 70), (225, 75), (220, 80)],
        ShapeStyle::from(&BLACK).filled(),
    );
    drawing_area.draw(&arrow_head)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("diagram.png", (300, 200)).into_drawing_area();
    root.fill(&WHITE)?;

    // Draw two rectangles
    let rect1 = [(50, 50), (100, 100)];
    let rect2 = [(200, 50), (250, 100)];
    root.draw(&Rectangle::new(rect1, ShapeStyle::from(&BLACK).filled()))?;
    root.draw(&Rectangle::new(rect2, ShapeStyle::from(&BLACK).filled()))?;

    // Draw an arrow from the center of the first rectangle to the center of the second
    draw_arrow(&root)?;

    Ok(())
}