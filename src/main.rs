use apng::{load_dynamic_image, Encoder, Frame, PNGImage};
use plotters::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

#[derive(Debug)]
pub struct Grid4D {
    pub delta_t: f64,
    pub delta_x: f64,
    pub num_grid: usize,
    pub x_1: Vec<Vec<Vec<Vec<f64>>>>,
    pub x_2: Vec<Vec<Vec<Vec<f64>>>>,
    pub tmp: Vec<Vec<Vec<Vec<f64>>>>,
}

impl Grid4D {
    pub fn new(dim: usize) -> Self {
        Grid4D {
            delta_t: (2.0 / (dim as f64 - 1.0)).powi(2),
            delta_x: 2.0 / (dim as f64 - 1.0),
            num_grid: dim,
            x_1: vec![vec![vec![vec![0.0; dim]; dim]; dim]; dim],
            x_2: vec![vec![vec![vec![0.0; dim]; dim]; dim]; dim],
            tmp: vec![vec![vec![vec![0.0; dim]; dim]; dim]; dim],
        }
    }

    pub fn initialize(&mut self) {
        // x in [-1, 1]
        for i in 0..self.num_grid {
            let x = -1.0 + 2.0 * i as f64 / (self.num_grid as f64 - 1.0);
            for j in 0..self.num_grid {
                let y = -1.0 + 2.0 * j as f64 / (self.num_grid as f64 - 1.0);
                for k in 0..self.num_grid {
                    let z = -1.0 + 2.0 * k as f64 / (self.num_grid as f64 - 1.0);
                    for l in 0..self.num_grid {
                        let w = -1.0 + 2.0 * l as f64 / (self.num_grid as f64 - 1.0);
                        self.x_1[i][j][k][l] = (-40.0 * (x * x + y * y + z * z + w * w)).exp();
                        self.x_2[i][j][k][l] = (-40.0 * (x * x + y * y + z * z + w * w)).exp();
                    }
                }
            }
        }
        for i in 0..self.num_grid {
            self.x_1[i][0][0][0] = 0.0;
            self.x_1[self.num_grid - 1][0][0][0] = 0.0;
            self.x_1[0][i][0][0] = 0.0;
            self.x_1[0][self.num_grid - 1][0][0] = 0.0;
            self.x_1[0][0][i][0] = 0.0;
            self.x_1[0][0][self.num_grid - 1][0] = 0.0;
            self.x_1[0][0][0][i] = 0.0;
            self.x_1[0][0][0][self.num_grid - 1] = 0.0;
        }
    }

