use std::char::MAX;
use std::f32::MIN;

use pyo3::{pyfunction, PyResult};
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Text, SVG};
use svg::Document;

const STROKE_WIDTH: usize = 4;
const FONT_SIZE: usize = 10;


fn get_relative_x(value: f64, n_items: usize, interval_len: usize) -> usize {
    (((n_items as f64 - value + 1.0) / n_items as f64) * interval_len as f64) as usize
}

fn draw_ruler(mut document: SVG, n_items: usize) -> SVG {
    let attributes = document.get_attributes();
    let width = attributes.get("width").unwrap().parse::<usize>().unwrap();
    let height = attributes.get("height").unwrap().parse::<usize>().unwrap();
    
    let min_y = height / 8;
    let max_y = height / 4;

    let item_step = 2*FONT_SIZE;
    let high_bar = (max_y - min_y) / 4;
    let low_bar = (max_y - min_y) / 8;

    println!("bar: {} {}", high_bar, low_bar);

    let start_y = max_y + STROKE_WIDTH / 2;
    let start_x = width / 2 - (n_items * item_step); 
    let end_x = width / 2 + (n_items * item_step);

    let ruler = Data::new()
        .move_to((start_x, start_y))
        .line_to((end_x, start_y))
        .close();
    let ruler = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", STROKE_WIDTH)
        .set("d", ruler);

    document = document.add(ruler);

    for i in 0..(2*n_items) + 1 {
        let node;
        let line;
        let x = start_x + (i * item_step);
        if i % 2 == 0 {
            line = Data::new()
                .move_to((x, start_y + STROKE_WIDTH / 2))
                .line_to((x, start_y - high_bar))
                .close();
            // Draw numbers
            let number = n_items - (i / 2) + 1;
            let text = Text::new(format!("{}", number))
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1.0)
                .set("font-size", FONT_SIZE)
                .set("text-anchor", "middle")
                .set("x", x)
                .set("y", start_y - high_bar - FONT_SIZE);
            document = document.add(text);
        } else {
            // Draw line
            line = Data::new()
                .move_to((x, start_y + STROKE_WIDTH / 2))
                .line_to((x, start_y - low_bar))
                .close();
        }
        node = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", STROKE_WIDTH)
            .set("d", line);
        document = document.add(node);
    }
    document
}

// fn draw_ranking(mut document: SVG, labels: &[String], avg_ranks: &[f64]) -> SVG {
//     let attributes = document.get_attributes();
//     let width = attributes.get("width").unwrap().parse::<usize>().unwrap();

//     let n_items = labels.len();
//     let item_step = 2 * FONT_SIZE;

//     let start_y = (n_items/2) * FONT_SIZE;
//     let start_x = width / 2 - (n_items * item_step); 
//     let end_x = width / 2 + (n_items * item_step);

//     let half_count = labels.len() / 2;

//     for (i, (label, value)) in labels.iter().zip(avg_ranks).enumerate() {
//         let text;
//         let v_line;
//         let h_line;

//         let x = start_x + get_relative_x(*value, labels.len(), end_x - start_x);
//         // Draw line
//         if i < half_count {
//             let end_y =
//                 start_y + ((i * 2 * FONT_SIZE)) + FONT_SIZE + STROKE_WIDTH;
//             v_line = Data::new()
//                 .move_to((x, start_y))
//                 .line_to((x, end_y))
//                 .close();
//             h_line = Data::new()
//                 .move_to((x, end_y))
//                 .line_to((start_x, end_y))
//                 .close();
//             text = Text::new(format!("{}", label))
//                 .set("fill", "none")
//                 .set("stroke", "black")
//                 .set("stroke-width", 1.0)
//                 .set("font-size", FONT_SIZE)
//                 .set("dominant-baseline", "middle")
//                 .set("text-anchor", "end")
//                 .set("x", start_x)
//                 .set("y", end_y);
//         } else {
//             let end_y = start_y
//                 + (labels.len() - i) * 2 * FONT_SIZE
//                 + FONT_SIZE
//                 + STROKE_WIDTH;
//             v_line = Data::new()
//                 .move_to((x, start_y))
//                 .line_to((x, end_y))
//                 .close();
//             h_line = Data::new()
//                 .move_to((x, end_y))
//                 .line_to((end_x, end_y))
//                 .close();
//             text = Text::new(format!("{}", label))
//                 .set("fill", "none")
//                 .set("stroke", "black")
//                 .set("stroke-width", 1.0)
//                 .set("font-size", FONT_SIZE)
//                 .set("text-anchor", "start")
//                 .set("dominant-baseline", "middle")
//                 .set("x", end_x)
//                 .set("y", end_y);
//         }
//         let v_node = Path::new()
//             .set("fill", "none")
//             .set("stroke", "black")
//             .set("stroke-width", STROKE_WIDTH / 2)
//             .set("d", v_line);
//         document = document.add(v_node);
//         let h_node = Path::new()
//             .set("fill", "none")
//             .set("stroke", "black")
//             .set("stroke-width", STROKE_WIDTH / 2)
//             .set("d", h_line);
//         document = document.add(h_node);
//         document = document.add(text);
//     }
//     document
// }

