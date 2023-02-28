use plotters::prelude::*;
use std::error::Error;
use fast_float::parse;
fn readcsv() -> Result<(Vec<f64>,Vec<i32>), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("kc_house_data.csv")?;
    let mut price:Vec<f64> = Vec::new();
    let mut sqft_living:Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        match record.get(2) {
            //divide the number of price by 1000 to normalize the data
            Some(i) => {
                let tmp: f64 = parse(i).unwrap();
                price.push(tmp/1000.0)
            },
            _ => ()
        }
        match record.get(5) {
            Some(i) => {
                sqft_living.push(i.parse::<i32>().unwrap())
            },
            _ => ()
        }
    }
    
    return Ok((price, sqft_living));
}

fn main() {
    let data = match readcsv(){
        Ok(t) => t,
        _ => (Vec::new(),Vec::new())
    };
    let price = data.0;
    let sqft_living = data.1;
    //use zip to put two vector into single vector
    let price_and_sqft: Vec<(f64, i32)>= price.iter().cloned().zip(sqft_living.iter().cloned()).collect();
    //set picture area and path to store
    let pict = BitMapBackend::new("plot.png", (600, 400)).into_drawing_area();
    //set background color
    pict.fill(&WHITE).unwrap();
    //set label format
    let mut ctx = ChartBuilder::on(&pict)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("House price data analysis", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..8000.0, 0..10000)
        .unwrap();
    //configure area and draw
    ctx.configure_mesh().x_desc("sqft living").y_desc("House price").draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(
        price_and_sqft.iter().map(|point| Circle::new(*point, 4.0_f64, &RED)),
    ).unwrap();

}
// fn main() {
//     println!("Hello, world!");
// }