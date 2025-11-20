use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力先（PNG, 800x500）
    let root = BitMapBackend::new("sine.png", (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    // 座標軸の設定：x=0..2π, y=-1.2..1.2
    let mut chart = ChartBuilder::on(&root)
        .caption("sin(x)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..(2.0 * std::f64::consts::PI), -1.2f64..1.2f64)?;

    chart.configure_mesh()
        .x_desc("x [rad]")
        .y_desc("sin(x)")
        .draw()?;

    // 折れ線を描画（サンプル点 1000 個）
    chart.draw_series(LineSeries::new(
        (0..1000).map(|i| {
            let x = i as f64 / 999.0 * (2.0 * std::f64::consts::PI);
            (x, x.sin())
        }),
        &BLUE, // 線色
    ))?
    .label("sin(x)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    root.present()?; // ファイルに反映
    println!("Saved to sine.png");
    Ok(())
}