// fn draw_clique(mut document: SVG, start_x: f64, start_y: f64, clique_len: f64) -> SVG {
//     let cd_line = Data::new()
//         .move_to((start_x, start_y))
//         .line_to((start_x + clique_len, start_y))
//         .close();
//     let cd_line = Path::new()
//         .set("fill", "none")
//         .set("stroke", "red")
//         .set("stroke-width", STROKE_WIDTH / 2.0)
//         .set("d", cd_line);

//     let l_rect = Rectangle::new()
//         .set("width", STROKE_WIDTH)
//         .set("height", STROKE_WIDTH)
//         .set("fill", "red")
//         .set("x", start_x - STROKE_WIDTH)
//         .set("y", start_y - STROKE_WIDTH / 2.0);

//     let r_rect = Rectangle::new()
//         .set("width", STROKE_WIDTH)
//         .set("height", STROKE_WIDTH)
//         .set("fill", "red")
//         .set("x", start_x + clique_len)
//         .set("y", start_y - STROKE_WIDTH / 2.0);

//     document = document.add(l_rect);
//     document = document.add(cd_line);
//     document = document.add(r_rect);

//     document
// }

// fn draw_cliques(mut document: SVG, cd: f64, avg_ranks: &[f64]) -> SVG {
//     let attributes = document.get_attributes();

//     // Get width and height
//     let width = attributes.get("width").unwrap().parse::<usize>().unwrap();
//     let height = attributes.get("height").unwrap().parse::<usize>().unwrap();

//     // Set start_x and start_y
//     let start_y = (START_Y_PERC - 0.15) * height as f64;
//     let start_x = 0.2 * width as f64;
//     let end_x = 0.8 * width as f64;
//     let cd_len = (end_x - start_x) * cd / avg_ranks.len() as f64;

//     document = draw_clique(
//         document,
//         start_x - cd_len / 2.0,
//         start_y + (0.01 * height as f64),
//         cd_len,
//     );
//     // Draw CD
//     let text = Text::new(format!("CD={:.2}", cd))
//         .set("fill", "none")
//         .set("stroke", "black")
//         .set("stroke-width", 1.0)
//         .set("font-size", FONT_SIZE)
//         .set("text-anchor", "middle")
//         .set("x", start_x)
//         .set("y", start_y as f64);

//     document = document.add(text);
//     let heigth_stride_perc = 1.0 / (avg_ranks.len() * 3) as f64;
//     let start_y = (START_Y_PERC+0.02) * height as f64;
//     let mut h = 0;
//     let mut last_x2 = -1.0;
//     for i in (0..avg_ranks.len()).rev() {
//         let mut count = 0;
//         for j in (0..i).rev() {
//             if (avg_ranks[i] - avg_ranks[j]).abs() < cd {
//                 count += 1;
//             } else {
//                 break;
//             }
//         }
//         if count > 0 {
//             let x1 =
//                 start_x + get_relative_x(avg_ranks[i], avg_ranks.len(), (end_x - start_x) as f64);
//             let x2 = start_x
//                 + get_relative_x(
//                     avg_ranks[i - count],
//                     avg_ranks.len(),
//                     (end_x - start_x) as f64,
//                 );
//             if x2 != last_x2 {
//                 last_x2 = x2;
//                 document = draw_clique(
//                     document,
//                     x2,
//                     start_y + heigth_stride_perc * (h * height) as f64,
//                     (x1 - x2).abs(),
//                 );
//                 h += 1;
//             }
//         }
//     }

//     document
// }

fn get_dimensions(labels: &[String]) -> (usize, usize) {
    let n_items = labels.len();
    let max_label_length = labels.iter().map(|s| s.len()).max().unwrap();

    let v_step = FONT_SIZE;
    let height = (2 * n_items) * v_step;

    let h_step = 2 * FONT_SIZE;
    let width = 2 * (n_items + max_label_length) * h_step;
    (width, height)
}


#[pyfunction]
#[pyo3(signature = (cd, avg_ranks, labels, title=None, out_file=None))]
pub fn cd_diagram(
    cd: f64,
    avg_ranks: Vec<f64>,
    labels: Vec<String>,
    title: Option<String>,
    out_file: Option<String>,
) -> PyResult<()> {
    
    let (width, height) = get_dimensions(&labels);
    println!("width: {}, height: {}", width, height);
    let mut document = Document::new()
        .set("height", height)
        .set("width", width)
        .set("style", "background-color:white");

    // Draw title
    let title = title.unwrap_or("".to_string());
    let text = Text::new(title)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1.0)
        .set("font-size", FONT_SIZE)
        .set("text-anchor", "middle")
        .set("x", width / 2)
        .set("y", height - FONT_SIZE);

    document = document.add(text);
    // Draw ruler
    document = draw_ruler(document, labels.len());
    // Draw models
    // document = draw_ranking(document, &labels, &avg_ranks);
    // // Draw cliques
    // document = draw_cliques(document, cd, &avg_ranks);

    svg::save(out_file.unwrap_or("image.svg".to_string()), &document).unwrap();
    Ok(())

}