    pub fn step(&mut self) {
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                for k in 1..(self.num_grid - 1) {
                    for l in 1..(self.num_grid - 1) {
                        self.tmp[i][j][k][l] = 2.0 * self.x_1[i][j][k][l] - self.x_2[i][j][k][l]
                            + self.delta_t * self.delta_t / (self.delta_x * self.delta_x)
                                * (self.x_1[i + 1][j][k][l] - 2.0 * self.x_1[i][j][k][l]
                                    + self.x_1[i - 1][j][k][l]
                                    + self.x_1[i][j + 1][k][l]
                                    - 2.0 * self.x_1[i][j][k][l]
                                    + self.x_1[i][j - 1][k][l]
                                    + self.x_1[i][j][k + 1][l]
                                    - 2.0 * self.x_1[i][j][k][l]
                                    + self.x_1[i][j][k - 1][l]
                                    + self.x_1[i][j][k][l + 1]
                                    - 2.0 * self.x_1[i][j][k][l]
                                    + self.x_1[i][j][k][l - 1]);
                    }
                }
            }
        }
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                for k in 1..(self.num_grid - 1) {
                    for l in 1..(self.num_grid - 1) {
                        self.x_2[i][j][k][l] = self.x_1[i][j][k][l];
                        self.x_1[i][j][k][l] = self.tmp[i][j][k][l];
                    }
                }
            }
        }
    }

    pub fn draw(&self, i: usize) {
        let out_file_name = format!("{:04}", i).to_string() + ".png";

        let root_area = BitMapBackend::new(&out_file_name, (2560, 1440)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let root_area = root_area
            .titled("4D, u_tt = u_xx + u_yy + u_zz + u_ww.", ("sans-serif", 100))
            .unwrap();

        let (upper, lower) = root_area.split_vertically(720);

        let drawing_areas = upper.split_evenly((1, 2));

        let mut cc0 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("x",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc0.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc0.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[i][self.num_grid / 2][self.num_grid / 2][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc1 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("y",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc1.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc1.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][i][self.num_grid / 2][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        let mut cc2 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("z"), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc2.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc2.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][self.num_grid / 2][i][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc3 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("w"), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc3.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc3.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][self.num_grid / 2][self.num_grid / 2][i]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();
    }

    pub fn gen_apng(&self, num: usize) {
        let mut files = vec![];

        for i in 0..num {
            files.push(format!("{:04}", i).to_string() + ".png");
        }

        let mut png_images: Vec<PNGImage> = Vec::new();

        for f in files.iter() {
            let mut file = File::open(f).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let img = image::load_from_memory(&buffer).unwrap();
            png_images.push(load_dynamic_image(img).unwrap());
        }

        let path = Path::new(r"wave_4d.png");
        let mut out = BufWriter::new(File::create(path).unwrap());

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut out, config).unwrap();

        for image in png_images.iter() {
            let frame = Frame {
                delay_num: Some(1),
                delay_den: Some(20),
                ..Default::default()
            };
            encoder.write_frame(image, frame).unwrap();
        }

        match encoder.finish_encode() {
            Ok(_n) => println!("success"),
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[derive(Debug)]
pub struct Grid3D {
    pub delta_t: f64,
    pub delta_x: f64,
    pub num_grid: usize,
    pub x_1: Vec<Vec<Vec<f64>>>,
    pub x_2: Vec<Vec<Vec<f64>>>,
    pub tmp: Vec<Vec<Vec<f64>>>,
}

impl Grid3D {
    pub fn new(dim: usize) -> Self {
        Grid3D {
            delta_t: (2.0 / (dim as f64 - 1.0)).powi(2),
            delta_x: 2.0 / (dim as f64 - 1.0),
            num_grid: dim,
            x_1: vec![vec![vec![0.0; dim]; dim]; dim],
            x_2: vec![vec![vec![0.0; dim]; dim]; dim],
            tmp: vec![vec![vec![0.0; dim]; dim]; dim],
        }
    }

    pub fn initialize(&mut self) {
        // x in [-1, 1]
        for i in 0..self.num_grid {
            let x = -1.0 + 2.0 * i as f64 / (self.num_grid as f64 - 1.0);
            for j in 0..self.num_grid {
                let y = -1.0 + 2.0 * j as f64 / (self.num_grid as f64 - 1.0);
                for k in 0..self.num_grid {
                    let z = -1.0 + 2.0 * k as f64 / (self.num_grid as f64 - 1.0);
                    self.x_1[i][j][k] = (-40.0 * (x * x + y * y + z * z)).exp();
                    self.x_2[i][j][k] = (-40.0 * (x * x + y * y + z * z)).exp();
                }
            }
        }
        for i in 0..self.num_grid {
            self.x_1[i][0][0] = 0.0;
            self.x_1[self.num_grid - 1][0][0] = 0.0;
            self.x_1[0][i][0] = 0.0;
            self.x_1[0][self.num_grid - 1][0] = 0.0;
            self.x_1[0][0][i] = 0.0;
            self.x_1[0][0][self.num_grid - 1] = 0.0;
        }
    }

    pub fn step(&mut self) {
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                for k in 1..(self.num_grid - 1) {
                    self.tmp[i][j][k] = 2.0 * self.x_1[i][j][k] - self.x_2[i][j][k]
                        + self.delta_t * self.delta_t / (self.delta_x * self.delta_x)
                            * (self.x_1[i + 1][j][k] - 2.0 * self.x_1[i][j][k]
                                + self.x_1[i - 1][j][k]
                                + self.x_1[i][j + 1][k]
                                - 2.0 * self.x_1[i][j][k]
                                + self.x_1[i][j - 1][k]
                                + self.x_1[i][j][k + 1]
                                - 2.0 * self.x_1[i][j][k]
                                + self.x_1[i][j][k - 1]);
                }
            }
        }
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                for k in 1..(self.num_grid - 1) {
                    self.x_2[i][j][k] = self.x_1[i][j][k];
                    self.x_1[i][j][k] = self.tmp[i][j][k];
                }
            }
        }
    }

    pub fn draw(&self, i: usize) {
        let out_file_name = format!("{:04}", i).to_string() + ".png";

        let root_area = BitMapBackend::new(&out_file_name, (2560, 1440)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let root_area = root_area
            .titled("3D, u_tt = u_xx + u_yy + u_zz.", ("sans-serif", 100))
            .unwrap();

        let (upper, lower) = root_area.split_vertically(720);

        let drawing_areas = upper.split_evenly((1, 2));

        let mut cc0 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("x",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc0.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc0.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[i][self.num_grid / 2][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc1 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("y",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc1.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc1.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][i][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        let mut cc2 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("z"), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc2.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc2.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][self.num_grid / 2][i]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc3 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!(""), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc3.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();
    }

    pub fn gen_apng(&self, num: usize) {
        let mut files = vec![];

        for i in 0..num {
            files.push(format!("{:04}", i).to_string() + ".png");
        }

        let mut png_images: Vec<PNGImage> = Vec::new();

        for f in files.iter() {
            let mut file = File::open(f).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let img = image::load_from_memory(&buffer).unwrap();
            png_images.push(load_dynamic_image(img).unwrap());
        }

        let path = Path::new(r"wave_3d.png");
        let mut out = BufWriter::new(File::create(path).unwrap());

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut out, config).unwrap();

        for image in png_images.iter() {
            let frame = Frame {
                delay_num: Some(1),
                delay_den: Some(20),
                ..Default::default()
            };
            encoder.write_frame(image, frame).unwrap();
        }

        match encoder.finish_encode() {
            Ok(_n) => println!("success"),
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[derive(Debug)]
pub struct Grid2D {
    pub delta_t: f64,
    pub delta_x: f64,
    pub num_grid: usize,
    pub x_1: Vec<Vec<f64>>,
    pub x_2: Vec<Vec<f64>>,
    pub tmp: Vec<Vec<f64>>,
}

impl Grid2D {
    pub fn new(dim: usize) -> Self {
        Grid2D {
            delta_t: (2.0 / (dim as f64 - 1.0)).powi(2),
            delta_x: 2.0 / (dim as f64 - 1.0),
            num_grid: dim,
            x_1: vec![vec![0.0; dim]; dim],
            x_2: vec![vec![0.0; dim]; dim],
            tmp: vec![vec![0.0; dim]; dim],
        }
    }

    pub fn initialize(&mut self) {
        // x in [-1, 1]
        for i in 0..self.num_grid {
            let x = -1.0 + 2.0 * i as f64 / (self.num_grid as f64 - 1.0);
            for j in 0..self.num_grid {
                let y = -1.0 + 2.0 * j as f64 / (self.num_grid as f64 - 1.0);
                self.x_1[i][j] = (-40.0 * (x * x + y * y)).exp();
                self.x_2[i][j] = (-40.0 * (x * x + y * y)).exp();
            }
        }
        for i in 0..self.num_grid {
            self.x_1[i][0] = 0.0;
            self.x_1[self.num_grid - 1][0] = 0.0;
            self.x_1[0][i] = 0.0;
            self.x_1[0][self.num_grid - 1] = 0.0;
        }
    }

    pub fn step(&mut self) {
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                self.tmp[i][j] = 2.0 * self.x_1[i][j] - self.x_2[i][j]
                    + self.delta_t * self.delta_t / (self.delta_x * self.delta_x)
                        * (self.x_1[i + 1][j] - 2.0 * self.x_1[i][j]
                            + self.x_1[i - 1][j]
                            + self.x_1[i][j + 1]
                            - 2.0 * self.x_1[i][j]
                            + self.x_1[i][j - 1]);
            }
        }
        for i in 1..(self.num_grid - 1) {
            for j in 1..(self.num_grid - 1) {
                self.x_2[i][j] = self.x_1[i][j];
                self.x_1[i][j] = self.tmp[i][j];
            }
        }
    }

    pub fn draw(&self, i: usize) {
        let out_file_name = format!("{:04}", i).to_string() + ".png";

        let root_area = BitMapBackend::new(&out_file_name, (2560, 1440)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let root_area = root_area
            .titled("2D, u_tt = u_xx + u_yy.", ("sans-serif", 100))
            .unwrap();

        let (upper, lower) = root_area.split_vertically(720);

        let drawing_areas = upper.split_evenly((1, 2));

        let mut cc0 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("x",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc0.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc0.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[i][self.num_grid / 2]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc1 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("y",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc1.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc1.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[self.num_grid / 2][i]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        let mut cc2 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!(""), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc2.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        let mut cc3 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!(""), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc3.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();
    }

    pub fn gen_apng(&self, num: usize) {
        let mut files = vec![];

        for i in 0..num {
            files.push(format!("{:04}", i).to_string() + ".png");
        }

        let mut png_images: Vec<PNGImage> = Vec::new();

        for f in files.iter() {
            let mut file = File::open(f).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let img = image::load_from_memory(&buffer).unwrap();
            png_images.push(load_dynamic_image(img).unwrap());
        }

        let path = Path::new(r"wave_2d.png");
        let mut out = BufWriter::new(File::create(path).unwrap());

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut out, config).unwrap();

        for image in png_images.iter() {
            let frame = Frame {
                delay_num: Some(1),
                delay_den: Some(20),
                ..Default::default()
            };
            encoder.write_frame(image, frame).unwrap();
        }

        match encoder.finish_encode() {
            Ok(_n) => println!("success"),
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[derive(Debug)]
pub struct Grid1D {
    pub delta_t: f64,
    pub delta_x: f64,
    pub num_grid: usize,
    pub x_1: Vec<f64>,
    pub x_2: Vec<f64>,
    pub tmp: Vec<f64>,
}

impl Grid1D {
    pub fn new(dim: usize) -> Self {
        Grid1D {
            delta_t: (2.0 / (dim as f64 - 1.0)).powi(2),
            delta_x: 2.0 / (dim as f64 - 1.0),
            num_grid: dim,
            x_1: vec![0.0; dim],
            x_2: vec![0.0; dim],
            tmp: vec![0.0; dim],
        }
    }

    pub fn initialize(&mut self) {
        // x in [-1, 1]
        for i in 0..self.num_grid {
            let x = -1.0 + 2.0 * i as f64 / (self.num_grid as f64 - 1.0);
            self.x_1[i] = (-40.0 * (x * x)).exp();
            self.x_2[i] = (-40.0 * (x * x)).exp();
        }
        self.x_1[0] = 0.0;
        self.x_1[self.num_grid - 1] = 0.0;
    }

    pub fn step(&mut self) {
        for i in 1..(self.num_grid - 1) {
            self.tmp[i] = 2.0 * self.x_1[i] - self.x_2[i]
                + self.delta_t * self.delta_t / (self.delta_x * self.delta_x)
                    * (self.x_1[i + 1] - 2.0 * self.x_1[i] + self.x_1[i - 1]);
        }
        for i in 1..(self.num_grid - 1) {
            self.x_2[i] = self.x_1[i];
            self.x_1[i] = self.tmp[i];
        }
    }

    pub fn draw(&self, i: usize) {
        let out_file_name = format!("{:04}", i).to_string() + ".png";

        let root_area = BitMapBackend::new(&out_file_name, (2560, 1440)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let root_area = root_area
            .titled("1D, u_tt = u_xx.", ("sans-serif", 100))
            .unwrap();

        let (upper, lower) = root_area.split_vertically(720);

        let drawing_areas = upper.split_evenly((1, 2));

        let mut cc0 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("x",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc0.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        cc0.draw_series(LineSeries::new(
            (0..self.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f32 / (self.num_grid as f32 - 1.0)) as f32,
                    (self.x_1[i]) as f32,
                )
            }),
            &BLUE,
        ))
        .unwrap();

        let mut cc1 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("",), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc1.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        let mut cc2 = ChartBuilder::on(&drawing_areas[0])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!(""), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc2.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        let mut cc3 = ChartBuilder::on(&drawing_areas[1])
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!(""), ("sans-serif", 60))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
            .unwrap();
        cc3.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();
    }

    pub fn gen_apng(&self, num: usize) {
        let mut files = vec![];

        for i in 0..num {
            files.push(format!("{:04}", i).to_string() + ".png");
        }

        let mut png_images: Vec<PNGImage> = Vec::new();

        for f in files.iter() {
            let mut file = File::open(f).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let img = image::load_from_memory(&buffer).unwrap();
            png_images.push(load_dynamic_image(img).unwrap());
        }

        let path = Path::new(r"wave_1d.png");
        let mut out = BufWriter::new(File::create(path).unwrap());

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut out, config).unwrap();

        for image in png_images.iter() {
            let frame = Frame {
                delay_num: Some(1),
                delay_den: Some(20),
                ..Default::default()
            };
            encoder.write_frame(image, frame).unwrap();
        }

        match encoder.finish_encode() {
            Ok(_n) => println!("success"),
            Err(err) => eprintln!("{}", err),
        }
    }
}

pub fn draw(vec1: &Grid1D, vec2: &Grid2D, vec3: &Grid3D, vec4: &Grid4D, i: usize) {
    let out_file_name = format!("{:04}", i).to_string() + ".png";

    let root = BitMapBackend::new(&out_file_name, (2560, 1440)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1.0..1.0, -1.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..vec1.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f64 / (vec3.num_grid as f64 - 1.0)),
                    vec1.x_1[i] as f64,
                )
            }),
            &BLACK,
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..vec2.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f64 / (vec3.num_grid as f64 - 1.0)),
                    vec2.x_1[i][vec2.x_1.len() / 2] as f64,
                )
            }),
            &RED,
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..vec3.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f64 / (vec3.num_grid as f64 - 1.0)),
                    vec3.x_1[i][vec3.x_1.len() / 2][vec3.x_1.len() / 2] as f64,
                )
            }),
            &GREEN,
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..vec4.x_1.len()).map(|i| {
                (
                    (-1.0 + 2.0 * i as f64 / (vec4.num_grid as f64 - 1.0)),
                    vec4.x_1[i][vec4.x_1.len() / 2][vec4.x_1.len() / 2][vec4.x_1.len() / 2] as f64,
                )
            }),
            &BLUE,
        ))
        .unwrap();

    root.present().unwrap();
}

pub fn gen_apng(num: usize) {
    let mut files = vec![];

    for i in 0..num {
        files.push(format!("{:04}", i).to_string() + ".png");
    }

    let mut png_images: Vec<PNGImage> = Vec::new();

    for f in files.iter() {
        let mut file = File::open(f).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let img = image::load_from_memory(&buffer).unwrap();
        png_images.push(load_dynamic_image(img).unwrap());
    }

    let path = Path::new(r"wave_1_2_3_4d.png");
    let mut out = BufWriter::new(File::create(path).unwrap());

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();

    for image in png_images.iter() {
        let frame = Frame {
            delay_num: Some(1),
            delay_den: Some(20),
            ..Default::default()
        };
        encoder.write_frame(image, frame).unwrap();
    }

    match encoder.finish_encode() {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
}
fn main() {
    let sim_num = 1200;
    let interval = 10;
    let size = 65;
    // 4D
    let mut vec_4d = Grid4D::new(size);
    vec_4d.initialize();
    for i in 0..sim_num {
        vec_4d.draw(i);
        for _ in 0..interval {
            vec_4d.step();
        }
    }
    vec_4d.gen_apng(sim_num);
    // 3D
    let mut vec_3d = Grid3D::new(size);
    vec_3d.initialize();
    for i in 0..sim_num {
        vec_3d.draw(i);
        for _ in 0..interval {
            vec_3d.step();
        }
    }
    vec_3d.gen_apng(sim_num);
    // 2D
    let mut vec_2d = Grid2D::new(size);
    vec_2d.initialize();
    for i in 0..sim_num {
        vec_2d.draw(i);
        for _ in 0..interval {
            vec_2d.step();
        }
    }
    vec_2d.gen_apng(sim_num);
    // 1D
    let mut vec_1d = Grid1D::new(size);
    vec_1d.initialize();
    for i in 0..sim_num {
        vec_1d.draw(i);
        for _ in 0..interval {
            vec_1d.step();
        }
    }
    vec_1d.gen_apng(sim_num);

    // 1,2,3,4D
    /*
    let mut vec_1d = Grid1D::new(size);
    let mut vec_2d = Grid2D::new(size);
    let mut vec_3d = Grid3D::new(size);
    let mut vec_4d = Grid4D::new(size);
    vec_1d.initialize();
    vec_2d.initialize();
    vec_3d.initialize();
    vec_4d.initialize();
    for i in 0..sim_num {
        draw(&vec_1d, &vec_2d, &vec_3d, &vec_4d, i);
        for _ in 0..interval {
            vec_1d.step();
            vec_2d.step();
            vec_3d.step();
            vec_4d.step();
        }
    }
    gen_apng(sim_num);
    */
}
